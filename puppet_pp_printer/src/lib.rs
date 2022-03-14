pub mod comment;
pub mod expression;
pub mod term;
pub mod identifier;
pub mod accessor;

use pretty::RcDoc;

pub trait Printer {
    fn to_doc(&self) -> RcDoc<()>;
}
