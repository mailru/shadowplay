use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    multi::separated_list0,
    sequence::{pair, preceded, tuple},
    Parser,
};
use puppet_lang::typing::ExternalType;

pub fn parse_or_default<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<Option<O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    alt((map(parser, |v: O| Some(v)), value(None, tag("default"))))
}

fn parse_min_max<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<(Option<O>, Option<O>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>> + Copy,
    O: Clone,
{
    move |input| {
        pair(
            crate::common::space0_delimimited(parse_or_default(parser)),
            map(
                opt(crate::common::space0_delimimited(preceded(
                    crate::common::comma_separator,
                    parse_or_default(parser),
                ))),
                |v| v.flatten(),
            ),
        )(input)
    }
}

fn parse_min_max_args<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Option<O>, Option<O>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>> + Copy,
    O: Clone,
{
    map(
        opt(crate::common::square_brackets_delimimited(parse_min_max(
            parser,
        ))),
        move |args: Option<(Option<O>, Option<O>)>| args.unwrap_or((None, None)),
    )
}

pub fn parse_float(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = preceded(
        tag("Float"),
        parse_min_max_args(crate::expression::term::parse_float_term),
    );

    map(parser, |(min, max)| {
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min,
            max,
            extra: crate::parser::Location::from(input),
        })
    })(input)
}

pub fn parse_integer(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = preceded(
        tag("Integer"),
        parse_min_max_args(crate::expression::term::parse_integer_term),
    );

    map(parser, |(min, max)| {
        puppet_lang::typing::TypeSpecificationVariant::Integer(puppet_lang::typing::TypeInteger {
            min,
            max,
            extra: crate::parser::Location::from(input),
        })
    })(input)
}

pub fn parse_string(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = preceded(
        tag("String"),
        parse_min_max_args(crate::expression::term::parse_usize_term),
    );

    map(parser, |(min, max)| {
        puppet_lang::typing::TypeSpecificationVariant::String(puppet_lang::typing::TypeString {
            min,
            max,
            extra: crate::parser::Location::from(input),
        })
    })(input)
}

fn parse_array(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = pair(
        parse_type_specification,
        opt(preceded(
            crate::common::comma_separator,
            parse_min_max(crate::expression::term::parse_usize_term),
        )),
    );

    let parser = map(parser, |(inner, min_max)| {
        let (min, max) = min_max.unwrap_or((None, None));
        puppet_lang::typing::TypeArray {
            inner: Some(Box::new(inner)),
            min,
            max,
            extra: crate::parser::Location::from(input),
        }
    });
    let parser = preceded(
        tag("Array"),
        map(
            opt(super::common::square_brackets_delimimited(parser)),
            |v| {
                v.unwrap_or(puppet_lang::typing::TypeArray {
                    inner: None,
                    min: None,
                    max: None,
                    extra: crate::parser::Location::from(input),
                })
            },
        ),
    );

    map(parser, puppet_lang::typing::TypeSpecificationVariant::Array)(input)
}

fn parse_hash(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let args_parser = tuple((
        parse_type_specification,
        preceded(super::common::comma_separator, parse_type_specification),
        opt(preceded(
            crate::common::comma_separator,
            parse_min_max_args(crate::expression::term::parse_usize_term),
        )),
    ));
    let args_parser = map(args_parser, |(key, value, min_max)| {
        let (min, max) = min_max.unwrap_or((None, None));
        puppet_lang::typing::TypeHash {
            key: Some(Box::new(key)),
            value: Some(Box::new(value)),
            min,
            max,
            extra: crate::parser::Location::from(input),
        }
    });

    let parser = preceded(
        tag("Hash"),
        map(
            opt(super::common::square_brackets_delimimited(args_parser)),
            |v| {
                v.unwrap_or(puppet_lang::typing::TypeHash {
                    key: None,
                    value: None,
                    min: None,
                    max: None,
                    extra: crate::parser::Location::from(input),
                })
            },
        ),
    );

    map(parser, puppet_lang::typing::TypeSpecificationVariant::Hash)(input)
}

fn parse_optional(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let arguments_parser = crate::common::square_brackets_delimimited(alt((
        map(parse_type_specification, |v| {
            puppet_lang::typing::TypeOptional {
                value: puppet_lang::typing::TypeOptionalVariant::TypeSpecification(Box::new(v)),
                extra: crate::parser::Location::from(input),
            }
        }),
        map(crate::expression::term::parse_term, |v| {
            puppet_lang::typing::TypeOptional {
                value: puppet_lang::typing::TypeOptionalVariant::Term(Box::new(v)),
                extra: crate::parser::Location::from(input),
            }
        }),
    )));

    let parser = preceded(tag("Optional"), arguments_parser);

    map(
        parser,
        puppet_lang::typing::TypeSpecificationVariant::Optional,
    )(input)
}

fn parse_sensitive(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = preceded(
        tag("Sensitive"),
        crate::common::square_brackets_delimimited(alt((
            map(parse_type_specification, |v| {
                puppet_lang::typing::TypeSensitive {
                    value: puppet_lang::typing::TypeSensitiveVariant::TypeSpecification(Box::new(
                        v,
                    )),
                    extra: crate::parser::Location::from(input),
                }
            }),
            map(super::expression::term::parse_term, |v| {
                puppet_lang::typing::TypeSensitive {
                    value: puppet_lang::typing::TypeSensitiveVariant::Term(Box::new(v)),
                    extra: crate::parser::Location::from(input),
                }
            }),
        ))),
    );

    map(
        parser,
        puppet_lang::typing::TypeSpecificationVariant::Sensitive,
    )(input)
}

fn parse_struct_key(input: Span) -> IResult<puppet_lang::typing::TypeStructKey<Location>> {
    let inner_parse = super::common::square_brackets_delimimited(alt((
        super::double_quoted::parse,
        super::single_quoted::parse,
    )));

    alt((
        preceded(
            tag("Optional"),
            map(inner_parse, puppet_lang::typing::TypeStructKey::Optional),
        ),
        map(
            super::double_quoted::parse,
            puppet_lang::typing::TypeStructKey::String,
        ),
        map(
            super::single_quoted::parse,
            puppet_lang::typing::TypeStructKey::String,
        ),
    ))(input)
}

fn parse_struct(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let kv_parser = pair(
        crate::common::space0_delimimited(parse_struct_key),
        preceded(
            tag("=>"),
            super::common::space0_delimimited(parse_type_specification),
        ),
    );

    let parser = preceded(
        tag("Struct"),
        map(
            super::common::square_brackets_delimimited(
                super::common::curly_brackets_comma_separated0(kv_parser),
            ),
            |keys| puppet_lang::typing::TypeStruct {
                keys,
                extra: Location::from(input),
            },
        ),
    );

    map(
        parser,
        puppet_lang::typing::TypeSpecificationVariant::Struct,
    )(input)
}

fn parse_tuple(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = preceded(
        tag("Tuple"),
        super::common::square_brackets_delimimited(pair(
            separated_list0(super::common::comma_separator, parse_type_specification),
            opt(preceded(
                super::common::comma_separator,
                parse_min_max(crate::expression::term::parse_usize_term),
            )),
        )),
    );

    let parser = map(parser, move |(list, min_max)| {
        let (min, max) = min_max.unwrap_or((None, None));
        puppet_lang::typing::TypeTuple {
            list,
            min,
            max,
            extra: Location::from(input),
        }
    });

    map(parser, puppet_lang::typing::TypeSpecificationVariant::Tuple)(input)
}

fn parse_external_type(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Location>> {
    let parser = pair(
        crate::identifier::camelcase_identifier_with_ns,
        opt(crate::common::square_brackets_comma_separated1(
            crate::expression::term::parse_term,
        )),
    );

    map(parser, |(name, arguments)| {
        puppet_lang::typing::TypeSpecificationVariant::ExternalType(ExternalType {
            name: name.iter().map(|v| v.to_string()).collect(),
            arguments: arguments.unwrap_or_default(),
            extra: Location::from(input),
        })
    })(input)
}

pub fn parse_type_specification(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecification<Location>> {
    let parse_variant = preceded(
        tag("Variant"),
        map(
            crate::common::square_brackets_comma_separated1(parse_type_specification),
            |list| {
                puppet_lang::typing::TypeSpecificationVariant::Variant(
                    puppet_lang::typing::Variant {
                        list,
                        extra: Location::from(input),
                    },
                )
            },
        ),
    );

    let parse_enum = preceded(
        tag("Enum"),
        map(
            super::common::square_brackets_comma_separated1(super::expression::term::parse_term),
            |list| {
                puppet_lang::typing::TypeSpecificationVariant::Enum(puppet_lang::typing::Enum {
                    list,
                    extra: Location::from(input),
                })
            },
        ),
    );

    let parse_pattern = preceded(
        tag("Pattern"),
        map(
            super::common::square_brackets_comma_separated1(crate::regex::parse),
            |list| {
                puppet_lang::typing::TypeSpecificationVariant::Pattern(
                    puppet_lang::typing::Pattern {
                        list,
                        extra: Location::from(input),
                    },
                )
            },
        ),
    );

    let parse_regexp = preceded(
        tag("Regexp"),
        map(
            super::common::square_brackets_delimimited(crate::regex::parse),
            |data| {
                puppet_lang::typing::TypeSpecificationVariant::Regex(puppet_lang::typing::Regex {
                    data,
                    extra: Location::from(input),
                })
            },
        ),
    );

    let parse_numeric = value(
        puppet_lang::typing::TypeSpecificationVariant::Numeric(puppet_lang::typing::Numeric {
            extra: Location::from(input),
        }),
        tag("Numeric"),
    );

    let parse_boolean = value(
        puppet_lang::typing::TypeSpecificationVariant::Boolean(puppet_lang::typing::Boolean {
            extra: Location::from(input),
        }),
        tag("Boolean"),
    );

    let parse_undef = value(
        puppet_lang::typing::TypeSpecificationVariant::Undef(puppet_lang::typing::Undef {
            extra: Location::from(input),
        }),
        tag("Undef"),
    );

    let parse_any = value(
        puppet_lang::typing::TypeSpecificationVariant::Any(puppet_lang::typing::Any {
            extra: Location::from(input),
        }),
        tag("Any"),
    );

    let parser = alt((
        parse_integer,
        parse_float,
        parse_numeric,
        parse_string,
        parse_boolean,
        parse_array,
        parse_hash,
        parse_optional,
        parse_sensitive,
        parse_struct,
        parse_tuple,
        parse_variant,
        parse_enum,
        parse_pattern,
        parse_regexp,
        parse_undef,
        parse_any,
        parse_external_type,
    ));

    map(parser, |data| puppet_lang::typing::TypeSpecification {
        data,
        extra: Location::from(input),
    })(input)
}

#[test]
fn test_float() {
    assert_eq!(
        parse_float(Span::new("Float")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min: None,
            max: None,
            extra: Location::new(0, 1, 1)
        })
    );
    assert_eq!(
        parse_float(Span::new("Float[ 100.0 ]")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min: Some(puppet_lang::expression::Float {
                value: 100.0,
                extra: Location::new(7, 1, 8)
            }),
            max: None,
            extra: Location::new(0, 1, 1)
        })
    );
    assert_eq!(
        parse_float(Span::new("Float[ 100.0, 200.0 ]")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min: Some(puppet_lang::expression::Float {
                value: 100.0,
                extra: Location::new(7, 1, 8)
            }),
            max: Some(puppet_lang::expression::Float {
                value: 200.0,
                extra: Location::new(14, 1, 15)
            }),
            extra: Location::new(0, 1, 1)
        })
    );
    assert!(parse_float(Span::new("Float[ 100,  1000, 10.0]")).is_ok());
    assert!(parse_float(Span::new("Float[]")).is_ok())
}

#[test]
fn test_array() {
    assert_eq!(
        parse_array(Span::new("Array")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Array(puppet_lang::typing::TypeArray {
            extra: Location::new(0, 1, 1),
            inner: None,
            min: None,
            max: None
        })
    );
    assert_eq!(
        parse_array(Span::new("Array [String[ 1,2], 10 ]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Array(puppet_lang::typing::TypeArray {
            extra: Location::new(0, 1, 1),
            inner: Some(Box::new(puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::String(
                    puppet_lang::typing::TypeString {
                        min: Some(puppet_lang::expression::Usize {
                            value: 1,
                            extra: Location::new(15, 1, 16)
                        }),
                        max: Some(puppet_lang::expression::Usize {
                            value: 2,
                            extra: Location::new(17, 1, 18)
                        }),
                        extra: Location::new(7, 1, 8)
                    }
                ),
                extra: Location::new(7, 1, 8)
            })),
            min: Some(puppet_lang::expression::Usize {
                value: 10,
                extra: Location::new(21, 1, 22)
            }),
            max: None
        })
    );
}

#[test]
fn test_hash() {
    assert_eq!(
        parse_hash(Span::new("Hash [String[1,2 ], Boolean]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Hash(puppet_lang::typing::TypeHash {
            extra: Location::new(0, 1, 1),
            key: Some(Box::new(puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::String(
                    puppet_lang::typing::TypeString {
                        min: Some(puppet_lang::expression::Usize {
                            value: 1,
                            extra: Location::new(13, 1, 14)
                        }),
                        max: Some(puppet_lang::expression::Usize {
                            value: 2,
                            extra: Location::new(15, 1, 16)
                        }),
                        extra: Location::new(6, 1, 7)
                    }
                ),
                extra: Location::new(6, 1, 7)
            })),
            value: Some(Box::new(puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::Boolean(
                    puppet_lang::typing::Boolean {
                        extra: Location::new(20, 1, 21)
                    }
                ),
                extra: Location::new(20, 1, 21)
            })),
            min: None,
            max: None
        })
    );
    assert!(parse_hash(Span::new("Hash[String, Hash[ String, String]]]")).is_ok())
}

#[test]
fn test_optional() {
    assert_eq!(
        parse_optional(Span::new("Optional [String[1,2 ] ]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Optional(
            puppet_lang::typing::TypeOptional {
                extra: Location::new(0, 1, 1),
                value: puppet_lang::typing::TypeOptionalVariant::TypeSpecification(Box::new(
                    puppet_lang::typing::TypeSpecification {
                        data: puppet_lang::typing::TypeSpecificationVariant::String(
                            puppet_lang::typing::TypeString {
                                min: Some(puppet_lang::expression::Usize {
                                    value: 1,
                                    extra: Location::new(17, 1, 18)
                                }),
                                max: Some(puppet_lang::expression::Usize {
                                    value: 2,
                                    extra: Location::new(19, 1, 20)
                                }),
                                extra: Location::new(10, 1, 11)
                            }
                        ),
                        extra: Location::new(10, 1, 11)
                    }
                ))
            }
        )
    )
}

#[test]
fn test_struct() {
    assert_eq!(
        parse_struct(Span::new("Struct [{some_key => Boolean } ]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Struct(puppet_lang::typing::TypeStruct {
            extra: Location::new(0, 1, 1),
            keys: vec![(
                puppet_lang::typing::TypeStructKey::String(puppet_lang::expression::StringExpr {
                    data: "some_key".to_owned(),
                    variant: puppet_lang::expression::StringVariant::SingleQuoted,
                    extra: Location::new(9, 1, 10)
                }),
                puppet_lang::typing::TypeSpecification {
                    data: puppet_lang::typing::TypeSpecificationVariant::Boolean(
                        puppet_lang::typing::Boolean {
                            extra: Location::new(21, 1, 22)
                        }
                    ),
                    extra: Location::new(21, 1, 22)
                }
            )]
        })
    );
}

#[test]
fn test_tuple() {
    assert_eq!(
        parse_tuple(Span::new("Tuple [Integer[1,2], 10, 100 ]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Tuple(puppet_lang::typing::TypeTuple {
            extra: Location::new(0, 1, 1),
            list: vec![puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::Integer(
                    puppet_lang::typing::TypeInteger {
                        min: Some(puppet_lang::expression::Integer {
                            value: 1,
                            extra: Location::new(15, 1, 16)
                        }),
                        max: Some(puppet_lang::expression::Integer {
                            value: 2,
                            extra: Location::new(17, 1, 18)
                        }),
                        extra: Location::new(7, 1, 8)
                    }
                ),
                extra: Location::new(7, 1, 8)
            }],
            min: Some(puppet_lang::expression::Usize {
                value: 10,
                extra: Location::new(21, 1, 22)
            }),
            max: Some(puppet_lang::expression::Usize {
                value: 100,
                extra: Location::new(25, 1, 26)
            }),
        })
    );
    assert_eq!(
        parse_tuple(Span::new("Tuple [Integer[1,2], Integer[3,4] ]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Tuple(puppet_lang::typing::TypeTuple {
            extra: Location::new(0, 1, 1),
            list: vec![
                puppet_lang::typing::TypeSpecification {
                    data: puppet_lang::typing::TypeSpecificationVariant::Integer(
                        puppet_lang::typing::TypeInteger {
                            min: Some(puppet_lang::expression::Integer {
                                value: 1,
                                extra: Location::new(15, 1, 16)
                            }),
                            max: Some(puppet_lang::expression::Integer {
                                value: 2,
                                extra: Location::new(17, 1, 18)
                            }),
                            extra: Location::new(7, 1, 8)
                        }
                    ),
                    extra: Location::new(7, 1, 8)
                },
                puppet_lang::typing::TypeSpecification {
                    data: puppet_lang::typing::TypeSpecificationVariant::Integer(
                        puppet_lang::typing::TypeInteger {
                            min: Some(puppet_lang::expression::Integer {
                                value: 3,
                                extra: Location::new(29, 1, 30)
                            }),
                            max: Some(puppet_lang::expression::Integer {
                                value: 4,
                                extra: Location::new(31, 1, 32)
                            }),
                            extra: Location::new(21, 1, 22)
                        }
                    ),
                    extra: Location::new(21, 1, 22)
                }
            ],
            min: None,
            max: None,
        })
    );
    assert!(parse_tuple(Span::new("Tuple")).is_err());
}

#[test]
fn test_type_specification() {
    assert_eq!(
        parse_type_specification(Span::new("Stdlib::Unixpath"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecification {
            data: puppet_lang::typing::TypeSpecificationVariant::ExternalType(
                puppet_lang::typing::ExternalType {
                    name: vec!["Stdlib".to_owned(), "Unixpath".to_owned()],
                    arguments: Vec::new(),
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse_type_specification(Span::new("Class['hello']"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecification {
            data: puppet_lang::typing::TypeSpecificationVariant::ExternalType(
                puppet_lang::typing::ExternalType {
                    name: vec!["Class".to_owned(),],
                    arguments: vec![puppet_lang::expression::Term {
                        value: puppet_lang::expression::TermVariant::String(
                            puppet_lang::expression::StringExpr {
                                data: "hello".to_owned(),
                                variant: puppet_lang::expression::StringVariant::SingleQuoted,
                                extra: Location::new(6, 1, 7),
                            }
                        ),
                        extra: Location::new(6, 1, 7),
                    }],
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse_type_specification(Span::new("Numeric")).unwrap().1,
        puppet_lang::typing::TypeSpecification {
            data: puppet_lang::typing::TypeSpecificationVariant::Numeric(
                puppet_lang::typing::Numeric {
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
    assert!(parse_type_specification(Span::new("Pattern[//, /sdfsdf/]")).is_ok());
    assert!(parse_type_specification(Span::new("Regexp[/sdfsdf/]")).is_ok());
}
