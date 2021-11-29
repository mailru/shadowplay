use std::primitive;

use super::parser::{IResult, IResultUnmarked, Marked, ParseError, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    multi::separated_list0,
    sequence::{pair, preceded, tuple},
    Parser,
};

pub fn parse_or_default<'a, O, F>(parser: F, default: O) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    Marked::parse(alt((parser, value(default, tag("default")))))
}

fn parse_min_max<'a, O, F>(
    parser: F,
    default_min: O,
    default_max: O,
) -> impl FnMut(Span<'a>) -> IResultUnmarked<(Marked<O>, Marked<O>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>> + Copy,
    O: Copy,
{
    move |input| {
        map(
            pair(
                super::common::space0_delimimited(parse_or_default(parser, default_min)),
                opt(super::common::space0_delimimited(preceded(
                    super::common::comma_separator,
                    parse_or_default(parser, default_max),
                ))),
            ),
            move |(min, max)| (min, max.unwrap_or_else(|| Marked::new(&input, default_max))),
        )(input)
    }
}

fn parse_min_max_args<'a, O, F>(
    parser: F,
    default_min: O,
    default_max: O,
) -> impl FnMut(Span<'a>) -> IResultUnmarked<(Marked<O>, Marked<O>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>> + Copy,
    O: Clone + Copy,
{
    move |input: Span| {
        map(
            opt(super::common::square_brackets_delimimited(Marked::parse(
                parse_min_max(parser, default_min, default_max),
            ))),
            move |args: Option<Marked<(Marked<O>, Marked<O>)>>| {
                args.map(|v| v.data).unwrap_or((
                    Marked::new(&input, default_min),
                    Marked::new(&input, default_max),
                ))
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeFloat {
    pub min: Marked<f32>,
    pub max: Marked<f32>,
}

impl TypeFloat {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = preceded(
            tag("Float"),
            parse_min_max_args(
                super::expression::Float::plain_parse,
                primitive::f32::MIN,
                primitive::f32::MAX,
            ),
        );

        map(parser, |(min, max)| Marked::new(&input, Self { min, max }))(input)
    }
}

#[test]
fn test_float() {
    assert_eq!(
        TypeFloat::parse(Span::new("Float")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: TypeFloat {
                min: Marked {
                    line: 1,
                    column: 6,
                    data: primitive::f32::MIN
                },
                max: Marked {
                    line: 1,
                    column: 6,
                    data: primitive::f32::MAX
                }
            }
        }
    );
    assert_eq!(
        TypeFloat::parse(Span::new("Float[ 100.0 ]")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: TypeFloat {
                min: Marked {
                    line: 1,
                    column: 8,
                    data: 100.
                },
                max: Marked {
                    line: 1,
                    column: 8,
                    data: primitive::f32::MAX
                }
            }
        }
    );
    assert_eq!(
        TypeFloat::parse(Span::new("Float[ 100.0,1000.0]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeFloat {
                min: Marked {
                    line: 1,
                    column: 8,
                    data: 100.
                },
                max: Marked {
                    line: 1,
                    column: 14,
                    data: 1000.
                },
            }
        }
    );
    assert!(TypeFloat::parse(Span::new("Float[ 100,  1000, 10.0]")).is_ok());
    assert!(TypeFloat::parse(Span::new("Float[]")).is_ok())
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeInteger {
    pub min: Marked<i64>,
    pub max: Marked<i64>,
}

impl TypeInteger {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = preceded(
            tag("Integer"),
            parse_min_max_args(
                nom::character::complete::i64,
                primitive::i64::MIN,
                primitive::i64::MAX,
            ),
        );

        map(parser, |(min, max)| Marked::new(&input, Self { min, max }))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeString {
    pub min: Marked<usize>,
    pub max: Marked<usize>,
}

impl TypeString {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = preceded(
            tag("String"),
            parse_min_max_args(
                nom::character::complete::u64,
                primitive::u64::MIN,
                primitive::u64::MAX,
            ),
        );

        map(parser, |(min, max)| {
            Marked::new(
                &input,
                Self {
                    min: min.map(|v| v as usize),
                    max: max.map(|v| v as usize),
                },
            )
        })(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeArray {
    pub inner: Option<Box<Marked<TypeSpecification>>>,
    pub min: Marked<usize>,
    pub max: Marked<usize>,
}

impl TypeArray {
    fn parse(input: Span) -> IResult<Self> {
        let parser = pair(
            Marked::parse(TypeSpecification::parse),
            opt(preceded(
                super::common::comma_separator,
                parse_min_max(
                    nom::character::complete::u64,
                    primitive::u64::MIN,
                    primitive::u64::MAX,
                ),
            )),
        );
        let parser = Marked::parse(map(parser, |(inner, limits)| {
            let (min, max) = limits.unwrap_or((
                Marked::new(&input, primitive::u64::MIN),
                Marked::new(&input, primitive::u64::MAX),
            ));
            Self {
                inner: Some(Box::new(inner.data)),
                min: min.map(|v| v as usize),
                max: max.map(|v| v as usize),
            }
        }));
        Marked::parse(preceded(
            tag("Array"),
            map(
                opt(super::common::square_brackets_delimimited(parser)),
                |v| {
                    v.map(|v| v.data).unwrap_or(Self {
                        inner: None,
                        min: Marked::new(&input, primitive::usize::MIN),
                        max: Marked::new(&input, primitive::usize::MAX),
                    })
                },
            ),
        ))(input)
    }
}

#[test]
fn test_array() {
    assert_eq!(
        TypeArray::parse(Span::new("Array [String[1,2 ],10]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeArray {
                inner: Some(Box::new(Marked {
                    line: 1,
                    column: 8,
                    data: TypeSpecification::String(TypeString {
                        min: Marked {
                            line: 1,
                            column: 15,
                            data: 1
                        },
                        max: Marked {
                            line: 1,
                            column: 17,
                            data: 2
                        }
                    })
                })),
                min: Marked {
                    line: 1,
                    column: 21,
                    data: 10
                },
                max: Marked {
                    line: 1,
                    column: 21,
                    data: primitive::usize::MAX
                }
            }
        }
    );
    assert_eq!(
        TypeArray::parse(Span::new("Array")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: TypeArray {
                inner: None,
                min: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::usize::MIN
                },
                max: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::usize::MAX
                }
            }
        }
    );
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeHash {
    pub key: Option<Box<Marked<TypeSpecification>>>,
    pub value: Option<Box<Marked<TypeSpecification>>>,
    pub min: Marked<usize>,
    pub max: Marked<usize>,
}

impl TypeHash {
    fn parse(input: Span) -> IResult<Self> {
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
            let (min, max) = limits.unwrap_or((
                Marked::new(&input, primitive::u64::MIN),
                Marked::new(&input, primitive::u64::MAX),
            ));
            Marked::new(
                &input,
                Self {
                    key: Some(Box::new(key)),
                    value: Some(Box::new(value)),
                    min: min.map(|v| v as usize),
                    max: max.map(|v| v as usize),
                },
            )
        });
        preceded(
            tag("Hash"),
            map(
                opt(super::common::square_brackets_delimimited(parser)),
                |v| {
                    v.unwrap_or_else(|| {
                        Marked::new(
                            &input,
                            Self {
                                key: None,
                                value: None,
                                min: Marked::new(&input, primitive::usize::MIN),
                                max: Marked::new(&input, primitive::usize::MAX),
                            },
                        )
                    })
                },
            ),
        )(input)
    }
}

#[test]
fn test_hash() {
    assert_eq!(
        TypeHash::parse(Span::new("Hash [String[1,2 ], Boolean]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeHash {
                key: Some(Box::new(Marked {
                    line: 1,
                    column: 7,
                    data: TypeSpecification::String(TypeString {
                        min: Marked {
                            line: 1,
                            column: 14,
                            data: 1
                        },
                        max: Marked {
                            line: 1,
                            column: 16,
                            data: 2
                        }
                    })
                })),
                value: Some(Box::new(Marked {
                    line: 1,
                    column: 21,
                    data: TypeSpecification::Boolean
                })),
                min: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::usize::MIN
                },
                max: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::usize::MAX
                }
            }
        }
    );
    assert!(TypeHash::parse(Span::new("Hash[String, Hash[ String, String]]]")).is_ok())
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeOptional {
    TypeSpecification(Box<Marked<TypeSpecification>>),
    Term(Box<Marked<super::expression::Term>>),
}

impl TypeOptional {
    fn parse(input: Span) -> IResult<Self> {
        let arguments_parser = super::common::square_brackets_delimimited(Marked::parse(alt((
            map(TypeSpecification::parse, |v| {
                Self::TypeSpecification(Box::new(v))
            }),
            map(super::expression::Term::parse, |v| Self::Term(Box::new(v))),
        ))));

        Marked::parse(preceded(tag("Optional"), map(arguments_parser, |v| v.data)))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSensitive {
    TypeSpecification(Box<TypeSpecification>),
    Term(Box<super::expression::Term>),
}

impl TypeSensitive {
    fn parse(input: Span) -> IResult<Self> {
        preceded(
            tag("Sensitive"),
            super::common::square_brackets_delimimited(Marked::parse(alt((
                map(TypeSpecification::parse, |v| {
                    Self::TypeSpecification(Box::new(v.data))
                }),
                map(super::expression::Term::parse, |v| {
                    Self::Term(Box::new(v.data))
                }),
            )))),
        )(input)
    }
}

#[test]
fn test_optional() {
    assert_eq!(
        TypeOptional::parse(Span::new("Optional [String[1,2 ] ]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeOptional::TypeSpecification(Box::new(Marked {
                line: 1,
                column: 11,
                data: TypeSpecification::String(TypeString {
                    min: Marked {
                        line: 1,
                        column: 18,
                        data: 1
                    },
                    max: Marked {
                        line: 1,
                        column: 20,
                        data: 2
                    }
                })
            }))
        }
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeStructKey {
    SingleQuoted(String),
    DoubleQuoted(String),
    Optional(String),
}

impl TypeStructKey {
    pub fn parse(input: Span) -> IResult<Self> {
        let inner_parse = super::common::square_brackets_delimimited(alt((
            super::double_quoted::parse,
            super::single_quoted::parse,
        )));

        Marked::parse(alt((
            preceded(
                tag("Optional"),
                map(inner_parse, |v| Self::Optional(v.data)),
            ),
            map(super::double_quoted::parse, |v| Self::DoubleQuoted(v.data)),
            map(super::single_quoted::parse, |v| Self::SingleQuoted(v.data)),
        )))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeStruct {
    pub keys: Vec<(Marked<TypeStructKey>, Marked<TypeSpecification>)>,
}

impl TypeStruct {
    fn parse(input: Span) -> IResult<Self> {
        let kv_parser = pair(
            super::common::space0_delimimited(TypeStructKey::parse),
            preceded(
                tag("=>"),
                super::common::space0_delimimited(TypeSpecification::parse),
            ),
        );

        let parser = preceded(
            tag("Struct"),
            map(
                super::common::square_brackets_delimimited(
                    super::common::curly_brackets_comma_separated0(Marked::parse(kv_parser)),
                ),
                |keys: Marked<_>| Self {
                    keys: keys.data.into_iter().map(|v| v.data).collect(),
                },
            ),
        );

        Marked::parse(parser)(input)
    }
}

#[test]
fn test_struct() {
    assert_eq!(
        TypeStruct::parse(Span::new("Struct [{some_key => Boolean } ]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeStruct {
                keys: vec![(
                    Marked {
                        line: 1,
                        column: 10,
                        data: TypeStructKey::SingleQuoted("some_key".to_owned())
                    },
                    Marked {
                        line: 1,
                        column: 22,
                        data: TypeSpecification::Boolean
                    }
                )]
            }
        }
    );
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeTuple {
    pub list: Vec<Marked<TypeSpecification>>,
    pub min: Marked<usize>,
    pub max: Marked<usize>,
}

impl TypeTuple {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = preceded(
            tag("Tuple"),
            super::common::square_brackets_delimimited(Marked::parse(pair(
                separated_list0(super::common::comma_separator, TypeSpecification::parse),
                opt(preceded(
                    super::common::comma_separator,
                    parse_min_max(
                        nom::character::complete::u64,
                        primitive::u64::MIN,
                        primitive::u64::MAX,
                    ),
                )),
            ))),
        );

        let parser = map(parser, move |parsed| {
            let (list, min_max) = parsed.data;
            let (min, max) = min_max.unwrap_or((
                Marked::new(&input, primitive::u64::MIN),
                Marked::new(&input, primitive::u64::MAX),
            ));
            Self {
                list,
                min: min.map(|v| v as usize),
                max: max.map(|v| v as usize),
            }
        });

        Marked::parse(parser)(input)
    }
}

#[test]
fn test_tuple() {
    assert_eq!(
        TypeTuple::parse(Span::new("Tuple [Integer[1,2], 10, 100 ]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeTuple {
                list: vec![Marked {
                    line: 1,
                    column: 8,
                    data: TypeSpecification::Integer(TypeInteger {
                        min: Marked {
                            line: 1,
                            column: 16,
                            data: 1
                        },
                        max: Marked {
                            line: 1,
                            column: 18,
                            data: 2
                        }
                    })
                }],
                min: Marked {
                    line: 1,
                    column: 22,
                    data: 10
                },
                max: Marked {
                    line: 1,
                    column: 26,
                    data: 100
                },
            }
        }
    );
    assert_eq!(
        TypeTuple::parse(Span::new("Tuple [Integer[1,2] ]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeTuple {
                list: vec![Marked {
                    line: 1,
                    column: 8,
                    data: TypeSpecification::Integer(TypeInteger {
                        min: Marked {
                            line: 1,
                            column: 16,
                            data: 1
                        },
                        max: Marked {
                            line: 1,
                            column: 18,
                            data: 2
                        }
                    })
                }],
                min: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::u64::MIN as usize
                },
                max: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::u64::MAX as usize
                },
            }
        }
    );
    assert_eq!(
        TypeTuple::parse(Span::new("Tuple [Integer[1,2], Integer[1,2] ]"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeTuple {
                list: vec![
                    Marked {
                        line: 1,
                        column: 8,
                        data: TypeSpecification::Integer(TypeInteger {
                            min: Marked {
                                line: 1,
                                column: 16,
                                data: 1
                            },
                            max: Marked {
                                line: 1,
                                column: 18,
                                data: 2
                            }
                        })
                    },
                    Marked {
                        line: 1,
                        column: 22,
                        data: TypeSpecification::Integer(TypeInteger {
                            min: Marked {
                                line: 1,
                                column: 30,
                                data: 1
                            },
                            max: Marked {
                                line: 1,
                                column: 32,
                                data: 2
                            }
                        })
                    }
                ],
                min: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::u64::MIN as usize
                },
                max: Marked {
                    line: 1,
                    column: 1,
                    data: primitive::u64::MAX as usize
                },
            }
        }
    );
    assert!(TypeTuple::parse(Span::new("Tuple")).is_err());
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpecification {
    Float(TypeFloat),
    Integer(TypeInteger),
    Numeric,
    String(TypeString),
    Pattern(Vec<Marked<String>>),
    Regex(String),
    Hash(TypeHash),
    Boolean,
    Array(TypeArray),
    Undef,
    Any,
    Optional(TypeOptional),
    Variant(Vec<Marked<TypeSpecification>>),
    Enum(Vec<Marked<super::expression::Term>>),
    Struct(TypeStruct),
    Custom(Vec<String>),
    Sensitive(TypeSensitive),
    Tuple(TypeTuple),
}

impl TypeSpecification {
    pub fn parse(input: Span) -> IResult<Self> {
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

        let parser = alt((
            map(TypeInteger::parse, |v| Self::Integer(v.data)),
            map(TypeFloat::parse, |v| Self::Float(v.data)),
            value(Self::Numeric, tag("Numeric")),
            map(TypeString::parse, |v| Self::String(v.data)),
            value(Self::Boolean, tag("Boolean")),
            map(TypeArray::parse, |v| Self::Array(v.data)),
            map(TypeHash::parse, |v| Self::Hash(v.data)),
            map(TypeOptional::parse, |v| Self::Optional(v.data)),
            map(TypeSensitive::parse, |v| Self::Sensitive(v.data)),
            map(TypeStruct::parse, |v| Self::Struct(v.data)),
            map(TypeTuple::parse, |v| Self::Tuple(v.data)),
            map(variant_parser, |v| Self::Variant(v.data)),
            map(enum_parser, |v| Self::Enum(v.data)),
            map(pattern_parser, |v| Self::Pattern(v.data)),
            map(regex_parser, |v| Self::Regex(v.data)),
            value(Self::Undef, tag("Undef")),
            value(Self::Any, tag("Any")),
            map(super::common::camelcase_identifier_with_ns, |v| {
                Self::Custom(v.data.into_iter().map(|v| String::from(v.data)).collect())
            }),
        ));

        Marked::parse(parser)(input)
    }
}

#[test]
fn test_type_specification() {
    assert_eq!(
        TypeSpecification::parse(Span::new("Stdlib::Unixpath"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: TypeSpecification::Custom(vec!["Stdlib".to_owned(), "Unixpath".to_owned()])
        }
    );
    assert_eq!(
        TypeSpecification::parse(Span::new("Numeric")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: TypeSpecification::Numeric
        }
    );
    assert!(TypeSpecification::parse(Span::new("Pattern[//, /sdfsdf/]")).is_ok());
    assert!(TypeSpecification::parse(Span::new("Regexp[/sdfsdf/]")).is_ok());
}
