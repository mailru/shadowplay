use crate::{expression::Expression, identifier::LowerIdentifier};

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceAttribute<EXTRA> {
    Name((crate::expression::StringExpr<EXTRA>, Expression<EXTRA>)),
    Group(crate::expression::Term<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resource<EXTRA> {
    pub title: Expression<EXTRA>,
    pub attributes: Vec<ResourceAttribute<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceSet<EXTRA> {
    pub name: LowerIdentifier<EXTRA>,
    pub list: Vec<Resource<EXTRA>>,
    pub is_virtual: bool,
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
    pub condition: ConditionAndStatement<EXTRA>,
    pub elsif_list: Vec<ConditionAndStatement<EXTRA>>,
    pub else_block: Option<Box<Vec<Statement<EXTRA>>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelationVariant {
    ExecOrder,
    Notify,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationType<EXTRA> {
    pub variant: RelationVariant,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelationElt<EXTRA> {
    ResourceSet(ResourceSet<EXTRA>),
    ResourceCollection(crate::resource_collection::ResourceCollection<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Relation<EXTRA> {
    pub relation_type: RelationType<EXTRA>,
    pub relation_to: Box<RelationList<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationList<EXTRA> {
    pub head: RelationElt<EXTRA>,
    pub tail: Option<Relation<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaseElement<EXTRA> {
    pub matches: Vec<crate::expression::CaseVariant<EXTRA>>,
    pub body: Box<Vec<Statement<EXTRA>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Case<EXTRA> {
    pub condition: Expression<EXTRA>,
    pub elements: Vec<CaseElement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateResources<EXTRA> {
    pub resource: LowerIdentifier<EXTRA>,
    pub args: Vec<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementVariant<EXTRA> {
    Include(LowerIdentifier<EXTRA>),
    Require(LowerIdentifier<EXTRA>),
    Contain(LowerIdentifier<EXTRA>),
    Realize(Vec<crate::typing::TypeSpecification<EXTRA>>),
    CreateResources(CreateResources<EXTRA>),
    Tag(Vec<crate::expression::StringExpr<EXTRA>>),
    Expression(crate::expression::Expression<EXTRA>),
    RelationList(RelationList<EXTRA>),
    IfElse(IfElse<EXTRA>),
    Unless(ConditionAndStatement<EXTRA>),
    Case(Case<EXTRA>),
    Toplevel(crate::toplevel::Toplevel<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<EXTRA> {
    pub value: StatementVariant<EXTRA>,
    pub extra: EXTRA,
}
