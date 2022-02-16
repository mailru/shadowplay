use puppet_parser::parser::Location;

use crate::check::pp_static::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

pub struct UpperCaseName;

impl LintPass for UpperCaseName {
    fn name(&self) -> &str {
        "upper_case_name_of_resource_set"
    }
}

impl EarlyLintPass for UpperCaseName {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        if elt
            .name
            .name
            .iter()
            .any(|v| v.chars().any(|v| v.is_uppercase()))
        {
            return vec![LintError::new(
                self.name(),
                "Name of resource set contains upper case characters",
                &elt.extra,
            )];
        }
        vec![]
    }
}
