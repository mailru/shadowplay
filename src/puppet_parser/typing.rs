use std::primitive;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    error::FromExternalError,
    multi::separated_list0,
    number::complete::float,
    sequence::{pair, preceded, tuple},
    IResult, Parser,
};

pub fn parse_or_default<'a, O, F, E>(
    parser: F,
    default: O,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    O: Clone,
    E: nom::error::ParseError<&'a str>,
{
    alt((parser, value(default, tag("default"))))
}

fn parse_min_max<'a, O, F, E>(
    parser: F,
    default_min: O,
    default_max: O,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O), E>
where
    F: Parser<&'a str, O, E> + Copy,
    O: Clone + Copy,
    E: nom::error::ParseError<&'a str>,
{
    let parser = pair(
        super::common::space0_delimimited(parse_or_default(parser, default_min)),
        opt(super::common::space0_delimimited(preceded(
            super::common::comma_separator,
            parse_or_default(parser, default_max),
        ))),
    );

    map(parser, move |(min, max)| (min, max.unwrap_or(default_max)))
}

fn parse_min_max_args<'a, O, F, E>(
    parser: F,
    default_min: O,
    default_max: O,
) -> impl FnMut(&'a str) -> IResult<&'a str, (O, O), E>
where
    F: Parser<&'a str, O, E> + Copy,
    O: Clone + Copy,
    E: nom::error::ParseError<&'a str>,
{
    map(
        opt(super::common::square_brackets_delimimited(parse_min_max(
            parser,
            default_min,
            default_max,
        ))),
        move |args| args.unwrap_or((default_min, default_max)),
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeFloat {
    pub min: f32,
    pub max: f32,
}

impl TypeFloat {
    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>,
    {
        let parser = preceded(
            tag("Float"),
            parse_min_max_args(float, primitive::f32::MIN, primitive::f32::MAX),
        );

        map(parser, |(min, max)| Self { min, max })(input)
    }
}

#[test]
fn test_float() {
    assert_eq!(
        TypeFloat::parse::<nom::error::Error<_>>("Float").unwrap(),
        (
            "",
            TypeFloat {
                min: primitive::f32::MIN,
                max: primitive::f32::MAX
            }
        )
    );
    assert_eq!(
        TypeFloat::parse::<nom::error::Error<_>>("Float[ 100 ]").unwrap(),
        (
            "",
            TypeFloat {
                min: 100.,
                max: primitive::f32::MAX
            }
        )
    );
    assert_eq!(
        TypeFloat::parse::<nom::error::Error<_>>("Float[ 100,1000]").unwrap(),
        (
            "",
            TypeFloat {
                min: 100.,
                max: 1000.,
            }
        )
    );
    assert!(TypeFloat::parse::<nom::error::Error<_>>("Float[ 100,  1000, 10]").is_ok());
    assert!(TypeFloat::parse::<nom::error::Error<_>>("Float[]").is_ok())
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeInteger {
    pub min: i64,
    pub max: i64,
}

impl TypeInteger {
    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>,
    {
        let parser = preceded(
            tag("Integer"),
            parse_min_max_args(
                nom::character::complete::i64,
                primitive::i64::MIN,
                primitive::i64::MAX,
            ),
        );

        map(parser, |(min, max)| Self { min, max })(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeString {
    pub min: usize,
    pub max: usize,
}

impl TypeString {
    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>,
    {
        let parser = preceded(
            tag("String"),
            parse_min_max_args(
                nom::character::complete::u64,
                primitive::u64::MIN,
                primitive::u64::MAX,
            ),
        );

        map(parser, |(min, max)| Self {
            min: min as usize,
            max: max as usize,
        })(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeArray<'a> {
    pub inner: Option<Box<TypeSpecification<'a>>>,
    pub min: usize,
    pub max: usize,
}

impl<'a> TypeArray<'a> {
    fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let parser = pair(
            TypeSpecification::parse,
            opt(preceded(
                super::common::comma_separator,
                parse_min_max(
                    nom::character::complete::u64,
                    primitive::u64::MIN,
                    primitive::u64::MAX,
                ),
            )),
        );
        let parser = map(parser, |(inner, limits)| {
            let (min, max) = limits.unwrap_or((primitive::u64::MIN, primitive::u64::MAX));
            Self {
                inner: Some(Box::new(inner)),
                min: min as usize,
                max: max as usize,
            }
        });
        preceded(
            tag("Array"),
            map(
                opt(super::common::square_brackets_delimimited(parser)),
                |v| {
                    v.unwrap_or(Self {
                        inner: None,
                        min: primitive::u64::MIN as usize,
                        max: primitive::u64::MAX as usize,
                    })
                },
            ),
        )(input)
    }
}

#[test]
fn test_array() {
    assert_eq!(
        TypeArray::parse::<nom::error::Error<_>>("Array [String[1,2 ],10]").unwrap(),
        (
            "",
            TypeArray {
                inner: Some(Box::new(TypeSpecification::String(TypeString {
                    min: 1,
                    max: 2
                }))),
                min: 10,
                max: primitive::u64::MAX as usize
            }
        )
    );
    assert_eq!(
        TypeArray::parse::<nom::error::Error<_>>("Array").unwrap(),
        (
            "",
            TypeArray {
                inner: None,
                min: primitive::u64::MIN as usize,
                max: primitive::u64::MAX as usize
            }
        )
    );
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHash<'a> {
    pub key: Option<Box<TypeSpecification<'a>>>,
    pub value: Option<Box<TypeSpecification<'a>>>,
    pub min: usize,
    pub max: usize,
}

impl<'a> TypeHash<'a> {
    fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let parser = tuple((
            TypeSpecification::parse,
            preceded(super::common::comma_separator, TypeSpecification::parse),
            opt(preceded(
                super::common::comma_separator,
                parse_min_max(
                    nom::character::complete::u64,
                    primitive::u64::MIN,
                    primitive::u64::MAX,
                ),
            )),
        ));
        let parser = map(parser, |(key, value, limits)| {
            let (min, max) = limits.unwrap_or((primitive::u64::MIN, primitive::u64::MAX));
            Self {
                key: Some(Box::new(key)),
                value: Some(Box::new(value)),
                min: min as usize,
                max: max as usize,
            }
        });
        preceded(
            tag("Hash"),
            map(
                opt(super::common::square_brackets_delimimited(parser)),
                |v| {
                    v.unwrap_or(Self {
                        key: None,
                        value: None,
                        min: primitive::u64::MIN as usize,
                        max: primitive::u64::MAX as usize,
                    })
                },
            ),
        )(input)
    }
}

#[test]
fn test_hash() {
    assert_eq!(
        TypeHash::parse::<nom::error::Error<_>>("Hash [String[1,2 ], Boolean]").unwrap(),
        (
            "",
            TypeHash {
                key: Some(Box::new(TypeSpecification::String(TypeString {
                    min: 1,
                    max: 2
                }))),
                value: Some(Box::new(TypeSpecification::Boolean)),
                min: primitive::u64::MIN as usize,
                max: primitive::u64::MAX as usize
            }
        )
    );
    assert!(TypeHash::parse::<nom::error::Error<_>>("Hash[String, Hash[ String, String]]]").is_ok())
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeOptional<'a> {
    TypeSpecification(Box<TypeSpecification<'a>>),
    Term(Box<super::expression::Term<'a>>),
}

impl<'a> TypeOptional<'a> {
    fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        preceded(
            tag("Optional"),
            super::common::square_brackets_delimimited(alt((
                map(TypeSpecification::parse, |v| {
                    Self::TypeSpecification(Box::new(v))
                }),
                map(super::expression::Term::parse, |v| Self::Term(Box::new(v))),
            ))),
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSensitive<'a> {
    TypeSpecification(Box<TypeSpecification<'a>>),
    Term(Box<super::expression::Term<'a>>),
}

impl<'a> TypeSensitive<'a> {
    fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        preceded(
            tag("Sensitive"),
            super::common::square_brackets_delimimited(alt((
                map(TypeSpecification::parse, |v| {
                    Self::TypeSpecification(Box::new(v))
                }),
                map(super::expression::Term::parse, |v| Self::Term(Box::new(v))),
            ))),
        )(input)
    }
}

#[test]
fn test_optional() {
    assert_eq!(
        TypeOptional::parse::<nom::error::Error<_>>("Optional [String[1,2 ] ]").unwrap(),
        (
            "",
            TypeOptional::TypeSpecification(Box::new(TypeSpecification::String(TypeString {
                min: 1,
                max: 2
            })))
        )
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeStructKey {
    SingleQuoted(String),
    DoubleQuoted(String),
    Optional(String),
}

impl TypeStructKey {
    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let inner_parse = super::common::square_brackets_delimimited(alt((
            super::double_quoted::parse,
            super::single_quoted::parse,
        )));

        alt((
            preceded(tag("Optional"), map(inner_parse, Self::Optional)),
            map(super::double_quoted::parse, Self::DoubleQuoted),
            map(super::single_quoted::parse, Self::SingleQuoted),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeStruct<'a> {
    pub keys: Vec<(TypeStructKey, TypeSpecification<'a>)>,
}

impl<'a> TypeStruct<'a> {
    fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let kv_parser = pair(
            super::common::space0_delimimited(TypeStructKey::parse),
            preceded(
                tag("=>"),
                super::common::space0_delimimited(TypeSpecification::parse),
            ),
        );

        preceded(
            tag("Struct"),
            map(
                super::common::square_brackets_delimimited(
                    super::common::curly_brackets_comma_separated0(kv_parser),
                ),
                |keys| Self { keys },
            ),
        )(input)
    }
}

#[test]
fn test_struct() {
    assert_eq!(
        TypeStruct::parse::<nom::error::Error<_>>("Struct [{some_key => Boolean } ]").unwrap(),
        (
            "",
            TypeStruct {
                keys: vec![(
                    TypeStructKey::SingleQuoted("some_key".to_owned()),
                    TypeSpecification::Boolean
                )]
            }
        )
    );
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTuple<'a> {
    pub list: Vec<TypeSpecification<'a>>,
    pub min: usize,
    pub max: usize,
}

impl<'a> TypeTuple<'a> {
    pub fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let parser = preceded(
            tag("Tuple"),
            super::common::square_brackets_delimimited(pair(
                separated_list0(super::common::comma_separator, TypeSpecification::parse),
                opt(preceded(
                    super::common::comma_separator,
                    parse_min_max(
                        nom::character::complete::u64,
                        primitive::u64::MIN,
                        primitive::u64::MAX,
                    ),
                )),
            )),
        );

        map(parser, move |(list, min_max)| {
            let (min, max) = min_max.unwrap_or((primitive::u64::MIN, primitive::u64::MAX));
            Self {
                list,
                min: min as usize,
                max: max as usize,
            }
        })(input)
    }
}

#[test]
fn test_tuple() {
    assert_eq!(
        TypeTuple::parse::<nom::error::Error<_>>("Tuple [Integer[1,2], 10, 100 ]").unwrap(),
        (
            "",
            TypeTuple {
                list: vec![TypeSpecification::Integer(TypeInteger { min: 1, max: 2 })],
                min: 10,
                max: 100,
            }
        )
    );
    assert_eq!(
        TypeTuple::parse::<nom::error::Error<_>>("Tuple [Integer[1,2] ]").unwrap(),
        (
            "",
            TypeTuple {
                list: vec![TypeSpecification::Integer(TypeInteger { min: 1, max: 2 })],
                min: primitive::u64::MIN as usize,
                max: primitive::u64::MAX as usize,
            }
        )
    );
    assert_eq!(
        TypeTuple::parse::<nom::error::Error<_>>("Tuple [Integer[1,2], Integer[1,2] ]").unwrap(),
        (
            "",
            TypeTuple {
                list: vec![
                    TypeSpecification::Integer(TypeInteger { min: 1, max: 2 }),
                    TypeSpecification::Integer(TypeInteger { min: 1, max: 2 })
                ],
                min: primitive::u64::MIN as usize,
                max: primitive::u64::MAX as usize,
            }
        )
    );
    assert!(TypeTuple::parse::<nom::error::Error<_>>("Tuple").is_err());
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpecification<'a> {
    Float(TypeFloat),
    Integer(TypeInteger),
    Numeric,
    String(TypeString),
    Pattern(Vec<String>),
    Regex(String),
    Hash(TypeHash<'a>),
    Boolean,
    Array(TypeArray<'a>),
    Undef,
    Any,
    Optional(TypeOptional<'a>),
    Variant(Vec<TypeSpecification<'a>>),
    Enum(Vec<super::expression::Term<'a>>),
    Struct(TypeStruct<'a>),
    Custom(Vec<&'a str>),
    Sensitive(TypeSensitive<'a>),
    Tuple(TypeTuple<'a>),
}

impl<'a> TypeSpecification<'a> {
    pub fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let variant_parser = preceded(
            tag("Variant"),
            super::common::square_brackets_comma_separated1(Self::parse),
        );

        let enum_parser = preceded(
            tag("Enum"),
            super::common::square_brackets_comma_separated1(super::expression::Term::parse),
        );

        let pattern_parser = preceded(
            tag("Pattern"),
            super::common::square_brackets_comma_separated1(super::regex::parse),
        );

        let regex_parser = preceded(
            tag("Regexp"),
            super::common::square_brackets_delimimited(super::regex::parse),
        );

        alt((
            map(TypeInteger::parse, Self::Integer),
            map(TypeFloat::parse, Self::Float),
            value(Self::Numeric, tag("Numeric")),
            map(TypeString::parse, Self::String),
            value(Self::Boolean, tag("Boolean")),
            map(TypeArray::parse, Self::Array),
            map(TypeHash::parse, Self::Hash),
            map(TypeOptional::parse, Self::Optional),
            map(TypeSensitive::parse, Self::Sensitive),
            map(TypeStruct::parse, Self::Struct),
            map(TypeTuple::parse, Self::Tuple),
            map(variant_parser, Self::Variant),
            map(enum_parser, Self::Enum),
            map(pattern_parser, Self::Pattern),
            map(regex_parser, Self::Regex),
            value(Self::Undef, tag("Undef")),
            value(Self::Any, tag("Any")),
            map(super::common::camelcase_identifier_with_ns, |v| {
                Self::Custom(v)
            }),
        ))(input)
    }
}

#[test]
fn test_type_specification() {
    assert_eq!(
        TypeSpecification::parse::<nom::error::Error<_>>("Stdlib::Unixpath").unwrap(),
        ("", TypeSpecification::Custom(vec!["Stdlib", "Unixpath"]))
    );
    assert_eq!(
        TypeSpecification::parse::<nom::error::Error<_>>("Numeric").unwrap(),
        ("", TypeSpecification::Numeric)
    );
    assert!(TypeSpecification::parse::<nom::error::Error<_>>("Pattern[//, /sdfsdf/]").is_ok());
    assert!(TypeSpecification::parse::<nom::error::Error<_>>("Regexp[/sdfsdf/]").is_ok());
}
