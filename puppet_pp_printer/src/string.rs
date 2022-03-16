use crate::Printer;
use pretty::RcDoc;

impl<EXTRA> Printer for puppet_lang::string::StringFragment<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::string::StringFragment::Literal(v) => RcDoc::text(&v.data),
            puppet_lang::string::StringFragment::EscapedUTF(v) => {
                RcDoc::text(v.data.escape_unicode().to_string())
            }
            puppet_lang::string::StringFragment::Escaped(v) => {
                RcDoc::text("\\").append(RcDoc::text(v.data.to_string()))
            }
        }
    }
}

impl<EXTRA> Printer for puppet_lang::string::DoubleQuotedFragment<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => elt.to_doc(),
            puppet_lang::string::DoubleQuotedFragment::Expression(expr) => {
                let inner_expr = if let puppet_lang::expression::ExpressionVariant::Term(term) =
                    &expr.data.value
                {
                    if let puppet_lang::expression::TermVariant::Variable(_) = &term.value {
                        crate::expression::to_doc(&expr.data, true)
                    } else {
                        crate::expression::to_doc(&expr.data, false)
                    }
                } else {
                    crate::expression::to_doc(&expr.data, false)
                };
                RcDoc::text("${")
                    .append(inner_expr)
                    .append(RcDoc::text("}"))
            }
        }
    }
}

impl<EXTRA> Printer for puppet_lang::string::StringExpr<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match &self.data {
            puppet_lang::string::StringVariant::SingleQuoted(list) => match &list.as_slice() {
                &[puppet_lang::string::StringFragment::Literal(v)]
                    if v.data.chars().all(|c| c.is_alphabetic() || c == '_') =>
                {
                    RcDoc::text(&v.data)
                }
                _ => RcDoc::text("'")
                    .append(RcDoc::intersperse(
                        list.iter().map(|v| v.to_doc()),
                        RcDoc::nil(),
                    ))
                    .append(RcDoc::text("'")),
            },
            puppet_lang::string::StringVariant::DoubleQuoted(list) => RcDoc::text("\"")
                .append(RcDoc::intersperse(
                    list.iter().map(|v| v.to_doc()),
                    RcDoc::nil(),
                ))
                .append(RcDoc::text("\"")),
        }
    }
}
