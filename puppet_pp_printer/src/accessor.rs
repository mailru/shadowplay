use crate::Printer;
use pretty::{Doc, RcDoc};

impl<EXTRA> Printer for Option<puppet_lang::expression::Accessor<EXTRA>> {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            None => RcDoc::nil(),
            Some(v) => {
                let accessor_list = v.list.iter().map(|sublist| {
                    RcDoc::text("[")
                        .append(RcDoc::line_())
                        .append(
                            RcDoc::intersperse(
                                sublist.iter().map(|elt| elt.to_doc()),
                                RcDoc::text(",").append(RcDoc::line()),
                            )
                            .group(),
                        )
                        .nest(2)
                        .append(RcDoc::line_())
                        .append(RcDoc::text("]"))
                        .group()
                });

                RcDoc::intersperse(accessor_list, RcDoc::nil())
                    .group()
                    .nest(2)
            }
        }
    }
}
