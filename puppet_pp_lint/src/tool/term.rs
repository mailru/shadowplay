pub fn is_constant<EXTRA>(term: &puppet_lang::expression::Term<EXTRA>) -> bool {
    match &term.value {
        puppet_lang::expression::TermVariant::String(v) => crate::tool::string::is_constant(v),
        puppet_lang::expression::TermVariant::Float(_) => true,
        puppet_lang::expression::TermVariant::Integer(_) => true,
        puppet_lang::expression::TermVariant::Boolean(_) => true,
        puppet_lang::expression::TermVariant::Array(v) => v
            .value
            .value
            .iter()
            .all(|v| crate::tool::expression::is_constant(v)),
        puppet_lang::expression::TermVariant::Identifier(_) => {
            // Reference to hiera value
            false
        }
        puppet_lang::expression::TermVariant::Parens(v) => {
            crate::tool::expression::is_constant(&v.value)
        }
        puppet_lang::expression::TermVariant::Map(_) => todo!(),
        puppet_lang::expression::TermVariant::Variable(_) => false,
        puppet_lang::expression::TermVariant::RegexpGroupID(_) => {
            // TODO context is required for best result
            false
        }
        puppet_lang::expression::TermVariant::Sensitive(v) => is_constant(v.value.as_ref()),
        puppet_lang::expression::TermVariant::TypeSpecitifaction(_) => {
            // TODO are there any cases when it's false?
            true
        }
        puppet_lang::expression::TermVariant::Regexp(_) => true,
    }
}
