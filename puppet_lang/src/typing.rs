#[derive(Clone, Debug, PartialEq)]
pub struct TypeFloat<EXTRA> {
    pub min: Option<crate::expression::Float<EXTRA>>,
    pub max: Option<crate::expression::Float<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeInteger<EXTRA> {
    pub min: Option<crate::expression::Integer<EXTRA>>,
    pub max: Option<crate::expression::Integer<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeString<EXTRA> {
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeArray<EXTRA> {
    pub inner: Option<Box<TypeSpecification<EXTRA>>>,
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHash<EXTRA> {
    pub key: Option<Box<TypeSpecification<EXTRA>>>,
    pub value: Option<Box<TypeSpecification<EXTRA>>>,
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeOptionalVariant<EXTRA> {
    TypeSpecification(Box<TypeSpecification<EXTRA>>),
    Term(Box<crate::expression::Term<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeOptional<EXTRA> {
    pub value: TypeOptionalVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSensitiveVariant<EXTRA> {
    TypeSpecification(Box<TypeSpecification<EXTRA>>),
    Term(Box<crate::expression::Term<EXTRA>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeSensitive<EXTRA> {
    pub value: TypeSensitiveVariant<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeStructKey<EXTRA> {
    String(crate::expression::StringExpr<EXTRA>),
    Optional(crate::expression::StringExpr<EXTRA>),
    // TODO
    NotUndef(crate::expression::StringExpr<EXTRA>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeStruct<EXTRA> {
    pub keys: Vec<(TypeStructKey<EXTRA>, TypeSpecification<EXTRA>)>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTuple<EXTRA> {
    pub list: Vec<TypeSpecification<EXTRA>>,
    pub min: Option<crate::expression::Usize<EXTRA>>,
    pub max: Option<crate::expression::Usize<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Numeric<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern<EXTRA> {
    pub list: Vec<crate::expression::Regexp<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Regex<EXTRA> {
    pub data: crate::expression::Regexp<EXTRA>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Boolean<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Undef<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Any<EXTRA> {
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant<EXTRA> {
    pub list: Vec<TypeSpecification<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum<EXTRA> {
    pub list: Vec<crate::expression::Term<EXTRA>>,
    pub extra: EXTRA,
}

// TODO определенные где-то в иных файлах типы
#[derive(Clone, Debug, PartialEq)]
pub struct ExternalType<EXTRA> {
    pub name: Vec<String>,
    pub arguments: Vec<crate::expression::Term<EXTRA>>,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct TypeSpecification<EXTRA> {
    pub data: TypeSpecificationVariant<EXTRA>,
    pub extra: EXTRA,
}
