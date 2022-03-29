pub fn is_constant<EXTRA>(term: &crate::puppet_lang::expression::Term<EXTRA>) -> bool {
    match &term.value {
        crate::puppet_lang::expression::TermVariant::String(v) => crate::puppet_pp_lint::tool::string::is_constant(v),
        crate::puppet_lang::expression::TermVariant::Float(_) => true,
        crate::puppet_lang::expression::TermVariant::Integer(_) => true,
        crate::puppet_lang::expression::TermVariant::Boolean(_) => true,
        crate::puppet_lang::expression::TermVariant::Array(v) => v
            .value
            .value
            .iter()
            .all(|v| crate::puppet_pp_lint::tool::expression::is_constant(v)),
        crate::puppet_lang::expression::TermVariant::Identifier(_) => {
            // Reference to hiera value
            false
        }
        crate::puppet_lang::expression::TermVariant::Parens(v) => {
            crate::puppet_pp_lint::tool::expression::is_constant(&v.value)
        }
        crate::puppet_lang::expression::TermVariant::Map(_) => todo!(),
        crate::puppet_lang::expression::TermVariant::Variable(_) => false,
        crate::puppet_lang::expression::TermVariant::RegexpGroupID(_) => {
            // TODO context is required for best result
            false
        }
        crate::puppet_lang::expression::TermVariant::Sensitive(v) => is_constant(v.value.as_ref()),
        crate::puppet_lang::expression::TermVariant::TypeSpecitifaction(_) => {
            // TODO are there any cases when it's false?
            true
        }
        crate::puppet_lang::expression::TermVariant::Regexp(_) => true,
    }
}
