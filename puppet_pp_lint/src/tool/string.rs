pub fn is_constant<EXTRA>(s: &puppet_lang::string::StringExpr<EXTRA>) -> bool {
    match &s.data {
        puppet_lang::string::StringVariant::SingleQuoted(_) => true,
        puppet_lang::string::StringVariant::DoubleQuoted(list) => {
            for elt in list {
                if let puppet_lang::string::DoubleQuotedFragment::Expression(fragment) = elt {
                    if !crate::tool::expression::is_constant(&fragment.data) {
                        return false;
                    }
                }
            }
            true
        }
    }
}
