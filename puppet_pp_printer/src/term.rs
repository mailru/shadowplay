use crate::Printer;
use pretty::{Doc, RcDoc};

impl<EXTRA> Printer for puppet_lang::expression::Float<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(self.value)
    }
}

impl<EXTRA> Printer for puppet_lang::expression::Integer<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(self.value)
    }
}

impl<EXTRA> Printer for puppet_lang::expression::Boolean<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(self.value)
    }
}

impl<EXTRA> Printer for puppet_lang::expression::Usize<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::as_string(self.value)
    }
}

impl<EXTRA> Printer for puppet_lang::expression::Regexp<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::text("/").append(&self.data).append(RcDoc::text("/"))
    }
}

pub fn mapkv_to_doc<EXTRA>(
    expr: &puppet_lang::expression::MapKV<EXTRA>,
    with_indent: bool,
) -> RcDoc<()> {
    crate::expression::to_doc(&expr.key, false)
        .append(RcDoc::column(move |w| {
            if with_indent {
                let offset = (w / 10 + 1) * 10;
                RcDoc::text(format!("{} =>", " ".repeat(offset - w)))
            } else {
                RcDoc::softline().append(RcDoc::text("=>"))
            }
        }))
        .append(RcDoc::softline())
        .append(crate::expression::to_doc(&expr.value, false))
        .group()
        .nest(2)
}

impl<EXTRA> Printer for puppet_lang::expression::Map<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        if self.value.value.len() < 2 && self.value.last_comment.is_empty() {
            let inner = RcDoc::intersperse(
                self.value.value.iter().map(|elt| mapkv_to_doc(elt, false)),
                RcDoc::text(",").append(RcDoc::softline()),
            )
            .append(self.value.last_comment.to_doc());
            RcDoc::text("{")
                .append(RcDoc::softline())
                .append(inner)
                .nest(2)
                .append(RcDoc::softline())
                .append(RcDoc::text("}"))
        } else {
            let inner = RcDoc::intersperse(
                self.value
                    .value
                    .iter()
                    .map(|elt| mapkv_to_doc(elt, true).append(RcDoc::text(","))),
                RcDoc::hardline(),
            )
            .append(self.value.last_comment.to_doc());

            RcDoc::text("{")
                .append(RcDoc::hardline())
                .append(inner)
                .nest(2)
                .append(RcDoc::hardline())
                .append(RcDoc::text("}"))
        }
    }
}

pub fn to_doc<EXTRA>(
    term: &puppet_lang::expression::Term<EXTRA>,
    hide_variable_tag: bool,
) -> RcDoc<()> {
    match &term.value {
        puppet_lang::expression::TermVariant::Float(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::Integer(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::Boolean(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::Parens(v) => RcDoc::text("(")
            .append(crate::expression::to_doc(&v.value, false).nest(2))
            .append(RcDoc::text(")"))
            .group(),
        puppet_lang::expression::TermVariant::Array(v) => RcDoc::text("[")
            .append(RcDoc::line().nest(2))
            .append(
                RcDoc::intersperse(
                    v.value
                        .value
                        .iter()
                        .map(|x| crate::expression::to_doc(x, false).append(RcDoc::text(","))),
                    Doc::line(),
                )
                .group()
                .append(v.value.last_comment.to_doc())
                .nest(2),
            )
            .append(RcDoc::line())
            .append(RcDoc::text("]"))
            .group(),
        puppet_lang::expression::TermVariant::Identifier(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::Map(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::Variable(v) => {
            if hide_variable_tag {
                v.identifier.to_doc()
            } else {
                RcDoc::text("$").append(v.identifier.to_doc())
            }
        }
        puppet_lang::expression::TermVariant::RegexpGroupID(id) => {
            RcDoc::text("$").append(format!("{}", id.identifier))
        }
        puppet_lang::expression::TermVariant::Sensitive(v) => RcDoc::text("Sensitive")
            .append(RcDoc::softline_())
            .append(RcDoc::text("("))
            .append(RcDoc::line())
            .append(to_doc(&v.value, false))
            .append(RcDoc::line())
            .append(RcDoc::text(")"))
            .group(),
        puppet_lang::expression::TermVariant::TypeSpecitifaction(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::Regexp(v) => v.to_doc(),
        puppet_lang::expression::TermVariant::String(v) => v.to_doc(),
    }
}
