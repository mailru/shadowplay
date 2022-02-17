use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

impl Check {
    pub fn check_file(&self, _repo_path: &std::path::Path, file_path: &std::path::Path) -> usize {
        let pp = match std::fs::read_to_string(file_path) {
            Err(err) => {
                println!("Failed to read {:?}: {}", file_path, err);
                return 1;
            }
            Ok(v) => v,
        };

        let ast = match super::PuppetAst::parse(&pp) {
            Err(err) => {
                println!("Parse error in {:?}: {}", file_path, err);
                return 1;
            }
            Ok(v) => v,
        };

        let linter_storage = puppet_pp_lint::lint::Storage::new();
        let linter = puppet_pp_lint::lint::AstLinter;

        let mut errors = Vec::new();
        for statement in &ast.data {
            errors.append(&mut linter.check_statement(&linter_storage, statement));
        }

        for error in &errors {
            let url_message = match &error.url {
                None => "".to_owned(),
                Some(url) => format!(" // See {}", url),
            };
            println!(
                "Puppet static error [{}] in {:?} at line {} column {}: {}{}",
                error.linter.name(),
                file_path,
                error.location.line(),
                error.location.column(),
                error.message,
                url_message
            );
        }

        errors.len()
    }

    pub fn check(&self, repo_path: &std::path::Path, _config: &crate::config::Config) {
        let mut errors = 0;
        for file_path in &self.paths {
            errors += self.check_file(repo_path, file_path)
        }

        if errors > 0 {
            std::process::exit(1)
        }
    }
}
