use puppet_parser::range::Range;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct InvalidStringEscape;

impl LintPass for InvalidStringEscape {
    fn name(&self) -> &str {
        "invalid_string_escape"
    }
}

impl EarlyLintPass for InvalidStringEscape {
    fn check_string_expression(
        &self,
        elt: &puppet_lang::string::StringExpr<Range>,
    ) -> Vec<super::lint::LintError> {
        let list = match &elt.data {
            puppet_lang::string::StringVariant::SingleQuoted(list) => list.clone(),
            puppet_lang::string::StringVariant::DoubleQuoted(list) => list
                .clone()
                .into_iter()
                .filter_map(|elt| match elt {
                    puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => Some(elt),
                    puppet_lang::string::DoubleQuotedFragment::Expression(_) => None,
                })
                .collect::<Vec<_>>(),
        };

        let mut errors = Vec::new();
        for fragment in list {
            if let puppet_lang::string::StringFragment::Escaped(c) = fragment {
                match &elt.data {
                    puppet_lang::string::StringVariant::SingleQuoted(_) => {
                        if c.data != '\'' && c.data != '\\' {
                            errors.push(LintError::new_with_url(
                                Box::new(self.clone()),
                                &format!("Unexpected escaped character {:?}", c.data),
                                "https://puppet.com/docs/puppet/7/lang_data_string.html#lang_data_string_single_quoted_strings-escape-sequences",
                                &c.extra,
                            ))
                        }
                    }
                    puppet_lang::string::StringVariant::DoubleQuoted(_) => {
                        if c.data != 'n'
                            && c.data != 'r'
                            && c.data != 't'
                            && c.data != 's'
                            && c.data != '$'
                            && c.data != 'b'
                            && c.data != 'f'
                            && c.data != '\\'
                            && c.data != '\"'
                            && c.data != '\''
                        {
                            errors.push(LintError::new_with_url(
                                Box::new(self.clone()),
                                &format!("Unexpected escaped character {:?}", c.data),
                                "https://puppet.com/docs/puppet/7/lang_data_string.html#lang_data_string_double_quoted_strings-escape-sequences",
                                &c.extra,
                            ))
                        }
                    }
                }
            }
        }

        errors
    }
}

#[derive(Clone)]
pub struct UselessDoubleQuotes;

impl LintPass for UselessDoubleQuotes {
    fn name(&self) -> &str {
        "useless_double_quotes"
    }
}

impl EarlyLintPass for UselessDoubleQuotes {
    fn check_string_expression(
        &self,
        elt: &puppet_lang::string::StringExpr<Range>,
    ) -> Vec<super::lint::LintError> {
        let s = match &elt.data {
            puppet_lang::string::StringVariant::SingleQuoted(_) => return Vec::new(),
            puppet_lang::string::StringVariant::DoubleQuoted(elt) => elt,
        };

        let mut is_useful = false;
        for fragment in s {
            match fragment {
                puppet_lang::string::DoubleQuotedFragment::StringFragment(fragment) => {
                    match fragment {
                        puppet_lang::string::StringFragment::Literal(_)
                        | puppet_lang::string::StringFragment::EscapedUTF(_) => {}
                        puppet_lang::string::StringFragment::Escaped(c) if c.data == '\'' => {
                            is_useful = true
                        }
                        puppet_lang::string::StringFragment::Escaped(_) => {}
                    }
                }
                puppet_lang::string::DoubleQuotedFragment::Expression(_) => return Vec::new(),
            }
        }

        if !is_useful {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Double quoted string with no interpolated values and no escaped single quotes",
                &elt.extra,
            )];
        }
        vec![]
    }
}
