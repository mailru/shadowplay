#[derive(Clone, Debug, PartialEq)]
pub struct Argument<EXTRA> {
    pub type_spec: Option<super::typing::TypeSpecification<EXTRA>>,
    pub name: String,
    pub default: Option<super::expression::Expression<EXTRA>>,
    pub extra: EXTRA,
}
