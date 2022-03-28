use puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct ArgumentLooksSensitive {
    #[serde(with = "serde_regex")]
    regex: regex::Regex,
}

impl Default for ArgumentLooksSensitive {
    fn default() -> Self {
        let regex = regex::Regex::new("(:?passw|secret$|token$)").unwrap();
        Self { regex }
    }
}

impl LintPass for ArgumentLooksSensitive {
    fn name(&self) -> &str {
        "ArgumentLooksSensitive"
    }
}

impl EarlyLintPass for ArgumentLooksSensitive {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Range>,
    ) -> Vec<super::lint::LintError> {
        let lc_name = arg.name.to_lowercase();
        if self.regex.is_match(&lc_name) {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct SensitiveArgumentWithDefault;

impl LintPass for SensitiveArgumentWithDefault {
    fn name(&self) -> &str {
        "SensitiveArgumentWithDefault"
    }
}

impl EarlyLintPass for SensitiveArgumentWithDefault {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Range>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ArgumentTyped;

impl LintPass for ArgumentTyped {
    fn name(&self) -> &str {
        "ArgumentTyped"
    }
}

impl EarlyLintPass for ArgumentTyped {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Range>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ReadableArgumentsName {
    #[serde(with = "serde_regex")]
    regex: regex::Regex,
}

impl Default for ReadableArgumentsName {
    fn default() -> Self {
        let regex = regex::Regex::new("^.$").unwrap();
        Self { regex }
    }
}

impl LintPass for ReadableArgumentsName {
    fn name(&self) -> &str {
        "ReadableArgumentsName"
    }
}

impl EarlyLintPass for ReadableArgumentsName {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Range>,
    ) -> Vec<super::lint::LintError> {
        if self.regex.is_match(&arg.name) {
            return vec![LintError::new(
                Box::new(self.clone()),
                &format!("Argument '{}' name is too short", arg.name),
                &arg.extra,
            )];
        }
        vec![]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LowerCaseArgumentName;

impl LintPass for LowerCaseArgumentName {
    fn name(&self) -> &str {
        "LowerCaseArgumentName"
    }
}

impl EarlyLintPass for LowerCaseArgumentName {
    fn check_argument(
        &self,
        arg: &puppet_lang::argument::Argument<Range>,
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
