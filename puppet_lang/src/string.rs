#[derive(Clone, Debug, PartialEq)]
pub struct Escaped<EXTRA> {
    pub data: char,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringFragment<EXTRA> {
    Literal(String),
    EscapedUTF(Escaped<EXTRA>),
    Escaped(Escaped<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DoubleQuotedFragment<EXTRA> {
    StringFragment(StringFragment<EXTRA>),
    Expression(crate::expression::Expression<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum StringVariant<EXTRA> {
    SingleQuoted(Vec<StringFragment<EXTRA>>),
    DoubleQuoted(Vec<DoubleQuotedFragment<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringExpr<EXTRA> {
    pub data: StringVariant<EXTRA>,
    pub accessor: Vec<Vec<Box<crate::expression::Expression<EXTRA>>>>,
    pub extra: EXTRA,
}
