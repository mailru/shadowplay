use crate::check::error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

pub fn static_check(
    file_path: &std::path::Path,
    yaml: &located_yaml::YamlLoader,
) -> Vec<error::Error> {
    let mut errors = Vec::new();

    match file_path.metadata() {
        Err(_) => {
            errors.push(error::Error::of_file(
                file_path,
                error::Type::FileError,
                "Cannot read file metadata",
            ));
        }
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 != 0 {
                errors.push(error::Error::of_file(
                    file_path,
                    error::Type::FileError,
                    "File is executable",
                ));
            }
        }
    }

    if yaml.docs.is_empty() {
        errors.push(error::Error::of_file(
            file_path,
            error::Type::Yaml,
            "Empty (untyped) yaml. Add '{{}}' or '[]' if this is expected.",
        ));
        return errors;
    }

    if yaml.docs.len() > 1 {
        errors.push(error::Error::of_file(
            file_path,
            error::Type::Yaml,
            "Empty (untyped) yaml. Add '{{}}' or '[]' if this is expected.",
        ));
    }

    errors.extend(
        &mut yaml
            .errors
            .iter()
            .map(|e| error::Error::from((file_path, e))),
    );

    errors
}

impl Check {
    pub fn check_file(
        &self,
        _repo_path: &std::path::Path,
        file_path: &std::path::Path,
    ) -> Vec<error::Error> {
        let yaml_str = match std::fs::read_to_string(file_path) {
            Ok(v) => v,
            Err(err) => {
                return vec![error::Error::of_file(
                    file_path,
                    error::Type::Yaml,
                    &format!("Failed to load file: {}", err),
                )]
            }
        };

        let yaml = match located_yaml::YamlLoader::load_from_str(&yaml_str) {
            Err(err) => {
                return vec![error::Error::of_file(
                    file_path,
                    error::Type::Yaml,
                    &format!("Failed to read file: {}", err),
                )]
            }
            Ok(v) => v,
        };

        static_check(file_path, &yaml)
    }

    pub fn check(
        &self,
        repo_path: &std::path::Path,
        _config: &crate::config::Config,
        format: &error::OutputFormat,
    ) -> crate::check::Summary {
        let mut errors = 0;
        for file_path in &self.paths {
            let file_errors = self.check_file(repo_path, file_path);
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
