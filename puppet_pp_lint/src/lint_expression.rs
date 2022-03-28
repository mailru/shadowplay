use puppet_lang::expression::{Expression, ExpressionVariant, Parens, Term, TermVariant};
use puppet_parser::range::Range;

use crate::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

#[derive(Clone)]
pub struct UselessParens;

impl LintPass for UselessParens {
    fn name(&self) -> &str {
        "UselessParens"
    }
}

impl UselessParens {
    fn check(
        &self,
        outer_priority: u32,
        errors: &mut Vec<super::lint::LintError>,
        elt: &Expression<Range>,
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

    fn inner_check<C>(&self, elt: &Expression<Range>, checker: C) -> bool
    where
        C: Fn(&ExpressionVariant<Range>) -> bool,
    {
        match &elt.value {
            ExpressionVariant::Term(Term {
                value: TermVariant::Parens(inner),
                ..
            }) => checker(&inner.value.value),
            _ => false,
        }
    }
}

impl EarlyLintPass for UselessParens {
    fn check_expression(
        &self,
        _ctx: &crate::ctx::Ctx,
        is_toplevel_expr: bool,
        elt: &Expression<Range>,
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
                self.check(outer_priority, &mut errors, right);
                if self.inner_check(left, |elt| matches!(elt, ExpressionVariant::And(_))) {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '($a and $b) and $c' can be replaced with '$a and $b and $c'",
                        &left.extra,
                    ));
                }
                if self.inner_check(right, |elt| matches!(elt, ExpressionVariant::And(_))) {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '$a and ($b and $c)' can be replaced with '$a and $b and $c'",
                        &right.extra,
                    ));
                }
            }
            ExpressionVariant::Or((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right);
                if self.inner_check(left, |elt| matches!(elt, ExpressionVariant::Or(_))) {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '($a or $b) or $c' can be replaced with '$a or $b or $c'",
                        &left.extra,
                    ));
                }
                if self.inner_check(right, |elt| matches!(elt, ExpressionVariant::Or(_))) {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '$a or ($b or $c)' can be replaced with '$a or $b or $c'",
                        &right.extra,
                    ));
                }
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
            ExpressionVariant::Plus((left, right)) | ExpressionVariant::Minus((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right);
                let eq = self.inner_check(left, |elt| {
                    matches!(elt, ExpressionVariant::Plus(_))
                        || matches!(elt, ExpressionVariant::Minus(_))
                });
                if eq {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '($a + $b) + $c' can be replaced with '$a + $b + $c'",
                        &left.extra,
                    ));
                }
                let eq = self.inner_check(right, |elt| {
                    matches!(elt, ExpressionVariant::Plus(_))
                        || matches!(elt, ExpressionVariant::Minus(_))
                });
                if eq {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '$a + ($b + $c)' can be replaced with '$a + $b + $c'",
                        &right.extra,
                    ));
                }
            }
            ExpressionVariant::Multiply((left, right))
            | ExpressionVariant::Divide((left, right)) => {
                self.check(outer_priority, &mut errors, left);
                self.check(outer_priority, &mut errors, right);
                let eq = self.inner_check(left, |elt| {
                    matches!(elt, ExpressionVariant::Multiply(_))
                        || matches!(elt, ExpressionVariant::Divide(_))
                });
                if eq {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '($a * $b) * $c' can be replaced with '$a * $b * $c'",
                        &left.extra,
                    ));
                }
                let eq = self.inner_check(right, |elt| {
                    matches!(elt, ExpressionVariant::Multiply(_))
                        || matches!(elt, ExpressionVariant::Divide(_))
                });
                if eq {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Parens can be safely removed. '$a * ($b * $c)' can be replaced with '$a * $b * $c'",
                        &right.extra,
                    ));
                }
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
            ExpressionVariant::FunctionCall(_) => {
                // no inner elements available
            }
            ExpressionVariant::BuiltinFunction(_) => {
                // no inner elements available
            }
            ExpressionVariant::Term(elt) => {
                if let TermVariant::Parens(Parens { value: inner, .. }) = &elt.value {
                    if let ExpressionVariant::Term(elt) = &inner.value {
                        if let TermVariant::Parens(_) = &elt.value {
                            errors.push(LintError::new(
                                Box::new(self.clone()),
                                "Double parens. Can be safely removed.",
                                &elt.extra,
                            ));
                        }
                    }
                }
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
pub struct DoubleNegation;

impl LintPass for DoubleNegation {
    fn name(&self) -> &str {
        "DoubleNegation"
    }
}

impl EarlyLintPass for DoubleNegation {
    fn check_expression(
        &self,
        _ctx: &crate::ctx::Ctx,
        _is_toplevel_expr: bool,
        elt: &Expression<Range>,
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
        "NegationOfEquation"
    }
}

impl EarlyLintPass for NegationOfEquation {
    fn check_expression(
        &self,
        _ctx: &crate::ctx::Ctx,
        _is_toplevel_expr: bool,
        elt: &Expression<Range>,
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

#[derive(Clone)]
pub struct ConstantExpressionInCondition;

impl LintPass for ConstantExpressionInCondition {
    fn name(&self) -> &str {
        "ConstantExpressionInCondition"
    }
}

impl EarlyLintPass for ConstantExpressionInCondition {
    fn check_condition_expression(&self, elt: &Expression<Range>) -> Vec<super::lint::LintError> {
        if crate::tool::expression::is_constant(elt) {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Constant expression in condition",
                &elt.extra,
            )];
        }

        Vec::new()
    }
}

#[derive(Clone)]
pub struct InvalidVariableAssignment;

impl LintPass for InvalidVariableAssignment {
    fn name(&self) -> &str {
        "InvalidVariableAssignment"
    }
}

impl EarlyLintPass for InvalidVariableAssignment {
    fn check_expression(
        &self,
        _ctx: &crate::ctx::Ctx,
        _is_toplevel_expr: bool,
        elt: &Expression<Range>,
    ) -> Vec<super::lint::LintError> {
        let left = match &elt.value {
            ExpressionVariant::Assign((left, _)) => left,
            _ => return vec![],
        };

        let term = match &left.value {
            ExpressionVariant::Term(term) => term,
            _ => {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Assignment to unexpected expression type",
                    &elt.extra,
                )];
            }
        };

        let variable = match &term.value {
            TermVariant::Variable(v) => v,
            TermVariant::Array(_) => {
                // TODO check recursively
                return vec![];
            }
            _ => {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Assignment to unexpected expression type",
                    &elt.extra,
                )];
            }
        };

        if variable.identifier.is_toplevel {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Assignment to variable in global scope",
                &elt.extra,
            )];
        }

        let name = match &variable.identifier.name.as_slice() {
            &[name] => name,
            _ => {
                return vec![LintError::new(
                    Box::new(self.clone()),
                    "Assignment to variable with namespace",
                    &elt.extra,
                )];
            }
        };

        if name == "name" || name == "facts" || name == "trusted" || name == "server_facts" {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Assignment to reserved variable name",
                &elt.extra,
            )];
        }

        Vec::new()
    }
}
