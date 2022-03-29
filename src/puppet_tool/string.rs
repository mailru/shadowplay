pub fn string_fragment_content<EXTRA>(expr: &crate::puppet_lang::string::StringFragment<EXTRA>) -> String {
    match expr {
        crate::puppet_lang::string::StringFragment::Literal(elt) => elt.data.clone(),
        crate::puppet_lang::string::StringFragment::EscapedUTF(c) => c.data.escape_unicode().to_string(),
        crate::puppet_lang::string::StringFragment::Escaped(c) => format!("\\{}", c.data),
    }
}

pub fn raw_content<EXTRA>(expr: &crate::puppet_lang::string::StringExpr<EXTRA>) -> String {
    match &expr.data {
        crate::puppet_lang::string::StringVariant::SingleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                r.push_str(&string_fragment_content(elt))
            }
            r
        }
        crate::puppet_lang::string::StringVariant::DoubleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                match elt {
                    crate::puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => {
                        r.push_str(&string_fragment_content(elt))
                    }
                    crate::puppet_lang::string::DoubleQuotedFragment::Expression(_) => {
                        // TODO
                    }
                }
            }
            r
        }
    }
}

fn string_fragment_constant_value<EXTRA>(
    expr: &crate::puppet_lang::string::StringFragment<EXTRA>,
) -> String {
    match expr {
        crate::puppet_lang::string::StringFragment::Literal(elt) => elt.data.clone(),
        crate::puppet_lang::string::StringFragment::EscapedUTF(c) => c.data.to_string(),
        crate::puppet_lang::string::StringFragment::Escaped(c) => c.data.to_string(),
    }
}

pub fn constant_value<EXTRA>(expr: &crate::puppet_lang::string::StringExpr<EXTRA>) -> Option<String> {
    match &expr.data {
        crate::puppet_lang::string::StringVariant::SingleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                r.push_str(&string_fragment_constant_value(elt))
            }
            Some(r)
        }
        crate::puppet_lang::string::StringVariant::DoubleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                match elt {
                    crate::puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => {
                        r.push_str(&string_fragment_constant_value(elt))
                    }
                    crate::puppet_lang::string::DoubleQuotedFragment::Expression(_) => return None,
                }
            }
            Some(r)
        }
    }
}
