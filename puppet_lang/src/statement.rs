use crate::{expression::Expression, identifier::LowerIdentifier};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ResourceAttributeVariant<EXTRA> {
    Name((crate::string::StringExpr<EXTRA>, Expression<EXTRA>)),
    Group(crate::expression::Term<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceAttribute<EXTRA> {
    pub value: ResourceAttributeVariant<EXTRA>,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Resource<EXTRA> {
    pub title: Expression<EXTRA>,
    pub attributes: crate::List<EXTRA, ResourceAttribute<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceSet<EXTRA> {
    pub name: LowerIdentifier<EXTRA>,
    pub list: crate::List<EXTRA, Resource<EXTRA>>,
    pub is_virtual: bool,
    pub extra: EXTRA,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConditionAndStatement<EXTRA> {
    pub condition: Expression<EXTRA>,
    pub comment_before_elsif_word: Vec<crate::comment::Comment<EXTRA>>,
    pub comment_before_body: Vec<crate::comment::Comment<EXTRA>>,
    pub body: Box<crate::List<EXTRA, Statement<EXTRA>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IfElse<EXTRA> {
    pub condition: ConditionAndStatement<EXTRA>,
    pub elsif_list: Vec<ConditionAndStatement<EXTRA>>,
    pub else_block: Option<Box<crate::List<EXTRA, Statement<EXTRA>>>>,
    pub comment_before_else_word: Vec<crate::comment::Comment<EXTRA>>,
    pub comment_before_else_body: Vec<crate::comment::Comment<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum RelationVariant {
    ExecOrderRight,
    NotifyRight,
    ExecOrderLeft,
    NotifyLeft,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationType<EXTRA> {
    pub variant: RelationVariant,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum RelationEltVariant<EXTRA> {
    ResourceSet(ResourceSet<EXTRA>),
    ResourceCollection(crate::resource_collection::ResourceCollection<EXTRA>),
}

impl<EXTRA> crate::ExtraGetter<EXTRA> for RelationEltVariant<EXTRA> {
    fn extra(&self) -> &EXTRA {
        match &self {
            RelationEltVariant::ResourceSet(v) => &v.extra,
            RelationEltVariant::ResourceCollection(v) => &v.extra,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationElt<EXTRA> {
    pub data: crate::List<EXTRA, RelationEltVariant<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Relation<EXTRA> {
    pub relation_type: RelationType<EXTRA>,
    pub relation_to: Box<RelationList<EXTRA>>,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RelationList<EXTRA> {
    pub head: RelationElt<EXTRA>,
    pub tail: Option<Relation<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseElement<EXTRA> {
    pub matches: Vec<crate::expression::CaseVariant<EXTRA>>,
    pub body: Box<crate::List<EXTRA, Statement<EXTRA>>>,
    pub extra: EXTRA,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Case<EXTRA> {
    pub condition: Expression<EXTRA>,
    pub elements: crate::List<EXTRA, CaseElement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResourceDefaults<EXTRA> {
    pub name: String,
    pub args: crate::List<EXTRA, (crate::expression::Term<EXTRA>, Expression<EXTRA>)>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum StatementVariant<EXTRA> {
    Expression(crate::expression::Expression<EXTRA>),
    RelationList(RelationList<EXTRA>),
    IfElse(IfElse<EXTRA>),
    Unless(ConditionAndStatement<EXTRA>),
    Case(Case<EXTRA>),
    Toplevel(crate::toplevel::Toplevel<EXTRA>),
    ResourceDefaults(ResourceDefaults<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Statement<EXTRA> {
    pub value: StatementVariant<EXTRA>,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}

impl<EXTRA> crate::ExtraGetter<EXTRA> for Statement<EXTRA> {
    fn extra(&self) -> &EXTRA {
        match &self.value {
            StatementVariant::Expression(v) => &v.extra,
            StatementVariant::RelationList(v) => &v.extra,
            StatementVariant::IfElse(v) => &v.extra,
            StatementVariant::Unless(v) => &v.extra,
            StatementVariant::Case(v) => &v.extra,
            StatementVariant::Toplevel(v) => &v.extra,
            StatementVariant::ResourceDefaults(v) => &v.extra,
        }
    }
}
