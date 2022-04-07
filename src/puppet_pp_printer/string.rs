use crate::puppet_pp_printer::Printer;
use pretty::RcDoc;

impl<EXTRA> Printer for crate::puppet_lang::string::StringFragment<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            crate::puppet_lang::string::StringFragment::Literal(v) => RcDoc::text(&v.data),
            crate::puppet_lang::string::StringFragment::EscapedUTF(v) => {
                RcDoc::text(v.data.escape_unicode().to_string())
            }
            crate::puppet_lang::string::StringFragment::Escaped(v) => {
                RcDoc::text("\\").append(RcDoc::text(v.data.to_string()))
            }
        }
    }
}

impl<EXTRA> Printer for crate::puppet_lang::string::DoubleQuotedFragment<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            crate::puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => elt.to_doc(),
            crate::puppet_lang::string::DoubleQuotedFragment::Expression(expr) => {
                let inner_expr =
                    if let crate::puppet_lang::expression::ExpressionVariant::Term(term) =
                        &expr.data.value
                    {
                        if let crate::puppet_lang::expression::TermVariant::Variable(_) =
                            &term.value
                        {
                            crate::puppet_pp_printer::expression::to_doc(&expr.data, true)
                        } else {
                            crate::puppet_pp_printer::expression::to_doc(&expr.data, false)
                        }
                    } else {
                        crate::puppet_pp_printer::expression::to_doc(&expr.data, false)
                    };
                RcDoc::text("${")
                    .append(inner_expr)
                    .append(RcDoc::text("}"))
            }
        }
    }
}

impl<EXTRA> Printer for crate::puppet_lang::string::StringExpr<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match &self.data {
            crate::puppet_lang::string::StringVariant::SingleQuoted(list) => {
                // Can we serialize this string as bareword?
                if list.len() == 1 {
                    if let crate::puppet_lang::string::StringFragment::Literal(v) =
                        list.first().unwrap()
                    {
                        if v.data.chars().all(|c| c.is_ascii_lowercase() || c == '_')
                            && !crate::puppet_lang::keywords::KEYWORDS.contains(&v.data.as_str())
                        {
                            return RcDoc::text(&v.data);
                        }
                    }
                }

                RcDoc::text("'")
                    .append(RcDoc::intersperse(
                        list.iter().map(|v| v.to_doc()),
                        RcDoc::nil(),
                    ))
                    .append(RcDoc::text("'"))
            }
            crate::puppet_lang::string::StringVariant::DoubleQuoted(list) => RcDoc::text("\"")
                .append(RcDoc::intersperse(
                    list.iter().map(|v| v.to_doc()),
                    RcDoc::nil(),
                ))
                .append(RcDoc::text("\"")),
        }
    }
}
