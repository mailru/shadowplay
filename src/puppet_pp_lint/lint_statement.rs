use crate::puppet_lang::statement::Statement;
use crate::puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use crate::puppet_lang::ExtraGetter;
use crate::puppet_pp_lint::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct StatementWithNoEffect;

impl LintPass for StatementWithNoEffect {
    fn name(&self) -> &str {
        "StatementWithNoEffect"
    }
    fn description(&self) -> &str {
        "Checks for statements without side effects"
    }
}

impl EarlyLintPass for StatementWithNoEffect {
    fn check_statement_set(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>,
        list: &[crate::puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        let list_len = list.len();
        for (idx, elt) in list.iter().enumerate() {
            if !crate::puppet_pp_lint::tool::statement::has_side_effect(elt) && idx != list_len - 1
            {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct RelationToTheLeft;

impl LintPass for RelationToTheLeft {
    fn name(&self) -> &str {
        "RelationToTheLeft"
    }
    fn description(&self) -> &str {
        "Checks for left-directed relations"
    }
}

impl EarlyLintPass for RelationToTheLeft {
    fn check_relation(
        &self,
        _left: &crate::puppet_lang::statement::RelationElt<Range>,
        elt: &crate::puppet_lang::statement::Relation<Range>,
    ) -> Vec<LintError> {
        match elt.relation_type.variant {
            crate::puppet_lang::statement::RelationVariant::ExecOrderRight => (),
            crate::puppet_lang::statement::RelationVariant::NotifyRight => (),
            crate::puppet_lang::statement::RelationVariant::ExecOrderLeft
            | crate::puppet_lang::statement::RelationVariant::NotifyLeft => {
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
#[derive(Clone, Serialize, Deserialize)]
pub struct DeepCode {
    max_deepness: usize,
}

impl Default for DeepCode {
    fn default() -> Self {
        Self { max_deepness: 9 }
    }
}

impl LintPass for DeepCode {
    fn name(&self) -> &str {
        "DeepCode"
    }
    fn description(&self) -> &str {
        "Warns if statement is too deep"
    }
}

impl EarlyLintPass for DeepCode {
    fn check_statement(
        &self,
        ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>,
        elt: &Statement<Range>,
    ) -> Vec<super::lint::LintError> {
        if ctx.path.len() > self.max_deepness {
            return vec![LintError::new(
                Box::new(self.clone()),
                &format!(
                    "Statement is at level {} deep inside the code.",
                    ctx.path.len()
                ),
                elt.extra(),
            )];
        }

        Vec::new()
    }
}
