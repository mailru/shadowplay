pub mod accessor;
pub mod argument;
pub mod comment;
pub mod expression;
pub mod identifier;
pub mod statement;
pub mod string;
pub mod term;
pub mod toplevel;
pub mod typing;

use pretty::RcDoc;

pub trait Printer {
    fn to_doc(&self) -> RcDoc<()>;
}
