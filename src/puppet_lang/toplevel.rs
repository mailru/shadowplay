use crate::puppet_lang::identifier::LowerIdentifier;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Class<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: crate::puppet_lang::List<EXTRA, crate::puppet_lang::argument::Argument<EXTRA>>,
    pub inherits: Option<LowerIdentifier<EXTRA>>,
    pub body: crate::puppet_lang::List<EXTRA, crate::puppet_lang::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

impl<EXTRA> Class<EXTRA> {
    pub fn get_argument(
        &self,
        argument_name: &str,
    ) -> Option<&crate::puppet_lang::argument::Argument<EXTRA>> {
        self.arguments
            .value
            .iter()
            .find(|a| a.name == argument_name)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Definition<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: crate::puppet_lang::List<EXTRA, crate::puppet_lang::argument::Argument<EXTRA>>,
    pub body: crate::puppet_lang::List<EXTRA, crate::puppet_lang::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Plan<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: crate::puppet_lang::List<EXTRA, crate::puppet_lang::argument::Argument<EXTRA>>,
    pub body: crate::puppet_lang::List<EXTRA, crate::puppet_lang::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeDef<EXTRA> {
    pub identifier: crate::puppet_lang::identifier::CamelIdentifier<EXTRA>,
    pub value: crate::puppet_lang::typing::TypeSpecification<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunctionDef<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub arguments: crate::puppet_lang::List<EXTRA, crate::puppet_lang::argument::Argument<EXTRA>>,
    pub return_type: Option<crate::puppet_lang::typing::TypeSpecification<EXTRA>>,
    pub body: crate::puppet_lang::List<EXTRA, crate::puppet_lang::statement::Statement<EXTRA>>,
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
    pub data: ToplevelVariant<EXTRA>,
    pub extra: EXTRA,
}
