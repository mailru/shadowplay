pub mod argument;
pub mod expression;
pub mod identifier;
pub mod resource_collection;
pub mod statement;
pub mod string;
pub mod toplevel;
pub mod typing;

pub trait ExtraGetter<EXTRA> {
    fn extra<'a>(&'a self) -> &'a EXTRA;
}
