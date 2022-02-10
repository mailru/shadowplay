use crate::identifier::LowerIdentifier;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementVariant<EXTRA> {
    Include(LowerIdentifier<EXTRA>),
    Require(LowerIdentifier<EXTRA>),
    Contain(LowerIdentifier<EXTRA>),
    // TODO parser
    Tag(Vec<crate::expression::StringExpr<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<EXTRA> {
    pub value: StatementVariant<EXTRA>,
    pub extra: EXTRA,
}
