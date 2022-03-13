use crate::identifier::LowerIdentifier;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Class<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
    pub inherits: Option<LowerIdentifier<EXTRA>>,
    pub body: Vec<crate::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

impl<EXTRA> Class<EXTRA> {
    pub fn get_argument(&self, argument_name: &str) -> Option<&crate::argument::Argument<EXTRA>> {
        self.arguments.iter().find(|a| a.name == argument_name)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Definition<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
    pub body: Vec<crate::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Plan<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
    pub body: Vec<crate::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeDef<EXTRA> {
    pub identifier: crate::identifier::CamelIdentifier<EXTRA>,
    pub value: crate::typing::TypeSpecification<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunctionDef<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: Vec<crate::argument::Argument<EXTRA>>,
    pub return_type: Option<crate::typing::TypeSpecification<EXTRA>>,
    pub body: Vec<crate::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ToplevelVariant<EXTRA> {
    Class(Class<EXTRA>),
    Definition(Definition<EXTRA>),
    Plan(Plan<EXTRA>),
    TypeDef(TypeDef<EXTRA>),
    FunctionDef(FunctionDef<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Toplevel<EXTRA> {
    pub comment: Option<Vec<String>>,
    pub data: ToplevelVariant<EXTRA>,
    pub extra: EXTRA,
}
