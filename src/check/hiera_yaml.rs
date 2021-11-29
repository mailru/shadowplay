use anyhow::Result;
use structopt::StructOpt;

use crate::puppet_parser::{toplevel::Ast, toplevel::Toplevel};

#[derive(Default)]
pub struct State {
    pp_ast_cache:
        std::collections::HashMap<std::path::PathBuf, Option<crate::puppet_parser::toplevel::Ast>>,
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
    ) -> Result<Option<Ast>> {
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

        let ast = match Ast::parse(&pp_content) {
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

    fn check_class_argument(
        &self,
        repo_path: &std::path::Path,
        yaml_path: &std::path::Path,
        yaml_marker: &crate::yaml::Marker,
        puppet_module: &crate::puppet::module::Module,
        argument: &str,
        state: &mut State,
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
                if class.data.identifier.data != puppet_module.identifier() {
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

        let _class_argument = match class.data.get_argument(argument) {
            None => {
                println!(
                    "Hiera static error in {:?} at {}: reference to puppet class {:?} does not have argument {:?}",
                    yaml_path, yaml_marker, puppet_module.name(), argument
                );
                return 1;
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
    ) -> usize {
        let yaml = match crate::yaml::load_file(file_path) {
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
            crate::yaml::YamlElt::Hash(h) => h,
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
                crate::yaml::YamlElt::String(v) => v,
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
                        println!(
                            "Hiera static error in {:?}: puppet module {:?} does not exists at {}",
                            file_path, module_file, key.marker
                        );
                        errors += 1;
                        continue;
                    }
                    errors += self.check_class_argument(
                        repo_path,
                        file_path,
                        &key.marker,
                        &puppet_module,
                        class_argument,
                        state,
                    )
                }
                Ok(None) => (),
            }
        }

        errors
    }

    pub fn check(&self, repo_path: &std::path::Path) {
        let mut state = State::default();
        let mut errors = 0;
        for file_path in &self.paths {
            errors += self.check_file(repo_path, file_path, &mut state)
        }

        if errors > 0 {
            std::process::exit(1)
        }
    }
}
