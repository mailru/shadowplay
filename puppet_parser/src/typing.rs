use crate::{range::Range, IResult, ParseError, Span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    multi::separated_list0,
    sequence::{pair, preceded, tuple},
    Parser,
};
use puppet_lang::typing::{ExternalType, OptionalStructKey};

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
                    opt(parse_or_default(parser)),
                ))),
                |v| v.flatten().flatten(),
            ),
        )(input)
    }
}

fn parse_min_max_args<'a, O, F>(
    keyword: &'static str,
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Option<O>, Option<O>, Range, Range)>
where
    F: Parser<Span<'a>, O, ParseError<'a>> + Copy,
    O: Clone,
{
    map(
        pair(
            tag(keyword),
            opt(crate::common::square_brackets_delimimited(parse_min_max(
                parser,
            ))),
        ),
        // move |args: Option<(Span<'a>, (Option<O>, Option<O>), Span<'a>)>| {
        move |(kw, args)| match args {
            Some((_left_bracket, (min, max), right_bracket)) => (
                min,
                max,
                Range::from((kw, kw)),
                Range::from((right_bracket, right_bracket)),
            ),
            None => (None, None, Range::from((kw, kw)), Range::from((kw, kw))),
        },
    )
}

pub fn parse_float(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    map(
        parse_min_max_args("Float", crate::term::parse_float_term),
        |(min, max, start_range, end_range)| {
            puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
                min,
                max,
                extra: Range::from((&start_range, &end_range)),
            })
        },
    )(input)
}

pub fn parse_integer(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    map(
        parse_min_max_args("Float", crate::term::parse_integer_term),
        |(min, max, start_range, end_range)| {
            puppet_lang::typing::TypeSpecificationVariant::Integer(
                puppet_lang::typing::TypeInteger {
                    min,
                    max,
                    extra: Range::from((&start_range, &end_range)),
                },
            )
        },
    )(input)
}

pub fn parse_string(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    map(
        parse_min_max_args("String", crate::term::parse_usize_term),
        |(min, max, start_range, end_range)| {
            puppet_lang::typing::TypeSpecificationVariant::String(puppet_lang::typing::TypeString {
                min,
                max,
                extra: Range::from((&start_range, &end_range)),
            })
        },
    )(input)
}

fn parse_array(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let parser = pair(
        parse_type_specification,
        opt(preceded(
            crate::common::comma_separator,
            parse_min_max(crate::term::parse_usize_term),
        )),
    );

    let (input, keyword) = tag("Array")(input)?;

    let parser = map(parser, move |(inner, min_max)| {
        let (min, max) = min_max.unwrap_or((None, None));
        puppet_lang::typing::TypeArray {
            inner: Some(Box::new(inner)),
            min,
            max,
            extra: Range::from((keyword, keyword)),
        }
    });
    let parser = map(
        opt(crate::common::square_brackets_delimimited(parser)),
        move |v| {
            v.map(|v| v.1).unwrap_or(puppet_lang::typing::TypeArray {
                inner: None,
                min: None,
                max: None,
                extra: Range::from((keyword, keyword)),
            })
        },
    );

    map(parser, puppet_lang::typing::TypeSpecificationVariant::Array)(input)
}

fn parse_hash(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let args_parser = tuple((
        parse_type_specification,
        preceded(crate::common::comma_separator, parse_type_specification),
        opt(preceded(
            crate::common::comma_separator,
            parse_min_max(crate::term::parse_usize_term),
        )),
    ));

    let (input, keyword) = tag("Hash")(input)?;

    let parser = map(
        opt(crate::common::square_brackets_delimimited(args_parser)),
        move |args| match args {
            None => puppet_lang::typing::TypeHash {
                key: None,
                value: None,
                min: None,
                max: None,
                extra: Range::from((keyword, keyword)),
            },
            Some((_left_bracket, (key_type, value_type, min_max), right_bracket)) => {
                let (min, max) = min_max.unwrap_or((None, None));
                puppet_lang::typing::TypeHash {
                    key: Some(Box::new(key_type)),
                    value: Some(Box::new(value_type)),
                    min,
                    max,
                    extra: Range::from((keyword, right_bracket)),
                }
            }
        },
    );

    map(parser, puppet_lang::typing::TypeSpecificationVariant::Hash)(input)
}

fn parse_optional(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let (input, keyword) = tag("Optional")(input)?;

    let parser = alt((
        map(
            crate::common::square_brackets_delimimited(parse_type_specification),
            move |(_left_bracket, v, right_bracket)| puppet_lang::typing::TypeOptional {
                value: puppet_lang::typing::TypeOptionalVariant::TypeSpecification(Box::new(v)),
                extra: Range::from((keyword, right_bracket)),
            },
        ),
        map(
            crate::common::square_brackets_delimimited(crate::term::parse_term),
            move |(_left_bracket, v, right_bracket)| puppet_lang::typing::TypeOptional {
                value: puppet_lang::typing::TypeOptionalVariant::Term(Box::new(v)),
                extra: Range::from((keyword, right_bracket)),
            },
        ),
    ));

    map(
        parser,
        puppet_lang::typing::TypeSpecificationVariant::Optional,
    )(input)
}

fn parse_sensitive(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let (input, keyword) = tag("Sensitive")(input)?;

    let parser = alt((
        map(
            crate::common::square_brackets_delimimited(parse_type_specification),
            move |(_left_bracket, v, right_bracket)| puppet_lang::typing::TypeSensitive {
                value: puppet_lang::typing::TypeSensitiveVariant::TypeSpecification(Box::new(v)),
                extra: Range::from((keyword, right_bracket)),
            },
        ),
        map(
            crate::common::square_brackets_delimimited(crate::term::parse_term),
            move |(_left_bracket, v, right_bracket)| puppet_lang::typing::TypeSensitive {
                value: puppet_lang::typing::TypeSensitiveVariant::Term(Box::new(v)),
                extra: Range::from((keyword, right_bracket)),
            },
        ),
    ));

    map(
        parser,
        puppet_lang::typing::TypeSpecificationVariant::Sensitive,
    )(input)
}

fn parse_struct_key(input: Span) -> IResult<puppet_lang::typing::TypeStructKey<Range>> {
    let inner_parse = crate::common::square_brackets_delimimited(alt((
        crate::double_quoted::parse,
        crate::single_quoted::parse,
    )));

    alt((
        map(
            pair(tag("Optional"), inner_parse),
            |(opt_kw, (_left_bracket, value, right_bracket))| {
                puppet_lang::typing::TypeStructKey::Optional(OptionalStructKey {
                    extra: (opt_kw, right_bracket).into(),
                    value,
                })
            },
        ),
        map(
            crate::double_quoted::parse,
            puppet_lang::typing::TypeStructKey::String,
        ),
        map(
            crate::single_quoted::parse,
            puppet_lang::typing::TypeStructKey::String,
        ),
    ))(input)
}

fn parse_struct(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let kv_parser = pair(
        crate::common::space0_delimimited(parse_struct_key),
        preceded(
            tag("=>"),
            crate::common::space0_delimimited(parse_type_specification),
        ),
    );

    let parser = map(
        pair(
            tag("Struct"),
            crate::common::square_brackets_delimimited(
                crate::common::curly_brackets_comma_separated0(kv_parser),
            ),
        ),
        |(
            tag_kw,
            (_left_bracket, (_inner_left_bracket, keys, _inner_right_bracket), right_bracket),
        )| puppet_lang::typing::TypeStruct {
            keys,
            extra: (tag_kw, right_bracket).into(),
        },
    );

    map(
        parser,
        puppet_lang::typing::TypeSpecificationVariant::Struct,
    )(input)
}

fn parse_tuple(input: Span) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let (input, tag_kw) = tag("Tuple")(input)?;

    let parser = crate::common::square_brackets_delimimited(pair(
        separated_list0(crate::common::comma_separator, parse_type_specification),
        opt(preceded(
            crate::common::comma_separator,
            parse_min_max(crate::term::parse_usize_term),
        )),
    ));

    let parser = map(
        parser,
        move |(_left_bracket, (list, min_max), right_bracket)| {
            let (min, max) = min_max.unwrap_or((None, None));
            puppet_lang::typing::TypeTuple {
                list,
                min,
                max,
                extra: (tag_kw, right_bracket).into(),
            }
        },
    );

    map(parser, puppet_lang::typing::TypeSpecificationVariant::Tuple)(input)
}

fn parse_external_type(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecificationVariant<Range>> {
    let parser = pair(
        crate::identifier::camelcase_identifier_with_ns,
        opt(crate::common::square_brackets_comma_separated1(alt((
            crate::expression::parse_expression,
            // TODO parse values like Class[some::class::name]
        )))),
    );

    map(parser, |(name, arguments)| {
        let (arguments, extra) = match arguments {
            Some((_left_bracket, args, right_bracket)) => {
                (args, (name.first().unwrap(), &right_bracket).into())
            }
            None => (
                Vec::new(),
                (name.first().unwrap(), name.last().unwrap()).into(),
            ),
        };
        puppet_lang::typing::TypeSpecificationVariant::ExternalType(ExternalType {
            name: name.iter().map(|v| v.to_string()).collect(),
            arguments,
            extra,
        })
    })(input)
}

pub fn parse_type_specification(
    input: Span,
) -> IResult<puppet_lang::typing::TypeSpecification<Range>> {
    let parse_variant = map(
        pair(
            tag("Variant"),
            crate::common::square_brackets_comma_separated1(parse_type_specification),
        ),
        |(tag_kw, (_left_bracket, list, right_bracket))| {
            puppet_lang::typing::TypeSpecificationVariant::Variant(puppet_lang::typing::Variant {
                list,
                extra: (tag_kw, right_bracket).into(),
            })
        },
    );

    let parse_enum = map(
        pair(
            tag("Enum"),
            crate::common::square_brackets_comma_separated1(crate::term::parse_term),
        ),
        |(tag_kw, (_left_bracket, list, right_bracket))| {
            puppet_lang::typing::TypeSpecificationVariant::Enum(puppet_lang::typing::Enum {
                list,
                extra: (tag_kw, right_bracket).into(),
            })
        },
    );

    let parse_pattern = map(
        pair(
            tag("Pattern"),
            crate::common::square_brackets_comma_separated1(crate::regex::parse),
        ),
        |(tag_kw, (_left_bracket, list, right_bracket))| {
            puppet_lang::typing::TypeSpecificationVariant::Pattern(puppet_lang::typing::Pattern {
                list,
                extra: (tag_kw, right_bracket).into(),
            })
        },
    );

    let parse_regex = map(
        pair(
            tag("Regex"),
            crate::common::square_brackets_delimimited(crate::regex::parse),
        ),
        |(tag_kw, (_left_bracket, data, right_bracket))| {
            puppet_lang::typing::TypeSpecificationVariant::Regex(puppet_lang::typing::Regex {
                data,
                extra: (tag_kw, right_bracket).into(),
            })
        },
    );

    let parse_numeric = map(tag("Numeric"), |kw| {
        puppet_lang::typing::TypeSpecificationVariant::Numeric(puppet_lang::typing::Numeric {
            extra: (kw, kw).into(),
        })
    });

    let parse_boolean = map(tag("Boolean"), |kw| {
        puppet_lang::typing::TypeSpecificationVariant::Boolean(puppet_lang::typing::Boolean {
            extra: (kw, kw).into(),
        })
    });

    let parse_undef = map(tag("Undef"), |kw| {
        puppet_lang::typing::TypeSpecificationVariant::Undef(puppet_lang::typing::Undef {
            extra: (kw, kw).into(),
        })
    });

    let parse_any = map(tag("Any"), |kw| {
        puppet_lang::typing::TypeSpecificationVariant::Any(puppet_lang::typing::Any {
            extra: (kw, kw).into(),
        })
    });

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
        parse_regex,
        parse_undef,
        parse_any,
        parse_external_type,
    ));

    map(parser, |data| puppet_lang::typing::TypeSpecification {
        extra: (&data).into(),
        data,
    })(input)
}

#[test]
fn test_float() {
    assert_eq!(
        parse_float(Span::new("Float")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min: None,
            max: None,
            extra: Range::new(0, 1, 1, 4, 1, 5)
        })
    );
    assert_eq!(
        parse_float(Span::new("Float[ 100.0 ]")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min: Some(puppet_lang::expression::Float {
                value: 100.0,
                extra: Range::new(7, 1, 8, 11, 1, 12)
            }),
            max: None,
            extra: Range::new(0, 1, 1, 13, 1, 14)
        })
    );
    assert_eq!(
        parse_float(Span::new("Float[ 100.0, 200.0 ]")).unwrap().1,
        puppet_lang::typing::TypeSpecificationVariant::Float(puppet_lang::typing::TypeFloat {
            min: Some(puppet_lang::expression::Float {
                value: 100.0,
                extra: Range::new(7, 1, 8, 11, 1, 12)
            }),
            max: Some(puppet_lang::expression::Float {
                value: 200.0,
                extra: Range::new(14, 1, 15, 18, 1, 19)
            }),
            extra: Range::new(0, 1, 1, 20, 1, 21)
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
            extra: Range::new(0, 1, 1, 1, 1, 1),
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
            extra: Range::new(0, 1, 1, 1, 1, 1),
            inner: Some(Box::new(puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::String(
                    puppet_lang::typing::TypeString {
                        min: Some(puppet_lang::expression::Usize {
                            value: 1,
                            extra: Range::new(15, 1, 16, 1, 1, 1)
                        }),
                        max: Some(puppet_lang::expression::Usize {
                            value: 2,
                            extra: Range::new(17, 1, 18, 1, 1, 1)
                        }),
                        extra: Range::new(7, 1, 8, 1, 1, 1)
                    }
                ),
                extra: Range::new(7, 1, 8, 1, 1, 1)
            })),
            min: Some(puppet_lang::expression::Usize {
                value: 10,
                extra: Range::new(21, 1, 22, 1, 1, 1)
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
            extra: Range::new(0, 1, 1, 1, 1, 1),
            key: Some(Box::new(puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::String(
                    puppet_lang::typing::TypeString {
                        min: Some(puppet_lang::expression::Usize {
                            value: 1,
                            extra: Range::new(13, 1, 14, 1, 1, 1)
                        }),
                        max: Some(puppet_lang::expression::Usize {
                            value: 2,
                            extra: Range::new(15, 1, 16, 1, 1, 1)
                        }),
                        extra: Range::new(6, 1, 7, 1, 1, 1)
                    }
                ),
                extra: Range::new(6, 1, 7, 1, 1, 1)
            })),
            value: Some(Box::new(puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::Boolean(
                    puppet_lang::typing::Boolean {
                        extra: Range::new(20, 1, 21, 1, 1, 1)
                    }
                ),
                extra: Range::new(20, 1, 21, 1, 1, 1)
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
                extra: Range::new(0, 1, 1, 1, 1, 1),
                value: puppet_lang::typing::TypeOptionalVariant::TypeSpecification(Box::new(
                    puppet_lang::typing::TypeSpecification {
                        data: puppet_lang::typing::TypeSpecificationVariant::String(
                            puppet_lang::typing::TypeString {
                                min: Some(puppet_lang::expression::Usize {
                                    value: 1,
                                    extra: Range::new(17, 1, 18, 1, 1, 1)
                                }),
                                max: Some(puppet_lang::expression::Usize {
                                    value: 2,
                                    extra: Range::new(19, 1, 20, 1, 1, 1)
                                }),
                                extra: Range::new(10, 1, 11, 1, 1, 1)
                            }
                        ),
                        extra: Range::new(10, 1, 11, 1, 1, 1)
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
            extra: Range::new(0, 1, 1, 1, 1, 1),
            keys: vec![(
                puppet_lang::typing::TypeStructKey::String(puppet_lang::string::StringExpr {
                    data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                        puppet_lang::string::StringFragment::Literal(
                            puppet_lang::string::Literal {
                                data: "some_key".to_owned(),
                                extra: Range::new(9, 1, 10, 1, 1, 1)
                            }
                        )
                    ]),
                    accessor: None,
                    extra: Range::new(9, 1, 10, 1, 1, 1)
                }),
                puppet_lang::typing::TypeSpecification {
                    data: puppet_lang::typing::TypeSpecificationVariant::Boolean(
                        puppet_lang::typing::Boolean {
                            extra: Range::new(21, 1, 22, 1, 1, 1)
                        }
                    ),
                    extra: Range::new(21, 1, 22, 1, 1, 1)
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
            extra: Range::new(0, 1, 1, 1, 1, 1),
            list: vec![puppet_lang::typing::TypeSpecification {
                data: puppet_lang::typing::TypeSpecificationVariant::Integer(
                    puppet_lang::typing::TypeInteger {
                        min: Some(puppet_lang::expression::Integer {
                            value: 1,
                            extra: Range::new(15, 1, 16, 1, 1, 1)
                        }),
                        max: Some(puppet_lang::expression::Integer {
                            value: 2,
                            extra: Range::new(17, 1, 18, 1, 1, 1)
                        }),
                        extra: Range::new(7, 1, 8, 1, 1, 1)
                    }
                ),
                extra: Range::new(7, 1, 8, 1, 1, 1)
            }],
            min: Some(puppet_lang::expression::Usize {
                value: 10,
                extra: Range::new(21, 1, 22, 1, 1, 1)
            }),
            max: Some(puppet_lang::expression::Usize {
                value: 100,
                extra: Range::new(25, 1, 26, 1, 1, 1)
            }),
        })
    );
    assert_eq!(
        parse_tuple(Span::new("Tuple [Integer[1,2], Integer[3,4] ]"))
            .unwrap()
            .1,
        puppet_lang::typing::TypeSpecificationVariant::Tuple(puppet_lang::typing::TypeTuple {
            extra: Range::new(0, 1, 1, 1, 1, 1),
            list: vec![
                puppet_lang::typing::TypeSpecification {
                    data: puppet_lang::typing::TypeSpecificationVariant::Integer(
                        puppet_lang::typing::TypeInteger {
                            min: Some(puppet_lang::expression::Integer {
                                value: 1,
                                extra: Range::new(15, 1, 16, 1, 1, 1)
                            }),
                            max: Some(puppet_lang::expression::Integer {
                                value: 2,
                                extra: Range::new(17, 1, 18, 1, 1, 1)
                            }),
                            extra: Range::new(7, 1, 8, 1, 1, 1)
                        }
                    ),
                    extra: Range::new(7, 1, 8, 1, 1, 1)
                },
                puppet_lang::typing::TypeSpecification {
                    data: puppet_lang::typing::TypeSpecificationVariant::Integer(
                        puppet_lang::typing::TypeInteger {
                            min: Some(puppet_lang::expression::Integer {
                                value: 3,
                                extra: Range::new(29, 1, 30, 1, 1, 1)
                            }),
                            max: Some(puppet_lang::expression::Integer {
                                value: 4,
                                extra: Range::new(31, 1, 32, 1, 1, 1)
                            }),
                            extra: Range::new(21, 1, 22, 1, 1, 1)
                        }
                    ),
                    extra: Range::new(21, 1, 22, 1, 1, 1)
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
                    extra: Range::new(0, 1, 1, 1, 1, 1)
                }
            ),
            extra: Range::new(0, 1, 1, 1, 1, 1)
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
                    arguments: vec![puppet_lang::expression::Expression {
                        value: puppet_lang::expression::ExpressionVariant::Term(
                            puppet_lang::expression::Term {
                                value: puppet_lang::expression::TermVariant::String(
                                    puppet_lang::string::StringExpr {
                                        data: puppet_lang::string::StringVariant::SingleQuoted(
                                            vec![puppet_lang::string::StringFragment::Literal(
                                                puppet_lang::string::Literal {
                                                    data: "hello".to_owned(),
                                                    extra: Range::new(9, 1, 10, 1, 1, 1)
                                                }
                                            )]
                                        ),
                                        accessor: None,
                                        extra: Range::new(6, 1, 7, 1, 1, 1),
                                    }
                                ),
                                extra: Range::new(6, 1, 7, 1, 1, 1),
                            }
                        ),
                        extra: Range::new(6, 1, 7, 1, 1, 1),
                    }],
                    extra: Range::new(0, 1, 1, 1, 1, 1)
                }
            ),
            extra: Range::new(0, 1, 1, 1, 1, 1)
        }
    );
    assert_eq!(
        parse_type_specification(Span::new("Numeric")).unwrap().1,
        puppet_lang::typing::TypeSpecification {
            data: puppet_lang::typing::TypeSpecificationVariant::Numeric(
                puppet_lang::typing::Numeric {
                    extra: Range::new(0, 1, 1, 1, 1, 1)
                }
            ),
            extra: Range::new(0, 1, 1, 1, 1, 1)
        }
    );
    assert!(parse_type_specification(Span::new("Pattern[//, /sdfsdf/]")).is_ok());
    assert!(parse_type_specification(Span::new("Regexp[/sdfsdf/]")).is_ok());
}
