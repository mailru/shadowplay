use crate::Printer;
use pretty::{Doc, RcDoc};

impl<EXTRA> Printer for puppet_lang::expression::Term<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        match &self.value {
            puppet_lang::expression::TermVariant::Float(v) => RcDoc::as_string(v.value),
            puppet_lang::expression::TermVariant::Integer(v) => RcDoc::as_string(v.value),
            puppet_lang::expression::TermVariant::Boolean(v) => RcDoc::as_string(v.value),
            puppet_lang::expression::TermVariant::Parens(v) => RcDoc::text("(")
                .append(v.value.to_doc().nest(2))
                .append(RcDoc::text(")"))
                .group(),
            puppet_lang::expression::TermVariant::Array(v) => RcDoc::text("[")
                .append(RcDoc::line().nest(2))
                .append(
                    RcDoc::intersperse(
                        v.value
                            .value
                            .iter()
                            .map(|x| x.to_doc().append(RcDoc::text(","))),
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
            puppet_lang::expression::TermVariant::Map(_) => todo!(),
            puppet_lang::expression::TermVariant::Variable(v) => {
                RcDoc::text("$").append(v.identifier.to_doc())
            }
            puppet_lang::expression::TermVariant::RegexpGroupID(_) => todo!(),
            puppet_lang::expression::TermVariant::Sensitive(v) => RcDoc::text("Sensitive")
                .append(RcDoc::softline_())
                .append(RcDoc::text("("))
                .append(RcDoc::line())
                .append(v.value.to_doc())
                .append(RcDoc::line())
                .append(RcDoc::text(")"))
                .group(),
            puppet_lang::expression::TermVariant::TypeSpecitifaction(_) => todo!(),
            puppet_lang::expression::TermVariant::Regexp(v) => {
                RcDoc::text("/").append(&v.data).append(RcDoc::text("/"))
            }
            puppet_lang::expression::TermVariant::String(_) => todo!(),
        }
    }
}
