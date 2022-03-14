use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Argument<EXTRA> {
    pub type_spec: Option<super::typing::TypeSpecification<EXTRA>>,
    pub name: String,
    pub default: Option<super::expression::Expression<EXTRA>>,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
    pub extra: EXTRA,
}
