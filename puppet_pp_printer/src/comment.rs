use pretty::{Doc, RcDoc};

pub fn to_doc<EXTRA>(comment: &[puppet_lang::comment::Comment<EXTRA>]) -> RcDoc<()> {
    if comment
        .iter()
        .flat_map(|v| v.value.split('\n'))
        .next()
        .is_none()
    {
        RcDoc::nil()
    } else {
        RcDoc::hardline().append(RcDoc::intersperse(
            comment
                .iter()
                .map(|line| RcDoc::text("#").append(&line.value)),
            Doc::hardline(),
        ))
    }
}

pub fn comment_or<'a, EXTRA>(
    comment: &'a [puppet_lang::comment::Comment<EXTRA>],
    after_comment: RcDoc<'a, ()>,
    alt: RcDoc<'a, ()>,
) -> RcDoc<'a, ()> {
    if comment.is_empty() {
        alt
    } else {
        to_doc(comment).append(after_comment)
    }
}
