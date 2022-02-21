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
    // TODO сделать менее наивную реализацию, с сохранением в EXTRA состояния
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

#[derive(Clone)]
pub struct RelationToTheLeft;

impl LintPass for RelationToTheLeft {
    fn name(&self) -> &str {
        "relation_to_the_left"
    }
}

impl EarlyLintPass for RelationToTheLeft {
    fn check_relation(
        &self,
        _left: &puppet_lang::statement::RelationElt<Location>,
        elt: &puppet_lang::statement::Relation<Location>,
    ) -> Vec<LintError> {
        match elt.relation_type.variant {
            puppet_lang::statement::RelationVariant::ExecOrderRight => (),
            puppet_lang::statement::RelationVariant::NotifyRight => (),
            puppet_lang::statement::RelationVariant::ExecOrderLeft
            | puppet_lang::statement::RelationVariant::NotifyLeft => {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Avoid relations directed to the left.",
                    &elt.relation_type.extra,
                )];
            }
        }

        vec![]
    }
}
