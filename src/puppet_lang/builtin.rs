use crate::puppet_lang::expression::{Expression, Lambda};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Many1<EXTRA> {
    pub lambda: Option<Lambda<EXTRA>>,
    pub args: Vec<Expression<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BuiltinVariant<EXTRA> {
    Undef,
    Return(Box<Option<Expression<EXTRA>>>),
    Template(Many1<EXTRA>),
    Tag(Many1<EXTRA>),
    Require(Many1<EXTRA>),
    Include(Many1<EXTRA>),
    Realize(Many1<EXTRA>),
    CreateResources(Many1<EXTRA>),
}
