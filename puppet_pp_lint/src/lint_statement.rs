use puppet_parser::range::Range;

use crate::lint::{EarlyLintPass, LintError, LintPass};
use puppet_lang::ExtraGetter;

#[derive(Clone)]
pub struct StatementWithNoEffect;

impl LintPass for StatementWithNoEffect {
    fn name(&self) -> &str {
        "statement_with_no_side_effects"
    }
}

impl EarlyLintPass for StatementWithNoEffect {
    // TODO сделать менее наивную реализацию, с сохранением в EXTRA состояния
    fn check_statement_set(
        &self,
        list: &[puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        let list_len = list.len();
        for (idx, elt) in list.iter().enumerate() {
            if !crate::tool::statement::has_side_effect(elt) && idx != list_len - 1 {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Statement without effect which is not a return value. Can be safely removed.",
                    elt.extra(),
                )];
            }
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
        _left: &puppet_lang::statement::RelationElt<Range>,
        elt: &puppet_lang::statement::Relation<Range>,
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
