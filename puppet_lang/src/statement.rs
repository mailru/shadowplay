use crate::{expression::Expression, identifier::LowerIdentifier};

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceAttribute<EXTRA> {
    Name((crate::string::StringExpr<EXTRA>, Expression<EXTRA>)),
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
    ExecOrderRight,
    NotifyRight,
    ExecOrderLeft,
    NotifyLeft,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelationType<EXTRA> {
    pub variant: RelationVariant,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelationElt<EXTRA> {
    ResourceSet(ResourceSet<EXTRA>),
    ResourceCollection(Vec<crate::resource_collection::ResourceCollection<EXTRA>>),
}

impl<EXTRA> crate::ExtraGetter<EXTRA> for RelationElt<EXTRA> {
    fn extra(&self) -> &EXTRA {
        match self {
            RelationElt::ResourceSet(v) => &v.extra,
            // FIXME Currently parser guarantees at least one element exists in the list
            RelationElt::ResourceCollection(v) => &v.first().unwrap().extra,
        }
    }
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
    pub extra: EXTRA,
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
    pub resource: crate::expression::Term<EXTRA>,
    pub args: Vec<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinFunction<EXTRA> {
    pub name: String,
    pub args: Vec<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementVariant<EXTRA> {
    BuiltinFunction(BuiltinFunction<EXTRA>),
    CreateResources(CreateResources<EXTRA>),
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
}

impl<EXTRA> crate::ExtraGetter<EXTRA> for Statement<EXTRA> {
    fn extra(&self) -> &EXTRA {
        match &self.value {
            StatementVariant::BuiltinFunction(v) => &v.extra,
            StatementVariant::CreateResources(v) => &v.extra,
            StatementVariant::Expression(v) => &v.extra,
            StatementVariant::RelationList(v) => &v.extra,
            StatementVariant::IfElse(v) => &v.extra,
            StatementVariant::Unless(v) => &v.extra,
            StatementVariant::Case(v) => &v.extra,
            StatementVariant::Toplevel(v) => v.extra(),
        }
    }
}
