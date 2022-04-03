use pretty::RcDoc;

pub fn multiline_list<ELT, MAP>(list: &[ELT], elt_to_doc: MAP) -> RcDoc<()>
where
    MAP: FnMut(&ELT) -> RcDoc<()>,
{
    if list.len() > 1 {
        return RcDoc::hardline()
            .append(
                RcDoc::intersperse(
                    list.iter().map(elt_to_doc),
                    RcDoc::text(",").append(RcDoc::hardline()),
                )
                .group(),
            )
            .nest(2)
            .append(RcDoc::hardline());
    }
    RcDoc::softline()
        .append(
            RcDoc::intersperse(
                list.iter().map(elt_to_doc),
                RcDoc::text(",").append(RcDoc::softline()),
            )
            .group(),
        )
        .nest(2)
        .append(RcDoc::softline())
}

pub fn multiline_docs_list<'a, T>(list: Vec<T>, multiline: Option<bool>) -> RcDoc<'a, ()>
where
    T: pretty::Pretty<'a, pretty::RcAllocator, ()>,
{
    let multiline = multiline.unwrap_or(list.len() > 1);
    if multiline {
        return RcDoc::hardline()
            .append(
                RcDoc::intersperse(list.into_iter(), RcDoc::text(",").append(RcDoc::hardline()))
                    .group(),
            )
            .nest(2)
            .append(RcDoc::hardline());
    }
    RcDoc::softline()
        .append(
            RcDoc::intersperse(list.into_iter(), RcDoc::text(",").append(RcDoc::softline()))
                .group(),
        )
        .nest(2)
        .append(RcDoc::softline())
}
