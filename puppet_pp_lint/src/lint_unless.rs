use puppet_parser::range::Range;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct DoNotUseUnless;

impl LintPass for DoNotUseUnless {
    fn name(&self) -> &str {
        "DoNotUseUnless"
    }
}

impl EarlyLintPass for DoNotUseUnless {
    fn check_unless(
        &self,
        elt: &puppet_lang::statement::ConditionAndStatement<Range>,
    ) -> Vec<super::lint::LintError> {
        vec![LintError::new(
            Box::new(self.clone()),
            "Use 'if !EXPR { ... }' instead of 'unless EXPR { ... }'",
            &elt.extra,
        )]
    }
}
