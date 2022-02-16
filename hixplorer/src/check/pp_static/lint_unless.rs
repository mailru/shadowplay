use puppet_parser::parser::Location;

use crate::check::pp_static::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

pub struct DoNotUseUnless;

impl LintPass for DoNotUseUnless {
    fn name(&self) -> &str {
        "dont_use_unless"
    }
}

impl EarlyLintPass for DoNotUseUnless {
    fn check_unless(
        &self,
        elt: &puppet_lang::statement::ConditionAndStatement<Location>,
    ) -> Vec<super::lint::LintError> {
        vec![LintError::new(
            self.name(),
            "Use 'if !EXPR { ... }' instead of 'unless EXPR { ... }'",
            &elt.extra,
        )]
    }
}