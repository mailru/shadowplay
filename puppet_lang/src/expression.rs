use crate::identifier::LowerIdentifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub accessor: Vec<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub args: Vec<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Float<EXTRA> {
    pub value: f32,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Integer<EXTRA> {
    pub value: i64,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Usize<EXTRA> {
    pub value: usize,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sensitive<EXTRA> {
    pub value: StringExpr<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Undef<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Regexp<EXTRA> {
    pub data: String,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringVariant {
    SingleQuoted,
    DoubleQuoted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringExpr<EXTRA> {
    pub data: String,
    pub variant: StringVariant,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean<EXTRA> {
    pub value: bool,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TermVariant<EXTRA> {
    String(StringExpr<EXTRA>),
    Float(Float<EXTRA>),
    Integer(Integer<EXTRA>),
    Boolean(Boolean<EXTRA>),
    Array(Vec<Expression<EXTRA>>),
    Map(Vec<(Expression<EXTRA>, Expression<EXTRA>)>),
    Undef(Undef<EXTRA>),
    Variable(Variable<EXTRA>),
    FunctionCall(FunctionCall<EXTRA>),
    Sensitive(Sensitive<EXTRA>),
    TypeSpecitifaction(crate::typing::TypeSpecification<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Term<EXTRA> {
    pub value: TermVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionVariant<EXTRA> {
    Multiply((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Divide((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Modulo((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Plus((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Minus((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Term(Term<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression<EXTRA> {
    pub value: ExpressionVariant<EXTRA>,
    pub extra: EXTRA,
}
