pub mod argument;
pub mod builtin;
pub mod comment;
pub mod expression;
pub mod identifier;
pub mod keywords;
pub mod resource_collection;
pub mod statement;
pub mod string;
pub mod toplevel;
pub mod typing;

use serde::Serialize;

pub trait ExtraGetter<EXTRA> {
    fn extra(&self) -> &EXTRA;
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct List<EXTRA, ELT> {
    pub value: Vec<ELT>,
    pub last_comment: Vec<crate::puppet_lang::comment::Comment<EXTRA>>,
}

impl<EXTRA, ELT> Default for List<EXTRA, ELT> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            last_comment: Default::default(),
        }
    }
}
