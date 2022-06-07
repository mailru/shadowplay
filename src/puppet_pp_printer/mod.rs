pub mod accessor;
pub mod argument;
pub mod comment;
pub mod common;
pub mod expression;
pub mod identifier;
pub mod resource;
pub mod statement;
pub mod string;
pub mod term;
pub mod toplevel;
pub mod typing;

pub const ARROW_STEP: usize = 1;

use pretty::RcDoc;

pub trait Printer {
    fn to_doc(&self) -> RcDoc<()>;
}
