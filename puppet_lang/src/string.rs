#[derive(Clone, Debug, PartialEq)]
pub struct Escaped<EXTRA> {
    pub data: char,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal<EXTRA> {
    pub data: String,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression<EXTRA> {
    pub data: crate::expression::Expression<EXTRA>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringFragment<EXTRA> {
    Literal(Literal<EXTRA>),
    EscapedUTF(Escaped<EXTRA>),
    Escaped(Escaped<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DoubleQuotedFragment<EXTRA> {
    StringFragment(StringFragment<EXTRA>),
    Expression(Expression<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringVariant<EXTRA> {
    SingleQuoted(Vec<StringFragment<EXTRA>>),
    DoubleQuoted(Vec<DoubleQuotedFragment<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringExpr<EXTRA> {
    pub data: StringVariant<EXTRA>,
    pub accessor: Option<crate::expression::Accessor<EXTRA>>,
    pub extra: EXTRA,
}
