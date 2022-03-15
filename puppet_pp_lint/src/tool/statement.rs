pub fn has_side_effect<EXTRA>(statement: &puppet_lang::statement::Statement<EXTRA>) -> bool {
    match &statement.value {
        puppet_lang::statement::StatementVariant::Expression(expr) => {
            crate::tool::expression::has_side_effect(expr)
        }
        puppet_lang::statement::StatementVariant::RelationList(_) => true,
        puppet_lang::statement::StatementVariant::IfElse(v) => {
            crate::tool::expression::has_side_effect(&v.condition.condition)
                || v.condition.body.value.iter().any(has_side_effect)
                || v.elsif_list.iter().any(|elt| {
                    crate::tool::expression::has_side_effect(&elt.condition)
                        || elt.body.value.iter().any(has_side_effect)
                })
                || v.else_block
                    .as_ref()
                    .map(|list| list.value.iter().any(has_side_effect))
                    .unwrap_or(false)
        }
        puppet_lang::statement::StatementVariant::Unless(cond) => {
            crate::tool::expression::has_side_effect(&cond.condition)
                || cond.body.value.iter().any(has_side_effect)
        }
        puppet_lang::statement::StatementVariant::Case(case) => {
            crate::tool::expression::has_side_effect(&case.condition)
                || case.elements.value.iter().any(|elt| {
                    elt.body
                        .value
                        .iter()
                        .any(|statement| has_side_effect(statement))
                })
        }
        puppet_lang::statement::StatementVariant::Toplevel(_) => true,
        puppet_lang::statement::StatementVariant::ResourceDefaults(v) => v
            .args
            .value
            .iter()
            .any(|(_k, v)| crate::tool::expression::has_side_effect(v)),
    }
}
