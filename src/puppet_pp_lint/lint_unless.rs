use crate::puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use crate::puppet_pp_lint::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct DoNotUseUnless;

impl LintPass for DoNotUseUnless {
    fn name(&self) -> &str {
        "DoNotUseUnless"
    }
    fn description(&self) -> &str {
        "Warns if 'unless' conditional statement is used"
    }
}

impl EarlyLintPass for DoNotUseUnless {
    fn check_unless(
        &self,
        elt: &crate::puppet_lang::statement::ConditionAndStatement<Range>,
    ) -> Vec<super::lint::LintError> {
        vec![LintError::new(
            Box::new(self.clone()),
            "Use 'if !EXPR { ... }' instead of 'unless EXPR { ... }'",
            &elt.extra,
        )]
    }
}
