use crate::expression::{Expression, Lambda};

#[derive(Clone, Debug, PartialEq)]
pub struct Many1<EXTRA> {
    pub lambda: Option<Lambda<EXTRA>>,
    pub args: Vec<Expression<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinVariant<EXTRA> {
    Undef,
    Tag(Many1<EXTRA>),
    Require(Many1<EXTRA>),
    Include(Many1<EXTRA>),
    Realize(Many1<EXTRA>),
    CreateResources(Many1<EXTRA>),
}
