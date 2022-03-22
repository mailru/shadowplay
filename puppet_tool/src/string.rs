pub fn string_fragment_content<EXTRA>(expr: &puppet_lang::string::StringFragment<EXTRA>) -> String {
    match expr {
        puppet_lang::string::StringFragment::Literal(elt) => elt.data.clone(),
        puppet_lang::string::StringFragment::EscapedUTF(c) => c.data.escape_unicode().to_string(),
        puppet_lang::string::StringFragment::Escaped(c) => format!("\\{}", c.data),
    }
}

pub fn raw_content<EXTRA>(expr: &puppet_lang::string::StringExpr<EXTRA>) -> String {
    match &expr.data {
        puppet_lang::string::StringVariant::SingleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                r.push_str(&string_fragment_content(elt))
            }
            r
        }
        puppet_lang::string::StringVariant::DoubleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                match elt {
                    puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => {
                        r.push_str(&string_fragment_content(elt))
                    }
                    puppet_lang::string::DoubleQuotedFragment::Expression(_) => {
                        // TODO
                    }
                }
            }
            r
        }
    }
}

fn string_fragment_constant_value<EXTRA>(
    expr: &puppet_lang::string::StringFragment<EXTRA>,
) -> String {
    match expr {
        puppet_lang::string::StringFragment::Literal(elt) => elt.data.clone(),
        puppet_lang::string::StringFragment::EscapedUTF(c) => c.data.to_string(),
        puppet_lang::string::StringFragment::Escaped(c) => c.data.to_string(),
    }
}

pub fn constant_value<EXTRA>(expr: &puppet_lang::string::StringExpr<EXTRA>) -> Option<String> {
    match &expr.data {
        puppet_lang::string::StringVariant::SingleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                r.push_str(&string_fragment_constant_value(elt))
            }
            Some(r)
        }
        puppet_lang::string::StringVariant::DoubleQuoted(list) => {
            let mut r = String::new();
            for elt in list {
                match elt {
                    puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => {
                        r.push_str(&string_fragment_constant_value(elt))
                    }
                    puppet_lang::string::DoubleQuotedFragment::Expression(_) => return None,
                }
            }
            Some(r)
        }
    }
}
