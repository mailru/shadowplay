pub mod accessor;
pub mod comment;
pub mod expression;
pub mod identifier;
pub mod string;
pub mod term;
pub mod typing;

use pretty::RcDoc;

pub trait Printer {
    fn to_doc(&self) -> RcDoc<()>;
}
