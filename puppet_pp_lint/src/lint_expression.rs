use puppet_lang::expression::{Expression, ExpressionVariant, Term, TermVariant};
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
        elt: &Expression<Location>,
    ) {
        let (inner, parens_extra) = match &elt.value {
            ExpressionVariant::Term(puppet_lang::expression::Term {
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
        elt: &Expression<Location>,
    ) -> Vec<super::lint::LintError> {
        let outer_priority = crate::tool::expression::priority(elt);

        let mut errors = Vec::new();

        match &elt.value {
            ExpressionVariant::Assign((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::And((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Or((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Equal((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::NotEqual((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Gt((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::GtEq((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Lt((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::LtEq((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::ShiftLeft((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::ShiftRight((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Plus((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Minus((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Multiply((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Divide((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::Modulo((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right)
            }
            ExpressionVariant::ChainCall(elt) => {
                self.check(outer_priority, &mut errors, &elt.left);
            }
            ExpressionVariant::MatchRegex((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            ExpressionVariant::NotMatchRegex((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            ExpressionVariant::MatchType((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            ExpressionVariant::NotMatchType((left, _)) => {
                self.check(outer_priority, &mut errors, left);
            }
            ExpressionVariant::In(_) => {
                // no inner elements available
            }
            ExpressionVariant::Not(elt) => {
                self.check(outer_priority, &mut errors, elt);
            }
            ExpressionVariant::Selector(elt) => {
                self.check(outer_priority, &mut errors, &elt.condition);
            }
            ExpressionVariant::FunctionCall(_) | ExpressionVariant::Term(_) => {
                // no inner elements available
            }
        }

        if is_toplevel_expr {
            if let ExpressionVariant::Term(t) = &elt.value {
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

#[derive(Clone)]
pub struct AssignmentToInvalidExpression;

impl LintPass for AssignmentToInvalidExpression {
    fn name(&self) -> &str {
        "assignment_to_invalid_expression"
    }
}

impl EarlyLintPass for AssignmentToInvalidExpression {
    fn check_expression(
        &self,
        _is_toplevel_expr: bool,
        elt: &Expression<Location>,
    ) -> Vec<super::lint::LintError> {
        if let ExpressionVariant::Assign((left, _)) = &elt.value {
            if !matches!(
                left.value,
                ExpressionVariant::Term(Term {
                    value: TermVariant::Variable(_),
                    ..
                })
            ) {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Left operand of assignment operator must be a variable",
                    &elt.extra,
                )];
            }
        }

        Vec::new()
    }
}

#[derive(Clone)]
pub struct DoubleNegation;

impl LintPass for DoubleNegation {
    fn name(&self) -> &str {
        "double_negation"
    }
}

impl EarlyLintPass for DoubleNegation {
    fn check_expression(
        &self,
        _is_toplevel_expr: bool,
        elt: &Expression<Location>,
    ) -> Vec<super::lint::LintError> {
        let inner = match &elt.value {
            ExpressionVariant::Not(inner) => inner,
            _ => return Vec::new(),
        };

        if matches!(inner.value, ExpressionVariant::Not(_)) {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Double negation",
                &elt.extra,
            )];
        }

        Vec::new()
    }
}

#[derive(Clone)]
pub struct NegationOfEquation;

impl LintPass for NegationOfEquation {
    fn name(&self) -> &str {
        "negation_of_equation"
    }
}

impl EarlyLintPass for NegationOfEquation {
    fn check_expression(
        &self,
        _is_toplevel_expr: bool,
        elt: &Expression<Location>,
    ) -> Vec<super::lint::LintError> {
        let inner = match &elt.value {
            ExpressionVariant::Not(inner) => inner,
            _ => return Vec::new(),
        };

        let inner = match &inner.value {
            ExpressionVariant::Term(Term {
                value: TermVariant::Parens(inner),
                ..
            }) => inner,
            _ => return Vec::new(),
        };

        if matches!(inner.value.value, ExpressionVariant::Equal(_)) {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Negation of equality. !($a == $b) can be replaced with $a != $b",
                &elt.extra,
            )];
        }

        if matches!(inner.value.value, ExpressionVariant::NotEqual(_)) {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Negation of inequality. !($a != $b) can be replaced with $a == $b",
                &elt.extra,
            )];
        }

        if matches!(inner.value.value, ExpressionVariant::NotMatchType(_))
            || matches!(inner.value.value, ExpressionVariant::NotMatchRegex(_))
        {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Negation of negative match. !($a !~ $b) can be replaced with $a ~= $b",
                &elt.extra,
            )];
        }

        if matches!(inner.value.value, ExpressionVariant::MatchType(_))
            || matches!(inner.value.value, ExpressionVariant::MatchRegex(_))
        {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Negation of matching. !($a ~= $b) can be replaced with $a !~ $b",
                &elt.extra,
            )];
        }

        Vec::new()
    }
}
