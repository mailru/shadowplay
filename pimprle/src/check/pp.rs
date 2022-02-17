use crate::check::error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Check {
    paths: Vec<std::path::PathBuf>,
}

impl Check {
    pub fn check_file(
        &self,
        _repo_path: &std::path::Path,
        file_path: &std::path::Path,
    ) -> Vec<error::Error> {
        let pp = match std::fs::read_to_string(file_path) {
            Err(err) => {
                return vec![error::Error::of_file(
                    file_path,
                    error::Type::FileError,
                    &format!("Cannot load: {}", err),
                )];
            }
            Ok(v) => v,
        };

        let ast = match super::PuppetAst::parse(&pp) {
            Err(err) => {
                let err = match err {
                    nom::Err::Incomplete(_) => {
                        // nom::complete doesn't generate this state
                        unreachable!()
                    }
                    nom::Err::Error(v) => v,
                    nom::Err::Failure(v) => v,
                };
                return vec![error::Error::from((file_path, &err))];
            }
            Ok(v) => v,
        };

        let linter_storage = puppet_pp_lint::lint::Storage::new();
        let linter = puppet_pp_lint::lint::AstLinter;

        let mut errors = Vec::new();
        for statement in &ast.data {
            errors.append(&mut linter.check_statement(&linter_storage, statement));
        }

        errors
            .into_iter()
            .map(|err| error::Error::from((file_path, &err)))
            .collect()
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
