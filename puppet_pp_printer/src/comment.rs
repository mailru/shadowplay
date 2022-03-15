use crate::Printer;
use pretty::{Doc, RcDoc};

impl<EXTRA> Printer for Vec<puppet_lang::comment::Comment<EXTRA>> {
    fn to_doc(&self) -> RcDoc<()> {
        if self
            .iter()
            .flat_map(|v| v.value.split('\n'))
            .next()
            .is_none()
        {
            RcDoc::nil()
        } else {
            RcDoc::hardline().append(RcDoc::intersperse(
                self.iter().map(|line| RcDoc::text("#").append(&line.value)),
                Doc::hardline(),
            ))
        }
    }
}

pub fn comment_or<'a, EXTRA>(
    comment: &'a Vec<puppet_lang::comment::Comment<EXTRA>>,
    after_comment: RcDoc<'a, ()>,
    alt: RcDoc<'a, ()>,
) -> RcDoc<'a, ()> {
    if comment.is_empty() {
        alt
    } else {
        comment.to_doc().append(after_comment)
    }
}
