#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<EXTRA> {
    pub name: String,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionVariant<EXTRA> {
    Equal((Attribute<EXTRA>, crate::expression::Term<EXTRA>)),
    NotEqual((Attribute<EXTRA>, crate::expression::Term<EXTRA>)),
    And((Box<SearchExpression<EXTRA>>, Box<SearchExpression<EXTRA>>)),
    Or((Box<SearchExpression<EXTRA>>, Box<SearchExpression<EXTRA>>)),
    Parens(Box<SearchExpression<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchExpression<EXTRA> {
    pub value: ExpressionVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceCollection<EXTRA> {
    pub type_specification: crate::typing::TypeSpecification<EXTRA>,
    pub search_expression: Option<SearchExpression<EXTRA>>,
    pub extra: EXTRA,
}
