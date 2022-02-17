use puppet_parser::parser::Location;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct UselessParens;

impl LintPass for UselessParens {
    fn name(&self) -> &str {
        "useless_parens"
    }
}

impl EarlyLintPass for UselessParens {
    fn check_term(
        &self,
        elt: &puppet_lang::expression::Term<Location>,
    ) -> Vec<super::lint::LintError> {
        if let puppet_lang::expression::TermVariant::Parens(inner) = &elt.value {
            match inner.value {
                puppet_lang::expression::ExpressionVariant::Not(_)
                | puppet_lang::expression::ExpressionVariant::ChainCall(_)
                | puppet_lang::expression::ExpressionVariant::Term(_) => {
                    return vec![LintError::new(
                        Box::new(self.clone()),
                        "Useless parens around term, chain call or negation",
                        &elt.extra,
                    )]
                }
                puppet_lang::expression::ExpressionVariant::Multiply(_)
                | puppet_lang::expression::ExpressionVariant::Divide(_)
                | puppet_lang::expression::ExpressionVariant::Modulo(_)
                | puppet_lang::expression::ExpressionVariant::Plus(_)
                | puppet_lang::expression::ExpressionVariant::Minus(_)
                | puppet_lang::expression::ExpressionVariant::ShiftLeft(_)
                | puppet_lang::expression::ExpressionVariant::ShiftRight(_)
                | puppet_lang::expression::ExpressionVariant::Equal(_)
                | puppet_lang::expression::ExpressionVariant::NotEqual(_)
                | puppet_lang::expression::ExpressionVariant::Gt(_)
                | puppet_lang::expression::ExpressionVariant::GtEq(_)
                | puppet_lang::expression::ExpressionVariant::Lt(_)
                | puppet_lang::expression::ExpressionVariant::LtEq(_)
                | puppet_lang::expression::ExpressionVariant::And(_)
                | puppet_lang::expression::ExpressionVariant::Or(_)
                | puppet_lang::expression::ExpressionVariant::Assign(_)
                | puppet_lang::expression::ExpressionVariant::MatchRegex(_)
                | puppet_lang::expression::ExpressionVariant::NotMatchRegex(_)
                | puppet_lang::expression::ExpressionVariant::MatchType(_)
                | puppet_lang::expression::ExpressionVariant::NotMatchType(_)
                | puppet_lang::expression::ExpressionVariant::In(_)
                | puppet_lang::expression::ExpressionVariant::Selector(_) => {
                    // TODO
                }
            }
        }

        vec![]
    }
}

#[derive(Clone)]
pub struct UselessDoubleQuotes;

impl LintPass for UselessDoubleQuotes {
    fn name(&self) -> &str {
        "useless_double_quotes"
    }
}

impl EarlyLintPass for UselessDoubleQuotes {
    fn check_string_expression(
        &self,
        elt: &puppet_lang::expression::StringExpr<Location>,
    ) -> Vec<super::lint::LintError> {
        if elt.variant == puppet_lang::expression::StringVariant::DoubleQuoted
            && !elt.data.contains('$')
            && !elt.data.contains('\'')
            && !elt.data.contains('\\')
        {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Double quotes of string with no interpolated values and no escaped chars [EXPERIMENTAL]",
                &elt.extra,
            )];
        }
        vec![]
    }
}

#[derive(Clone)]
pub struct LowerCaseVariable;

impl LintPass for LowerCaseVariable {
    fn name(&self) -> &str {
        "lower_case_variable"
    }
}

impl EarlyLintPass for LowerCaseVariable {
    fn check_term(
        &self,
        elt: &puppet_lang::expression::Term<Location>,
    ) -> Vec<super::lint::LintError> {
        if let puppet_lang::expression::TermVariant::Variable(var) = &elt.value {
            if var
                .identifier
                .name
                .iter()
                .any(|elt| elt.chars().any(|c| c.is_uppercase()))
            {
                return vec![LintError::new_with_url(
                    Box::new(self.clone()),
                    "Variable name with upper case letters.",
                    "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_variables-variable-format",
                    &elt.extra,
                )];
            }
        }
        vec![]
    }
}