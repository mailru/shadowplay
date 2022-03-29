use crate::puppet_lang::expression::{ExpressionVariant, TermVariant};

pub fn string_constant_value<EXTRA>(
    expr: &crate::puppet_lang::expression::Expression<EXTRA>,
) -> Option<String> {
    match &expr.value {
        ExpressionVariant::Term(term) => match &term.value {
            TermVariant::String(value) => crate::puppet_tool::string::constant_value(value),
            TermVariant::Float(_)
            | TermVariant::Integer(_)
            | TermVariant::Boolean(_)
            | TermVariant::Array(_)
            | TermVariant::Identifier(_)
            | TermVariant::Parens(_)
            | TermVariant::Map(_)
            | TermVariant::Variable(_)
            | TermVariant::RegexpGroupID(_)
            | TermVariant::Sensitive(_)
            | TermVariant::TypeSpecitifaction(_)
            | TermVariant::Regexp(_) => None,
        },
        ExpressionVariant::Assign(_)
        | ExpressionVariant::And(_)
        | ExpressionVariant::Or(_)
        | ExpressionVariant::Equal(_)
        | ExpressionVariant::NotEqual(_)
        | ExpressionVariant::Gt(_)
        | ExpressionVariant::GtEq(_)
        | ExpressionVariant::Lt(_)
        | ExpressionVariant::LtEq(_)
        | ExpressionVariant::ShiftLeft(_)
        | ExpressionVariant::ShiftRight(_)
        | ExpressionVariant::Plus(_)
        | ExpressionVariant::Minus(_)
        | ExpressionVariant::Multiply(_)
        | ExpressionVariant::Divide(_)
        | ExpressionVariant::Modulo(_)
        | ExpressionVariant::ChainCall(_)
        | ExpressionVariant::MatchRegex(_)
        | ExpressionVariant::NotMatchRegex(_)
        | ExpressionVariant::MatchType(_)
        | ExpressionVariant::NotMatchType(_)
        | ExpressionVariant::In(_)
        | ExpressionVariant::Not(_)
        | ExpressionVariant::Selector(_)
        | ExpressionVariant::FunctionCall(_)
        | ExpressionVariant::BuiltinFunction(_) => None,
    }
}
