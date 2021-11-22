use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

pub fn static_check(file_path: &std::path::Path, yaml: &crate::yaml::YamlLoader) -> bool {
    if yaml.docs.is_empty() {
        println!("Yaml static error in {:?}: Empty (untyped) yaml. Add '{{}}' or '[]' if this is expected.", file_path);
        return true;
    }

    if yaml.docs.len() > 1 {
        println!(
            "Yaml static error in {:?}: contains multiple documents",
            file_path
        );
        return true;
    }

    if !yaml.errors.is_empty() {
        for err in &yaml.errors {
            println!("Yaml static error in {:?}: {}", file_path, err)
        }
        return true;
    }

    false
}

impl Check {
    pub fn check_file(&self, _repo_path: &std::path::Path, file_path: &std::path::Path) {
        let yaml = match crate::yaml::load_file(file_path) {
            Err(err) => {
                println!("Failed to read {:?}: {}", file_path, err);
                return;
            }
            Ok(v) => v,
        };

        let _ = static_check(file_path, &yaml);
    }

    pub fn check(&self, repo_path: &std::path::Path) {
        for file_path in &self.paths {
            self.check_file(repo_path, file_path)
        }
    }
}
