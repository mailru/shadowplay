use crate::identifier::LowerIdentifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub accessor: Vec<Vec<Box<Expression<EXTRA>>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegexpGroupID<EXTRA> {
    pub identifier: u64,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lambda<EXTRA> {
    pub args: Vec<crate::argument::Argument<EXTRA>>,
    pub body: Vec<crate::statement::Statement<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub args: Vec<Expression<EXTRA>>,
    pub lambda: Option<Lambda<EXTRA>>,
    pub accessor: Vec<Vec<Box<Expression<EXTRA>>>>,
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
    pub value: Box<Term<EXTRA>>,
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
pub struct Boolean<EXTRA> {
    pub value: bool,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parens<EXTRA> {
    pub value: Box<Expression<EXTRA>>,
    pub accessor: Vec<Vec<Box<Expression<EXTRA>>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Map<EXTRA> {
    pub value: Vec<(Expression<EXTRA>, Expression<EXTRA>)>,
    pub accessor: Vec<Vec<Box<Expression<EXTRA>>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TermVariant<EXTRA> {
    String(crate::string::StringExpr<EXTRA>),
    Float(Float<EXTRA>),
    Integer(Integer<EXTRA>),
    Boolean(Boolean<EXTRA>),
    Array(Vec<Expression<EXTRA>>),
    Identifier(LowerIdentifier<EXTRA>),
    Parens(Parens<EXTRA>),
    Map(Map<EXTRA>),
    Undef(Undef<EXTRA>),
    Variable(Variable<EXTRA>),
    RegexpGroupID(RegexpGroupID<EXTRA>),
    Sensitive(Sensitive<EXTRA>),
    TypeSpecitifaction(crate::typing::TypeSpecification<EXTRA>),
    Regexp(Regexp<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Term<EXTRA> {
    pub value: TermVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Default<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CaseVariant<EXTRA> {
    Term(Term<EXTRA>),
    Default(Default<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectorCase<EXTRA> {
    pub case: CaseVariant<EXTRA>,
    pub body: Box<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selector<EXTRA> {
    pub condition: Box<Expression<EXTRA>>,
    pub cases: Vec<SelectorCase<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChainCall<EXTRA> {
    pub left: Box<Expression<EXTRA>>,
    pub right: Box<FunctionCall<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionVariant<EXTRA> {
    Assign((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),

    And((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Or((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),

    Equal((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    NotEqual((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Gt((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    GtEq((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Lt((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    LtEq((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),

    ShiftLeft((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    ShiftRight((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),

    Plus((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Minus((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),

    Multiply((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Divide((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Modulo((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),

    ChainCall(ChainCall<EXTRA>),

    MatchRegex((Box<Expression<EXTRA>>, Regexp<EXTRA>)),
    NotMatchRegex((Box<Expression<EXTRA>>, Regexp<EXTRA>)),
    MatchType(
        (
            Box<Expression<EXTRA>>,
            Box<crate::typing::TypeSpecification<EXTRA>>,
        ),
    ),
    NotMatchType(
        (
            Box<Expression<EXTRA>>,
            Box<crate::typing::TypeSpecification<EXTRA>>,
        ),
    ),
    In((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Not(Box<Expression<EXTRA>>),
    Selector(Selector<EXTRA>),
    FunctionCall(FunctionCall<EXTRA>),
    Term(Term<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression<EXTRA> {
    pub value: ExpressionVariant<EXTRA>,
    pub extra: EXTRA,
}
