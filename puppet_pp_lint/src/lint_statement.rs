use puppet_parser::parser::Location;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct StatementWithNoSideEffects;

impl LintPass for StatementWithNoSideEffects {
    fn name(&self) -> &str {
        "statement_with_no_side_effects"
    }
}

impl EarlyLintPass for StatementWithNoSideEffects {
    fn check_statement(&self, elt: &puppet_lang::statement::Statement<Location>) -> Vec<LintError> {
        if !crate::tool::statement::has_side_effect(elt) {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Statement without side effects. Can be safely removed.",
                &elt.extra,
            )];
        }

        vec![]
    }
}
