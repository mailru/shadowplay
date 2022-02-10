use crate::identifier::LowerIdentifier;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementVariant<EXTRA> {
    Include(LowerIdentifier<EXTRA>),
    Require(LowerIdentifier<EXTRA>),
    Contain(LowerIdentifier<EXTRA>),
    Tag(Vec<crate::expression::StringExpr<EXTRA>>),
    Expression(crate::expression::Expression<EXTRA>),
    ResourceTypeRelation(Vec<crate::typing::TypeSpecification<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<EXTRA> {
    pub value: StatementVariant<EXTRA>,
    pub extra: EXTRA,
}
