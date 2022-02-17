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
            for case in &selector.cases {
                if has_side_effect(&case.body) {
                    return true;
                }
            }
            false
        }
        puppet_lang::expression::ExpressionVariant::ChainCall(_) => true,
        puppet_lang::expression::ExpressionVariant::Term(_) => false,
        puppet_lang::expression::ExpressionVariant::FunctionCall(_) => true, // TODO check function's side effects
    }
}
