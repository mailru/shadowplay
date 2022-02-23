use puppet_parser::Location;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct ArgumentLooksSensitive;

impl LintPass for ArgumentLooksSensitive {
    fn name(&self) -> &str {
        "argument_looks_sensitive"
    }
}

impl EarlyLintPass for ArgumentLooksSensitive {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Location>,
    ) -> Vec<super::lint::LintError> {
        let lc_name = arg.name.to_lowercase();
        if lc_name.contains("passw") || lc_name.ends_with("secret") || lc_name.ends_with("token") {
            match &arg.type_spec {
                None => vec![LintError::new(
                    Box::new(self.clone()),
                    &format!("Assuming argument {:?} contains a secret value, it is not typed with 'Sensitive'", arg.name),
                    &arg.extra,
                )],
                Some(t)
                    if !matches!(
                        t.data,
                        puppet_lang::typing::TypeSpecificationVariant::Sensitive(_)
                    ) =>
                {
                    vec![LintError::new(
                        Box::new(self.clone()),
                        &format!("Assuming argument {:?} contains a secret value, it is not typed with 'Sensitive' type", arg.name),
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

#[derive(Clone)]
pub struct SensitiveArgumentWithDefault;

impl LintPass for SensitiveArgumentWithDefault {
    fn name(&self) -> &str {
        "sensitive_argument_with_default"
    }
}

impl EarlyLintPass for SensitiveArgumentWithDefault {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Location>,
    ) -> Vec<super::lint::LintError> {
        if let Some(t) = &arg.type_spec {
            if matches!(
                t.data,
                puppet_lang::typing::TypeSpecificationVariant::Sensitive(_)
            ) && arg.default.is_some()
            {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Sensitive argument with default value",
                    &arg.extra,
                )];
            }
        }
        vec![]
    }
}

#[derive(Clone)]
pub struct ArgumentTyped;

impl LintPass for ArgumentTyped {
    fn name(&self) -> &str {
        "argument_typed"
    }
}

impl EarlyLintPass for ArgumentTyped {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Location>,
    ) -> Vec<super::lint::LintError> {
        if arg.type_spec.is_none() {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Argument is not typed",
                &arg.extra,
            )];
        }
        vec![]
    }
}

#[derive(Clone)]
pub struct ReadableArgumentsName;

impl LintPass for ReadableArgumentsName {
    fn name(&self) -> &str {
        "readable_argument_name"
    }
}

impl EarlyLintPass for ReadableArgumentsName {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Location>,
    ) -> Vec<super::lint::LintError> {
        if arg.name.len() < 2 {
            return vec![LintError::new(
                Box::new(self.clone()),
                &format!("Argument '{}' name is too short", arg.name),
                &arg.extra,
            )];
        }
        vec![]
    }
}

#[derive(Clone)]
pub struct LowerCaseArgumentName;

impl LintPass for LowerCaseArgumentName {
    fn name(&self) -> &str {
        "lower_case_argument_name"
    }
}

impl EarlyLintPass for LowerCaseArgumentName {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Location>,
    ) -> Vec<super::lint::LintError> {
        if arg.name.chars().any(|c| c.is_uppercase()) {
            return vec![LintError::new_with_url(
                Box::new(self.clone()),
                "Argument name with upper case letters.",
                "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_variables-variable-format",
                &arg.extra,
            )];
        }
        vec![]
    }
}
