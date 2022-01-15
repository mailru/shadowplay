use crate::check::pp_static::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

pub struct ArgumentLooksSensitive;

impl LintPass for ArgumentLooksSensitive {
    fn name(&self) -> &str {
        "argument_looks_sensitive"
    }
}

impl EarlyLintPass for ArgumentLooksSensitive {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<puppet_parser::parser::Location>,
    ) -> Vec<super::lint::LintError> {
        let lc_name = arg.name.to_lowercase();
        if lc_name.contains("passw") || lc_name.ends_with("secret") || lc_name.ends_with("token") {
            match &arg.type_spec {
                None => vec![LintError::new(
                    self.name(),
                    "Assuming argument contains a secret value, it is not typed with 'Sensitive'",
                    &arg.extra,
                )],
                Some(t)
                    if !matches!(
                        t.data,
                        puppet_lang::typing::TypeSpecificationVariant::Sensitive(_)
                    ) =>
                {
                    vec![LintError::new(
                    self.name(),
                    "Assume argument contains a secret value, it is not typed with 'Sensitive' type",
                    &arg.extra,
                )]
                }
                Some(_) => vec![],
            }
        } else {
            vec![]
        }
    }
}
