use anyhow::Result;
use structopt::StructOpt;

use puppet_lang::toplevel::Toplevel;

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
    ) -> usize {
        let module_file = puppet_module.full_file_path(repo_path);
        let ast = match self.parse_pp(repo_path, &module_file, state) {
            Err(err) => {
                println!(
                    "Hiera static error in {:?} at {}: reference to puppet class {:?} which failed to parse with error: {:?}",
                    yaml_path, yaml_marker, puppet_module.name(), err
                );
                return 1;
            }
            Ok(None) => {
                println!(
                    "Hiera static error in {:?} at {}: reference to puppet class {:?} which failed to parse earlier",
                    yaml_path, yaml_marker, puppet_module.name()
                );
                return 1;
            }
            Ok(Some(v)) => v,
        };

        let class = match ast.data {
            Toplevel::Class(class) => {
                if class.identifier.name != puppet_module.identifier() {
                    println!(
                    "Hiera static error in {:?} at {}: reference to puppet file {:?} which toplevel class does not match module name",
                    yaml_path, yaml_marker, puppet_module.file_path()
                );
                    return 1;
                }
                class
            }
            _ => {
                println!(
                    "Hiera static error in {:?} at {}: reference to puppet file {:?} which toplevel expression is not a class",
                    yaml_path, yaml_marker, puppet_module.file_path()
                );
                return 1;
            }
        };

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
                    println!(
                    "Hiera static error in {:?} at {}: reference to puppet class {:?} does not have argument {:?}",
                    yaml_path, yaml_marker, puppet_module.name(), argument
                );
                    return 1;
                }
            }
            Some(_) => (),
        };

        0
    }

    pub fn check_file(
        &self,
        repo_path: &std::path::Path,
        file_path: &std::path::Path,
        state: &mut State,
        config: &crate::config::Config,
    ) -> usize {
        let yaml_str = match std::fs::read_to_string(file_path) {
            Ok(v) => v,
            Err(err) => {
                println!("Failed to load file {:?}: {}", file_path, err);
                return 1;
            }
        };

        let yaml = match located_yaml::YamlLoader::load_from_str(&yaml_str) {
            Err(err) => {
                println!("Failed to read {:?}: {}", file_path, err);
                return 1;
            }
            Ok(v) => v,
        };

        let mut errors = 0;

        errors += crate::check::yaml::static_check(file_path, &yaml);

        let doc = match yaml.docs.as_slice() {
            [doc] => doc,
            _ => return errors,
        };

        let doc = match &doc.yaml {
            located_yaml::YamlElt::Hash(h) => h,
            _ => {
                println!(
                    "Hiera static error in {:?}: Root element is not a map",
                    file_path
                );
                return errors + 1;
            }
        };

        for key in doc.keys() {
            let hiera_key = match &key.yaml {
                located_yaml::YamlElt::String(v) => v,
                v => {
                    println!(
                        "Hiera static error in {:?}: Invalid key type {:?} at {}",
                        file_path,
                        v.type_name(),
                        key.marker
                    );
                    errors += 1;
                    continue;
                }
            };

            lazy_static! {
                // Строка начинается с одиночного ":", содержит его в середине, или заканчивается одиночным ":"
                static ref SINGLE_SEMICOLON_RE: regex::Regex = regex::Regex::new("(?:^:[^:]|[^:]:[^:]|[^:]:$)").unwrap();
            }

            if SINGLE_SEMICOLON_RE.is_match(hiera_key) {
                println!(
                    "Hiera static error in {:?}: key {:?} contains single semicolon at {}",
                    file_path, hiera_key, key.marker
                );
                errors += 1;
            }

            match crate::puppet::module::Module::of_hiera(hiera_key) {
                Err(err) => {
                    println!(
                        "Hiera static error in {:?}: {} at {}",
                        file_path, err, key.marker
                    );
                    errors += 1;
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
                            println!(
                                "Hiera static error in {:?}: puppet module {:?} does not exists at {}",
                                file_path, module_file, key.marker
                            );
                            errors += 1;
                        }
                        continue;
                    }
                    errors += self.check_class_argument(
                        repo_path,
                        file_path,
                        &key.marker,
                        &puppet_module,
                        class_argument,
                        state,
                        config,
                    )
                }
                Ok(None) => (),
            }
        }

        errors
    }

    pub fn check(&self, repo_path: &std::path::Path, config: &crate::config::Config) {
        let mut state = State::default();
        let mut errors = 0;
        for file_path in &self.paths {
            errors += self.check_file(repo_path, file_path, &mut state, config)
        }

        if errors > 0 {
            std::process::exit(1)
        }
    }
}
