use crate::Printer;
use pretty::{Doc, RcDoc};

impl<EXTRA> Printer for puppet_lang::comment::Comment<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::intersperse(
            self.value
                .split('\n')
                .map(|line| RcDoc::text("#").append(line).append(Doc::hardline())),
            Doc::nil(),
        )
    }
}

impl<EXTRA> Printer for Vec<puppet_lang::comment::Comment<EXTRA>> {
    fn to_doc(&self) -> RcDoc<()> {
        if self.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::hardline().append(RcDoc::intersperse(
                self.iter().map(|elt| elt.to_doc()),
                Doc::nil(),
            ))
        }
    }
}
