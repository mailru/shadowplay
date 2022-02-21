use puppet_parser::parser::Location;

use crate::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

#[derive(Clone)]
pub struct UselessParens;

impl LintPass for UselessParens {
    fn name(&self) -> &str {
        "useless_parens_expr"
    }
}

impl UselessParens {
    fn check(
        &self,
        outer_priority: u32,
        errors: &mut Vec<super::lint::LintError>,
        elt: &puppet_lang::expression::Expression<Location>,
    ) {
        let (inner, parens_extra) = match &elt.value {
            puppet_lang::expression::ExpressionVariant::Term(puppet_lang::expression::Term {
                value: puppet_lang::expression::TermVariant::Parens(inner),
                extra,
            }) => (inner, extra),
            _ => return,
        };

        let inner_priority = crate::tool::expression::priority(&inner.value);
        if outer_priority < inner_priority {
            errors.push(LintError::new(
                Box::new(self.clone()),
                "Parens can be safely removed",
                parens_extra,
            ));
            return;
        }

        if matches!(
            inner.value.value,
            puppet_lang::expression::ExpressionVariant::Term(_)
        ) {
            errors.push(LintError::new(
                Box::new(self.clone()),
                "Parens around term can be safely removed",
                parens_extra,
            ));
        }
    }
}

impl EarlyLintPass for UselessParens {
    fn check_expression(
        &self,
        is_toplevel_expr: bool,
        elt: &puppet_lang::expression::Expression<Location>,
    ) -> Vec<super::lint::LintError> {
        let outer_priority = crate::tool::expression::priority(elt);

        let mut errors = Vec::new();

        match &elt.value {
            puppet_lang::expression::ExpressionVariant::Assign((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::And((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            puppet_lang::expression::ExpressionVariant::ChainCall(elt) => {
                self.check(outer_priority, &mut errors, &elt.left);
            }
            puppet_lang::expression::ExpressionVariant::MatchRegex((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            puppet_lang::expression::ExpressionVariant::MatchType((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            puppet_lang::expression::ExpressionVariant::NotMatchType((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            puppet_lang::expression::ExpressionVariant::In(_) => {
                // no inner elements available
            }
            puppet_lang::expression::ExpressionVariant::Not(elt) => {
                self.check(outer_priority, &mut errors, elt);
            }
            puppet_lang::expression::ExpressionVariant::Selector(_) => {
                todo!()
            }
            puppet_lang::expression::ExpressionVariant::FunctionCall(_)
            | puppet_lang::expression::ExpressionVariant::Term(_) => {
                // no inner elements available
            }
        }

        if is_toplevel_expr {
            if let puppet_lang::expression::ExpressionVariant::Term(t) = &elt.value {
                if matches!(t.value, puppet_lang::expression::TermVariant::Parens(_)) {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Toplevel parens can be safely removed",
                        &elt.extra,
                    ));
                }
            }
        }

        errors
    }
}
