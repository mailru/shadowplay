use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

pub fn static_check(file_path: &std::path::Path, yaml: &crate::yaml::YamlLoader) -> usize {
    let mut errors = 0;

    match file_path.metadata() {
        Err(_) => {
            println!(
                "Yaml static error for {:?}: cannot get file metadata",
                file_path
            );
            errors += 1;
        }
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                println!("Yaml static error for {:?}: file is executable", file_path);
                errors += 1;
            }
        }
    }

    if yaml.docs.is_empty() {
        println!("Yaml static error in {:?}: Empty (untyped) yaml. Add '{{}}' or '[]' if this is expected.", file_path);
        return errors + 1;
    }

    if yaml.docs.len() > 1 {
        println!(
            "Yaml static error in {:?}: contains multiple documents",
            file_path
        );
        errors += 1;
    }

    if !yaml.errors.is_empty() {
        for err in &yaml.errors {
            println!("Yaml static error in {:?}: {}", file_path, err)
        }
        errors += 1;
    }

    errors
}

impl Check {
    pub fn check_file(&self, _repo_path: &std::path::Path, file_path: &std::path::Path) -> usize {
        let yaml = match crate::yaml::load_file(file_path) {
            Err(err) => {
                println!("Failed to read {:?}: {}", file_path, err);
                return 1;
            }
            Ok(v) => v,
        };

        static_check(file_path, &yaml)
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
