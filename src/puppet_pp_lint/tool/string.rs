pub fn is_constant<EXTRA>(s: &crate::puppet_lang::string::StringExpr<EXTRA>) -> bool {
    match &s.data {
        crate::puppet_lang::string::StringVariant::SingleQuoted(_) => true,
        crate::puppet_lang::string::StringVariant::DoubleQuoted(list) => {
            for elt in list {
                if let crate::puppet_lang::string::DoubleQuotedFragment::Expression(fragment) = elt {
                    if !crate::puppet_pp_lint::tool::expression::is_constant(&fragment.data) {
                        return false;
                    }
                }
            }
            true
        }
    }
}
