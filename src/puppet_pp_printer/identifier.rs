use crate::puppet_pp_printer::Printer;
use pretty::{Doc, RcDoc};

impl<EXTRA> Printer for crate::puppet_lang::identifier::CamelIdentifier<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        RcDoc::intersperse(self.name.iter(), Doc::text("::"))
    }
}

impl<EXTRA> Printer for crate::puppet_lang::identifier::LowerIdentifier<EXTRA> {
    fn to_doc(&self) -> RcDoc<()> {
        let head = if self.is_toplevel {
            RcDoc::text("::")
        } else {
            RcDoc::nil()
        };
        head.append(RcDoc::intersperse(self.name.iter(), Doc::text("::")))
    }
}
