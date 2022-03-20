pub fn has_side_effect<EXTRA>(expr: &puppet_lang::expression::Expression<EXTRA>) -> bool {
    match &expr.value {
        puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::And((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
            has_side_effect(left) || has_side_effect(right)
        }
        puppet_lang::expression::ExpressionVariant::Assign(_) => true,
        puppet_lang::expression::ExpressionVariant::MatchRegex((left, _)) => has_side_effect(left),
        puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, _)) => {
            has_side_effect(left)
        }
        puppet_lang::expression::ExpressionVariant::MatchType((left, _)) => has_side_effect(left),
        puppet_lang::expression::ExpressionVariant::NotMatchType((left, _)) => {
            has_side_effect(left)
        }
        puppet_lang::expression::ExpressionVariant::In(_) => false,
        puppet_lang::expression::ExpressionVariant::Not(inner) => has_side_effect(inner),
        puppet_lang::expression::ExpressionVariant::Selector(selector) => {
            for case in &selector.cases.value {
                if has_side_effect(&case.body) {
                    return true;
                }
            }
            false
        }
        puppet_lang::expression::ExpressionVariant::ChainCall(_) => true,
        puppet_lang::expression::ExpressionVariant::Term(_) => false,
        puppet_lang::expression::ExpressionVariant::FunctionCall(_) => true, // TODO check function's side effects
        puppet_lang::expression::ExpressionVariant::BuiltinFunction(_) => true, // TODO check function's side effects
    }
}

pub fn priority<EXTRA>(expr: &puppet_lang::expression::Expression<EXTRA>) -> u32 {
    match expr.value {
        puppet_lang::expression::ExpressionVariant::Assign(_) => 1,
        puppet_lang::expression::ExpressionVariant::And(_)
        | puppet_lang::expression::ExpressionVariant::Or(_) => 2,
        puppet_lang::expression::ExpressionVariant::Equal(_)
        | puppet_lang::expression::ExpressionVariant::NotEqual(_)
        | puppet_lang::expression::ExpressionVariant::Gt(_)
        | puppet_lang::expression::ExpressionVariant::GtEq(_)
        | puppet_lang::expression::ExpressionVariant::Lt(_)
        | puppet_lang::expression::ExpressionVariant::LtEq(_) => 3,
        puppet_lang::expression::ExpressionVariant::In(_)
        | puppet_lang::expression::ExpressionVariant::Selector(_)
        | puppet_lang::expression::ExpressionVariant::ShiftLeft(_)
        | puppet_lang::expression::ExpressionVariant::ShiftRight(_)
        | puppet_lang::expression::ExpressionVariant::Plus(_)
        | puppet_lang::expression::ExpressionVariant::Minus(_)
        | puppet_lang::expression::ExpressionVariant::Multiply(_)
        | puppet_lang::expression::ExpressionVariant::Divide(_)
        | puppet_lang::expression::ExpressionVariant::Modulo(_)
        | puppet_lang::expression::ExpressionVariant::ChainCall(_)
        | puppet_lang::expression::ExpressionVariant::MatchRegex(_)
        | puppet_lang::expression::ExpressionVariant::NotMatchRegex(_)
        | puppet_lang::expression::ExpressionVariant::MatchType(_)
        | puppet_lang::expression::ExpressionVariant::NotMatchType(_)
        | puppet_lang::expression::ExpressionVariant::Not(_)
        | puppet_lang::expression::ExpressionVariant::FunctionCall(_)
        | puppet_lang::expression::ExpressionVariant::BuiltinFunction(_)
        | puppet_lang::expression::ExpressionVariant::Term(_) => 4,
    }
}

pub fn is_constant<EXTRA>(expr: &puppet_lang::expression::Expression<EXTRA>) -> bool {
    match &expr.value {
        puppet_lang::expression::ExpressionVariant::Assign((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::And((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Or((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Equal((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::NotEqual((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Gt((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::GtEq((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Lt((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::LtEq((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::ShiftLeft((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::ShiftRight((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Plus((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Minus((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Multiply((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Divide((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Modulo((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::ChainCall(_) => {
            // TODO
            false
        }
        puppet_lang::expression::ExpressionVariant::MatchRegex((left, _right)) => is_constant(left),
        puppet_lang::expression::ExpressionVariant::NotMatchRegex((left, _right)) => {
            is_constant(left)
        }
        puppet_lang::expression::ExpressionVariant::MatchType((left, _right)) => is_constant(left),
        puppet_lang::expression::ExpressionVariant::NotMatchType((left, _right)) => {
            is_constant(left)
        }
        puppet_lang::expression::ExpressionVariant::In((left, right)) => {
            is_constant(left) && is_constant(right)
        }
        puppet_lang::expression::ExpressionVariant::Not(inner) => is_constant(inner),
        puppet_lang::expression::ExpressionVariant::Selector(v) => {
            if !is_constant(&v.condition) {
                return false;
            }
            for case in &v.cases.value {
                if let puppet_lang::expression::CaseVariant::Term(term) = &case.case {
                    if !crate::tool::term::is_constant(term) {
                        return false;
                    }
                }
            }
            true
        }
        puppet_lang::expression::ExpressionVariant::FunctionCall(_) => {
            // TODO
            false
        }
        puppet_lang::expression::ExpressionVariant::BuiltinFunction(v) => {
            crate::tool::builtin::is_constant(v)
        }
        puppet_lang::expression::ExpressionVariant::Term(inner) => {
            crate::tool::term::is_constant(inner)
        }
    }
}
