use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

impl Check {
    pub fn check_file(&self, repo_path: &std::path::Path, file_path: &std::path::Path) -> usize {
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

            if let Some(puppet_module) = crate::puppet::module::Module::of_hiera(hiera_key) {
                let module_file = repo_path.join("modules").join(puppet_module.file_path());
                if !module_file.exists() {
                    println!(
                        "Hiera static error in {:?}: puppet module {:?} does not exists at {}",
                        file_path, module_file, key.marker
                    );
                    errors += 1;
                }
            }
        }

        errors
    }

    pub fn check(&self, repo_path: &std::path::Path) {
        let mut errors = 0;
        for file_path in &self.paths {
            errors += self.check_file(repo_path, file_path)
        }

        if errors > 0 {
            std::process::exit(1)
        }
    }
}
