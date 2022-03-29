use crate::puppet_lang::identifier::LowerIdentifier;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Accessor<EXTRA> {
    pub list: Vec<Vec<Box<Expression<EXTRA>>>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Variable<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub is_local_scope: bool,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RegexpGroupID<EXTRA> {
    pub identifier: u64,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Lambda<EXTRA> {
    pub args: crate::puppet_lang::List<EXTRA, crate::puppet_lang::argument::Argument<EXTRA>>,
    pub body: crate::puppet_lang::List<EXTRA, crate::puppet_lang::statement::Statement<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FunctionCall<EXTRA> {
    pub identifier: LowerIdentifier<EXTRA>,
    pub args: Vec<Expression<EXTRA>>,
    pub lambda: Option<Lambda<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Float<EXTRA> {
    pub value: f32,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Integer<EXTRA> {
    pub value: i64,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Usize<EXTRA> {
    pub value: usize,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Sensitive<EXTRA> {
    pub value: Box<Term<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Regexp<EXTRA> {
    pub data: String,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Boolean<EXTRA> {
    pub value: bool,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Parens<EXTRA> {
    pub value: Box<Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MapKV<EXTRA> {
    pub key: Expression<EXTRA>,
    pub value: Expression<EXTRA>,
    pub comment: Vec<crate::puppet_lang::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Map<EXTRA> {
    pub value: crate::puppet_lang::List<EXTRA, MapKV<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Array<EXTRA> {
    pub value: crate::puppet_lang::List<EXTRA, Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TermVariant<EXTRA> {
    String(crate::puppet_lang::string::StringExpr<EXTRA>),
    Float(Float<EXTRA>),
    Integer(Integer<EXTRA>),
    Boolean(Boolean<EXTRA>),
    Array(Array<EXTRA>),
    Identifier(LowerIdentifier<EXTRA>),
    Parens(Parens<EXTRA>),
    Map(Map<EXTRA>),
    Variable(Variable<EXTRA>),
    RegexpGroupID(RegexpGroupID<EXTRA>),
    Sensitive(Sensitive<EXTRA>),
    TypeSpecitifaction(crate::puppet_lang::typing::TypeSpecification<EXTRA>),
    Regexp(Regexp<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Term<EXTRA> {
    pub value: TermVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Default<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum CaseVariant<EXTRA> {
    Term(Term<EXTRA>),
    Default(Default<EXTRA>),
}

impl<EXTRA> crate::puppet_lang::ExtraGetter<EXTRA> for CaseVariant<EXTRA> {
    fn extra(&self) -> &EXTRA {
        match self {
            Self::Term(v) => &v.extra,
            Self::Default(v) => &v.extra,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SelectorCase<EXTRA> {
    pub case: CaseVariant<EXTRA>,
    pub body: Box<Expression<EXTRA>>,
    pub extra: EXTRA,
    pub comment: Vec<crate::puppet_lang::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Selector<EXTRA> {
    pub condition: Box<Expression<EXTRA>>,
    pub cases: crate::puppet_lang::List<EXTRA, SelectorCase<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChainCall<EXTRA> {
    pub left: Box<Expression<EXTRA>>,
    pub right: Box<FunctionCall<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
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
            Box<crate::puppet_lang::typing::TypeSpecification<EXTRA>>,
        ),
    ),
    NotMatchType(
        (
            Box<Expression<EXTRA>>,
            Box<crate::puppet_lang::typing::TypeSpecification<EXTRA>>,
        ),
    ),
    In((Box<Expression<EXTRA>>, Box<Expression<EXTRA>>)),
    Not(Box<Expression<EXTRA>>),
    Selector(Selector<EXTRA>),
    FunctionCall(FunctionCall<EXTRA>),
    BuiltinFunction(crate::puppet_lang::builtin::BuiltinVariant<EXTRA>),
    Term(Term<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Expression<EXTRA> {
    pub value: ExpressionVariant<EXTRA>,
    pub extra: EXTRA,
    pub accessor: Option<Accessor<EXTRA>>,
    pub comment: Vec<crate::puppet_lang::comment::Comment<EXTRA>>,
}
