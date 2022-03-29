pub fn has_side_effect<EXTRA>(expr: &crate::puppet_lang::expression::Expression<EXTRA>) -> bool {
    match &expr.value {
        crate::puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::And((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Assign(_) => true,
        crate::puppet_lang::expression::ExpressionVariant::MatchRegex((left, _)) => has_side_effect(left),
        crate::puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, _)) => {
            has_side_effect(left)
        }
        crate::puppet_lang::expression::ExpressionVariant::MatchType((left, _)) => has_side_effect(left),
        crate::puppet_lang::expression::ExpressionVariant::NotMatchType((left, _)) => {
            has_side_effect(left)
        }
        crate::puppet_lang::expression::ExpressionVariant::In(_) => false,
        crate::puppet_lang::expression::ExpressionVariant::Not(inner) => has_side_effect(inner),
        crate::puppet_lang::expression::ExpressionVariant::Selector(selector) => {
            for case in &selector.cases.value {
                if has_side_effect(&case.body) {
                    return true;
                }
            }
            false
        }
        crate::puppet_lang::expression::ExpressionVariant::ChainCall(_) => true,
        crate::puppet_lang::expression::ExpressionVariant::Term(_) => false,
        crate::puppet_lang::expression::ExpressionVariant::FunctionCall(_) => true, // TODO check function's side effects
        crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(_) => true, // TODO check function's side effects
    }
}

pub fn priority<EXTRA>(expr: &crate::puppet_lang::expression::Expression<EXTRA>) -> u32 {
    match expr.value {
        crate::puppet_lang::expression::ExpressionVariant::Assign(_) => 1,
        crate::puppet_lang::expression::ExpressionVariant::And(_)
        | crate::puppet_lang::expression::ExpressionVariant::Or(_) => 2,
        crate::puppet_lang::expression::ExpressionVariant::Equal(_)
        | crate::puppet_lang::expression::ExpressionVariant::NotEqual(_)
        | crate::puppet_lang::expression::ExpressionVariant::Gt(_)
        | crate::puppet_lang::expression::ExpressionVariant::GtEq(_)
        | crate::puppet_lang::expression::ExpressionVariant::Lt(_)
        | crate::puppet_lang::expression::ExpressionVariant::LtEq(_) => 3,
        crate::puppet_lang::expression::ExpressionVariant::In(_)
        | crate::puppet_lang::expression::ExpressionVariant::Selector(_)
        | crate::puppet_lang::expression::ExpressionVariant::ShiftLeft(_)
        | crate::puppet_lang::expression::ExpressionVariant::ShiftRight(_)
        | crate::puppet_lang::expression::ExpressionVariant::Plus(_)
        | crate::puppet_lang::expression::ExpressionVariant::Minus(_)
        | crate::puppet_lang::expression::ExpressionVariant::Multiply(_)
        | crate::puppet_lang::expression::ExpressionVariant::Divide(_)
        | crate::puppet_lang::expression::ExpressionVariant::Modulo(_)
        | crate::puppet_lang::expression::ExpressionVariant::ChainCall(_)
        | crate::puppet_lang::expression::ExpressionVariant::MatchRegex(_)
        | crate::puppet_lang::expression::ExpressionVariant::NotMatchRegex(_)
        | crate::puppet_lang::expression::ExpressionVariant::MatchType(_)
        | crate::puppet_lang::expression::ExpressionVariant::NotMatchType(_)
        | crate::puppet_lang::expression::ExpressionVariant::Not(_)
        | crate::puppet_lang::expression::ExpressionVariant::FunctionCall(_)
        | crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(_)
        | crate::puppet_lang::expression::ExpressionVariant::Term(_) => 4,
    }
}

pub fn is_constant<EXTRA>(expr: &crate::puppet_lang::expression::Expression<EXTRA>) -> bool {
    match &expr.value {
        crate::puppet_lang::expression::ExpressionVariant::Assign((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::And((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::ChainCall(_) => {
            // TODO
            false
        }
        crate::puppet_lang::expression::ExpressionVariant::MatchRegex((left, _right)) => is_constant(left),
        crate::puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, _right)) => {
            is_constant(left)
        }
        crate::puppet_lang::expression::ExpressionVariant::MatchType((left, _right)) => is_constant(left),
        crate::puppet_lang::expression::ExpressionVariant::NotMatchType((left, _right)) => {
            is_constant(left)
        }
        crate::puppet_lang::expression::ExpressionVariant::In((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        crate::puppet_lang::expression::ExpressionVariant::Not(inner) => is_constant(inner),
        crate::puppet_lang::expression::ExpressionVariant::Selector(v) => {
            if !is_constant(&v.condition) {
                return false;
            }
            for case in &v.cases.value {
                if let crate::puppet_lang::expression::CaseVariant::Term(term) = &case.case {
                    if !crate::puppet_pp_lint::tool::term::is_constant(term) {
                        return false;
                    }
                }
            }
            true
        }
        crate::puppet_lang::expression::ExpressionVariant::FunctionCall(_) => {
            // TODO
            false
        }
        crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(v) => {
            crate::puppet_pp_lint::tool::builtin::is_constant(v)
        }
        crate::puppet_lang::expression::ExpressionVariant::Term(inner) => {
            crate::puppet_pp_lint::tool::term::is_constant(inner)
        }
    }
}
