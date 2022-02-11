use crate::{expression::Expression, identifier::LowerIdentifier};

#[derive(Clone, Debug, PartialEq)]
pub struct Resource<EXTRA> {
    pub title: Expression<EXTRA>,
    pub arguments: Vec<(crate::expression::StringExpr<EXTRA>, Expression<EXTRA>)>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceSet<EXTRA> {
    pub name: LowerIdentifier<EXTRA>,
    pub list: Vec<Resource<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionAndStatement<EXTRA> {
    pub condition: Expression<EXTRA>,
    pub body: Box<Vec<Statement<EXTRA>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfElse<EXTRA> {
    pub if_block: ConditionAndStatement<EXTRA>,
    pub elsif_list: Vec<ConditionAndStatement<EXTRA>>,
    pub else_block: Option<Box<Vec<Statement<EXTRA>>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementVariant<EXTRA> {
    Include(LowerIdentifier<EXTRA>),
    Require(LowerIdentifier<EXTRA>),
    Contain(LowerIdentifier<EXTRA>),
    Tag(Vec<crate::expression::StringExpr<EXTRA>>),
    Expression(crate::expression::Expression<EXTRA>),
    ResourceTypeRelation(Vec<crate::typing::TypeSpecification<EXTRA>>),
    ResourceSet(ResourceSet<EXTRA>),
    ResourceSetRelation(Vec<ResourceSet<EXTRA>>),
    IfElse(IfElse<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<EXTRA> {
    pub value: StatementVariant<EXTRA>,
    pub extra: EXTRA,
}
