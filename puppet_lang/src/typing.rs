use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeFloat<EXTRA> {
    pub min: Option<crate::expression::Float<EXTRA>>,
    pub max: Option<crate::expression::Float<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeInteger<EXTRA> {
    pub min: Option<crate::expression::Integer<EXTRA>>,
    pub max: Option<crate::expression::Integer<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeString<EXTRA> {
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeArray<EXTRA> {
    pub inner: Option<Box<TypeSpecification<EXTRA>>>,
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeHash<EXTRA> {
    pub key: Option<Box<TypeSpecification<EXTRA>>>,
    pub value: Option<Box<TypeSpecification<EXTRA>>>,
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TypeOptionalVariant<EXTRA> {
    TypeSpecification(Box<TypeSpecification<EXTRA>>),
    Term(Box<crate::expression::Term<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeOptional<EXTRA> {
    pub value: TypeOptionalVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TypeSensitiveVariant<EXTRA> {
    TypeSpecification(Box<TypeSpecification<EXTRA>>),
    Term(Box<crate::expression::Term<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeSensitive<EXTRA> {
    pub value: TypeSensitiveVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OptionalStructKey<EXTRA> {
    pub value: crate::string::StringExpr<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TypeStructKey<EXTRA> {
    String(crate::string::StringExpr<EXTRA>),
    Optional(OptionalStructKey<EXTRA>),
    // TODO
    NotUndef(crate::string::StringExpr<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeStructKV<EXTRA> {
    pub key: TypeStructKey<EXTRA>,
    pub value: TypeSpecification<EXTRA>,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeStruct<EXTRA> {
    pub keys: crate::List<EXTRA, TypeStructKV<EXTRA>>,
    pub extra: EXTRA,
    pub comment_before_keys: Vec<crate::comment::Comment<EXTRA>>,
    pub comment_after_keys: Vec<crate::comment::Comment<EXTRA>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeTuple<EXTRA> {
    pub list: Vec<TypeSpecification<EXTRA>>,
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Numeric<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Pattern<EXTRA> {
    pub list: Vec<crate::expression::Regexp<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Regex<EXTRA> {
    pub data: crate::expression::Regexp<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Boolean<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Undef<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Any<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Variant<EXTRA> {
    pub list: Vec<TypeSpecification<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Enum<EXTRA> {
    pub list: Vec<crate::expression::Term<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExternalType<EXTRA> {
    pub name: Vec<String>,
    pub arguments: Vec<crate::expression::Expression<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TypeSpecificationVariant<EXTRA> {
    Float(TypeFloat<EXTRA>),
    Integer(TypeInteger<EXTRA>),
    Numeric(Numeric<EXTRA>),
    String(TypeString<EXTRA>),
    Pattern(Pattern<EXTRA>),
    Regex(Regex<EXTRA>),
    Hash(TypeHash<EXTRA>),
    Boolean(Boolean<EXTRA>),
    Array(TypeArray<EXTRA>),
    Undef(Undef<EXTRA>),
    Any(Any<EXTRA>),
    Optional(TypeOptional<EXTRA>),
    Variant(Variant<EXTRA>),
    Enum(Enum<EXTRA>),
    Struct(TypeStruct<EXTRA>),
    ExternalType(ExternalType<EXTRA>),
    Sensitive(TypeSensitive<EXTRA>),
    Tuple(TypeTuple<EXTRA>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TypeSpecification<EXTRA> {
    pub data: TypeSpecificationVariant<EXTRA>,
    pub extra: EXTRA,
    pub comment: Vec<crate::comment::Comment<EXTRA>>,
}
