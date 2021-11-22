use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

impl Check {
    pub fn check_file(&self, repo_path: &std::path::Path, file_path: &std::path::Path) {
        let yaml = match crate::yaml::load_file(file_path) {
            Err(err) => {
                println!("Failed to read {:?}: {}", file_path, err);
                return;
            }
            Ok(v) => v,
        };

        let _ = crate::check::yaml::static_check(file_path, &yaml);

        let doc = match yaml.docs.as_slice() {
            [doc] => doc,
            _ => return,
        };

        let doc = match &doc.yaml {
            crate::yaml::YamlElt::Hash(h) => h,
            _ => {
                println!(
                    "Hiera static error in {:?}: Root element is not a map",
                    file_path
                );
                return;
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
                    continue;
                }
            };

            if let Some(puppet_module) = crate::puppet::module::Module::of_hiera(&hiera_key) {
                let module_file = repo_path.join("modules").join(puppet_module.file_path());
                if !module_file.exists() {
                    println!(
                        "Hiera static error in {:?}: puppet module {:?} does not exists at {}",
                        file_path, module_file, key.marker
                    );
                }
            }
        }
    }

    pub fn check(&self, repo_path: &std::path::Path) {
        for file_path in &self.paths {
            self.check_file(repo_path, file_path)
        }
    }
}
