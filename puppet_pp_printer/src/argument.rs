use crate::Printer;
use pretty::RcDoc;

impl<EXTRA> Printer for puppet_lang::argument::Argument<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let type_spec = match &self.type_spec {
            Some(v) => v.to_doc().append(RcDoc::softline()),
            None => RcDoc::nil(),
        };

        let default = match &self.default {
            Some(v) => RcDoc::softline_()
                .append(RcDoc::column(|w| {
                    let offset = (w / 30 + 1) * 30;
                    RcDoc::text(format!("{} =", " ".repeat(offset - w)))
                }))
                .append(RcDoc::softline())
                .append(crate::expression::to_doc(v, false)),
            None => RcDoc::nil(),
        };

        crate::comment::comment_or(&self.comment, RcDoc::hardline(), RcDoc::nil())
            .append(type_spec)
            .append(RcDoc::text("$"))
            .append(RcDoc::text(&self.name))
            .append(default)
    }
}

impl<EXTRA> Printer for puppet_lang::List<EXTRA, puppet_lang::argument::Argument<EXTRA>> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::intersperse(
            self.value
                .iter()
                .map(|x| x.to_doc().append(RcDoc::text(","))),
            RcDoc::hardline(),
        )
        .append(self.last_comment.to_doc())
    }
}

pub fn list_to_rounded_doc<EXTRA>(
    elt: &puppet_lang::List<EXTRA, puppet_lang::argument::Argument<EXTRA>>,
) -> RcDoc<()> {
    if elt.value.is_empty() && elt.last_comment.is_empty() {
        return RcDoc::text("()");
    }

    RcDoc::text("(")
        .append(RcDoc::hardline())
        .append(elt.to_doc())
        .nest(2)
        .append(RcDoc::hardline())
        .append(RcDoc::text(")"))
}

pub fn list_to_piped_doc<EXTRA>(
    elt: &puppet_lang::List<EXTRA, puppet_lang::argument::Argument<EXTRA>>,
) -> RcDoc<()> {
    if elt.value.is_empty() && elt.last_comment.is_empty() {
        return RcDoc::text("||");
    }

    let list = RcDoc::intersperse(
        elt.value.iter().map(|x| x.to_doc()),
        RcDoc::text(",").append(RcDoc::softline()),
    )
    .append(elt.last_comment.to_doc());

    RcDoc::text("|")
        .append(RcDoc::softline_())
        .append(list)
        .nest(2)
        .append(RcDoc::softline_())
        .append(RcDoc::text("|"))
}