use crate::identifier::LowerIdentifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Class<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
    pub inherits: Option<LowerIdentifier<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Toplevel<EXTRA> {
    Class(Class<EXTRA>),
    Definition(Definition<EXTRA>),
    Plan(Plan<EXTRA>),
}
