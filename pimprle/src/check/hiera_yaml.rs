use crate::check::error;
use anyhow::Result;
use structopt::StructOpt;

use puppet_lang::toplevel::ToplevelVariant;

#[derive(Default)]
pub struct State {
    pp_ast_cache: std::collections::HashMap<std::path::PathBuf, Option<super::PuppetAst>>,
}

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

impl Check {
    fn parse_pp(
        &self,
        repo_path: &std::path::Path,
        file_path: &std::path::Path,
        state: &mut State,
    ) -> Result<Option<super::PuppetAst>> {
        if let Some(parsed) = state.pp_ast_cache.get(file_path) {
            return Ok((*parsed).clone());
        }

        let pp_content = match std::fs::read_to_string(repo_path.join(file_path)) {
            Ok(v) => v,
            Err(err) => {
                let _ = state.pp_ast_cache.insert(file_path.to_path_buf(), None);
                return Err(anyhow::Error::from(err));
            }
        };

        let ast = match super::PuppetAst::parse(&pp_content) {
            Ok(v) => v,
            Err(err) => {
                let _ = state.pp_ast_cache.insert(file_path.to_path_buf(), None);
                return Err(anyhow::format_err!("{:?}", err));
            }
        };

        let _ = state
            .pp_ast_cache
            .insert(file_path.to_path_buf(), Some(ast.clone()));

        Ok(Some(ast))
    }

    #[allow(clippy::too_many_arguments)]
    fn check_class_argument(
        &self,
        repo_path: &std::path::Path,
        yaml_path: &std::path::Path,
        yaml_marker: &located_yaml::Marker,
        puppet_module: &crate::puppet::module::Module,
        argument: &str,
        state: &mut State,
        config: &crate::config::Config,
    ) -> Vec<error::Error> {
        let module_file = puppet_module.full_file_path(repo_path);
        let ast = match self.parse_pp(repo_path, &module_file, state) {
            Err(err) => {
                return vec![error::Error::from((
                    yaml_path,
                    error::Type::Hiera,
                    format!(
                        "Reference to puppet class {:?} which failed to parse with error: {:?}",
                        puppet_module.name(),
                        err
                    )
                    .as_str(),
                    yaml_marker,
                ))];
            }
            Ok(None) => {
                return vec![error::Error::from((
                    yaml_path,
                    error::Type::Hiera,
                    format!(
                        "Reference to puppet class {:?} which failed to parse earlier",
                        puppet_module.name(),
                    )
                    .as_str(),
                    yaml_marker,
                ))];
            }
            Ok(Some(v)) => v,
        };

        let mut class = None;

        if let Some(elt) = ast.data.value.into_iter().next() {
            match elt.value {
                puppet_lang::statement::StatementVariant::Toplevel(
                    puppet_lang::toplevel::Toplevel {
                        data: ToplevelVariant::Class(v),
                        ..
                    },
                ) => {
                    if v.identifier.name != puppet_module.identifier() {
                        return vec![error::Error::from((
                            yaml_path,
                            error::Type::Hiera,
                            format!(
                                "Reference to puppet file {:?} which toplevel class does not match module name",
                                puppet_module.file_path(),
                            )
                            .as_str(),
                            yaml_marker,
                        ))];
                    }
                    class = Some(v)
                }
                _ => {
                    return vec![error::Error::from((
                        yaml_path,
                        error::Type::Hiera,
                        format!(
                            "Reference to puppet file {:?} which toplevel expression is not a class",
                            puppet_module.file_path(),
                        )
                        .as_str(),
                        yaml_marker,
                    ))];
                }
            }
        }

        let class = class.unwrap();

        let _class_argument = match class.get_argument(argument) {
            None => {
                if config
                    .checks
                    .hiera_yaml
                    .forced_values_exists
                    .contains(&format!("{}::{}", puppet_module.name(), argument))
                {
                    // OK, value is whitelisted
                } else {
                    return vec![error::Error::from((
                        yaml_path,
                        error::Type::Hiera,
                        format!(
                            "Reference to puppet class {:?} which does not have argument {:?}",
                            puppet_module.name(),
                            argument,
                        )
                        .as_str(),
                        yaml_marker,
                    ))];
                }
            }
            Some(_) => (),
        };

        Vec::new()
    }

    pub fn check_file(
        &self,
        repo_path: &std::path::Path,
        file_path: &std::path::Path,
        state: &mut State,
        config: &crate::config::Config,
    ) -> Vec<error::Error> {
        let mut errors = Vec::new();

        let yaml_str = match std::fs::read_to_string(file_path) {
            Ok(v) => v,
            Err(err) => {
                return vec![error::Error::of_file(
                    file_path,
                    error::Type::FileError,
                    &format!("Cannot load: {}", err),
                )];
            }
        };

        let yaml = match located_yaml::YamlLoader::load_from_str(&yaml_str) {
            Err(err) => {
                return vec![error::Error::from((file_path, &err))];
            }
            Ok(v) => v,
        };

        errors.extend(crate::check::yaml::static_check(file_path, &yaml));

        let doc = match yaml.docs.as_slice() {
            [doc] => doc,
            _ => return errors,
        };

        let doc = match &doc.yaml {
            located_yaml::YamlElt::Hash(h) => h,
            _ => {
                errors.push(error::Error::of_file(
                    file_path,
                    error::Type::Hiera,
                    "Root element is not a map",
                ));

                return errors;
            }
        };

        for key in doc.keys() {
            let hiera_key = match &key.yaml {
                located_yaml::YamlElt::String(v) => v,
                v => {
                    errors.push(error::Error::from((
                        file_path,
                        error::Type::Hiera,
                        format!("Invalid key type {:?}", v.type_name()).as_str(),
                        &key.marker,
                    )));
                    continue;
                }
            };

            lazy_static! {
                // Строка начинается с одиночного ":", содержит его в середине, или заканчивается одиночным ":"
                static ref SINGLE_SEMICOLON_RE: regex::Regex = regex::Regex::new("(?:^:[^:]|[^:]:[^:]|[^:]:$)").unwrap();
            }

            if SINGLE_SEMICOLON_RE.is_match(hiera_key) {
                errors.push(error::Error::from((
                    file_path,
                    error::Type::Hiera,
                    format!("Key {:?} contains single semicolon", hiera_key).as_str(),
                    &key.marker,
                )));
            }

            match crate::puppet::module::Module::of_hiera(hiera_key) {
                Err(err) => {
                    errors.push(error::Error::from((
                        file_path,
                        error::Type::Hiera,
                        err.to_string().as_str(),
                        &key.marker,
                    )));
                }
                Ok(Some((puppet_module, class_argument))) => {
                    let module_file = puppet_module.full_file_path(repo_path);
                    if !module_file.exists() {
                        if config
                            .checks
                            .hiera_yaml
                            .forced_modules_exists
                            .contains(&puppet_module.name())
                        {
                            // whitelisted module
                        } else {
                            errors.push(error::Error::from((
                                file_path,
                                error::Type::Hiera,
                                format!("Puppet module {:?} does not exist", module_file).as_str(),
                                &key.marker,
                            )));
                        }
                        continue;
                    }
                    errors.extend(self.check_class_argument(
                        repo_path,
                        file_path,
                        &key.marker,
                        &puppet_module,
                        class_argument,
                        state,
                        config,
                    ))
                }
                Ok(None) => (),
            }
        }

        errors
    }

    pub fn check(
        &self,
        repo_path: &std::path::Path,
        config: &crate::config::Config,
        format: &error::OutputFormat,
    ) -> crate::check::Summary {
        let mut state = State::default();
        let mut errors = 0;
        for file_path in &self.paths {
            let file_errors = self.check_file(repo_path, file_path, &mut state, config);
            for err in &file_errors {
                println!("{}", err.output(format))
            }
            errors += file_errors.len();
        }

        crate::check::Summary {
            errors_count: errors,
            files_checked: self.paths.len(),
        }
    }
}
