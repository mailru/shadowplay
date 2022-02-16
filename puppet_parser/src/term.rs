use crate::common::{
    round_brackets_delimimited, space0_delimimited, square_brackets_comma_separated1,
};
use crate::parser::Location;
use crate::parser::{IResult, ParseError, Span};
use nom::combinator::{map_res, value};
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
};
use puppet_lang::expression::StringExpr;

pub fn parse_variable(input: Span) -> IResult<puppet_lang::expression::Variable<Location>> {
    let accessor_parser = many0(square_brackets_comma_separated1(
        crate::expression::parse_expression,
    ));

    map(
        pair(
            preceded(tag("$"), crate::identifier::identifier_with_toplevel),
            accessor_parser,
        ),
        |(identifier, accessor)| puppet_lang::expression::Variable {
            extra: identifier.extra.clone(),
            identifier,
            accessor,
        },
    )(input)
}

pub fn parse_regexp_group_id(
    input: Span,
) -> IResult<puppet_lang::expression::RegexpGroupID<Location>> {
    map(
        preceded(tag("$"), map_res(digit1, |s: Span| s.parse::<u64>())),
        |identifier| puppet_lang::expression::RegexpGroupID {
            extra: Location::from(input),
            identifier,
        },
    )(input)
}

pub fn parse_lambda(input: Span) -> IResult<puppet_lang::expression::Lambda<Location>> {
    map(
        pair(
            crate::common::pipes_comma_separated0(crate::argument::parse),
            space0_delimimited(ParseError::protect(
                |_| "'{' expected".to_string(),
                crate::statement::parse_statement_set,
            )),
        ),
        |(args, body)| puppet_lang::expression::Lambda { args, body },
    )(input)
}

fn parse_funcall(input: Span) -> IResult<puppet_lang::expression::FunctionCall<Location>> {
    map(
        tuple((
            crate::identifier::anycase_identifier_with_ns,
            space0_delimimited(crate::common::round_brackets_comma_separated0(
                crate::expression::parse_expression,
            )),
            opt(space0_delimimited(parse_lambda)),
        )),
        |(identifier, args, lambda)| puppet_lang::expression::FunctionCall {
            extra: identifier.extra.clone(),
            identifier,
            args,
            lambda,
        },
    )(input)
}

pub fn parse_float(input: Span) -> IResult<f32> {
    let number = delimited(digit1, alt((tag("e"), tag("E"), tag("."))), digit1);
    let (tail, s) = recognize(pair(opt(tag("-")), number))(input)?;

    let f = match s.parse::<f32>() {
        Ok(v) => v,
        Err(err) => return ParseError::fatal(format!("{}", err), input),
    };

    Ok((tail, f))
}

pub fn parse_float_term(input: Span) -> IResult<puppet_lang::expression::Float<Location>> {
    map(parse_float, |value| puppet_lang::expression::Float {
        value,
        extra: Location::from(input),
    })(input)
}

pub fn parse_integer(input: Span) -> IResult<i64> {
    let (tail, s) = recognize(pair(opt(tag("-")), digit1))(input)?;

    let v = match s.parse::<i64>() {
        Ok(v) => v,
        Err(err) => return ParseError::fatal(format!("{}", err), input),
    };

    Ok((tail, v))
}

pub fn parse_integer_term(input: Span) -> IResult<puppet_lang::expression::Integer<Location>> {
    map(parse_integer, |value| puppet_lang::expression::Integer {
        value,
        extra: Location::from(input),
    })(input)
}

pub fn parse_usize(input: Span) -> IResult<usize> {
    let (tail, s) = digit1(input)?;

    let v = match s.parse::<usize>() {
        Ok(v) => v,
        Err(err) => return ParseError::fatal(format!("{}", err), input),
    };

    Ok((tail, v))
}

pub fn parse_usize_term(input: Span) -> IResult<puppet_lang::expression::Usize<Location>> {
    map(parse_usize, |value| puppet_lang::expression::Usize {
        value,
        extra: Location::from(input),
    })(input)
}

pub fn parse_sensitive(input: Span) -> IResult<puppet_lang::expression::TermVariant<Location>> {
    preceded(
        tag("Sensitive"),
        map(
            ParseError::protect(
                |_| "Expected round brackets after Sensitive value".to_string(),
                round_brackets_delimimited(ParseError::protect(
                    |_| "Expected term".to_string(),
                    parse_term,
                )),
            ),
            |value| {
                puppet_lang::expression::TermVariant::Sensitive(
                    puppet_lang::expression::Sensitive {
                        value: Box::new(value),
                        extra: Location::from(input),
                    },
                )
            },
        ),
    )(input)
}

fn parse_map(input: Span) -> IResult<puppet_lang::expression::TermVariant<Location>> {
    let kv_parser = pair(
        space0_delimimited(crate::expression::parse_expression),
        preceded(
            tag("=>"),
            space0_delimimited(ParseError::protect(
                |_| "Expression expected after '=>'".to_string(),
                crate::expression::parse_expression,
            )),
        ),
    );

    map(
        crate::common::curly_brackets_comma_separated0(kv_parser),
        puppet_lang::expression::TermVariant::Map,
    )(input)
}

pub fn parse_string_variant(input: Span) -> IResult<StringExpr<Location>> {
    alt((crate::double_quoted::parse, crate::single_quoted::parse))(input)
}

pub fn parse_term(input: Span) -> IResult<puppet_lang::expression::Term<Location>> {
    let parse_undef = value(
        puppet_lang::expression::TermVariant::Undef(puppet_lang::expression::Undef {
            extra: Location::from(input),
        }),
        tag("undef"),
    );

    let parse_true = value(
        puppet_lang::expression::TermVariant::Boolean(puppet_lang::expression::Boolean {
            value: true,
            extra: Location::from(input),
        }),
        tag("true"),
    );

    let parse_false = value(
        puppet_lang::expression::TermVariant::Boolean(puppet_lang::expression::Boolean {
            value: false,
            extra: Location::from(input),
        }),
        tag("false"),
    );

    let parse_type_specification = map(crate::typing::parse_type_specification, |v| {
        puppet_lang::expression::TermVariant::TypeSpecitifaction(v)
    });

    let parser = alt((
        parse_undef,
        parse_true,
        parse_false,
        parse_sensitive,
        map(
            parse_float_term,
            puppet_lang::expression::TermVariant::Float,
        ),
        map(
            parse_integer_term,
            puppet_lang::expression::TermVariant::Integer,
        ),
        map(parse_funcall, |v| {
            puppet_lang::expression::TermVariant::FunctionCall(v)
        }),
        parse_type_specification,
        map(
            parse_string_variant,
            puppet_lang::expression::TermVariant::String,
        ),
        map(
            crate::common::square_brackets_comma_separated0(crate::expression::parse_expression),
            puppet_lang::expression::TermVariant::Array,
        ),
        map(
            crate::common::round_brackets_delimimited(crate::expression::parse_expression),
            |v| puppet_lang::expression::TermVariant::Parens(Box::new(v)),
        ),
        parse_map,
        map(
            parse_variable,
            puppet_lang::expression::TermVariant::Variable,
        ),
        map(
            parse_regexp_group_id,
            puppet_lang::expression::TermVariant::RegexpGroupID,
        ),
        map(
            crate::regex::parse,
            puppet_lang::expression::TermVariant::Regexp,
        ),
    ));

    map(parser, |value| puppet_lang::expression::Term {
        value,
        extra: Location::from(input),
    })(input)
}

#[test]
fn test_single_quoted() {
    assert_eq!(
        parse_term(Span::new("'aaa'")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::String(
                puppet_lang::expression::StringExpr {
                    data: "aaa".to_owned(),
                    variant: puppet_lang::expression::StringVariant::SingleQuoted,
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    )
}

#[test]
fn test_array_of_types() {
    assert_eq!(
        parse_term(Span::new("[ Class['some_class'] ]"))
            .unwrap()
            .1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Array(vec![
                puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::TypeSpecitifaction(
                                puppet_lang::typing::TypeSpecification {
                                    data:
                                        puppet_lang::typing::TypeSpecificationVariant::ExternalType(
                                            puppet_lang::typing::ExternalType {
                                                name: vec!["Class".to_owned()],
                                                arguments: vec![puppet_lang::expression::Term {
                                                    value: puppet_lang::expression::TermVariant::String(puppet_lang::expression::StringExpr {
                                                        data: "some_class".to_owned(),
                                                        variant: puppet_lang::expression::StringVariant::SingleQuoted,
                                                        extra: Location::new(8, 1, 9)
                                                    }),
                                                    extra: Location::new(8, 1, 9)
                                                }],
                                                extra: Location::new(2, 1, 3)
                                            }
                                        ),
                                    extra: Location::new(2, 1, 3)
                                }
                            ),
                            extra: Location::new(2, 1, 3)
                        }
                    ),
                    extra: Location::new(2, 1, 3)
                }
            ]),
            extra: Location::new(0, 1, 1)
        }
    )
}

#[test]
fn test_double_quoted() {
    assert_eq!(
        parse_term(Span::new("\"aaa\"")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::String(
                puppet_lang::expression::StringExpr {
                    data: "aaa".to_owned(),
                    variant: puppet_lang::expression::StringVariant::DoubleQuoted,
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    )
}

#[test]
fn test_numbers() {
    assert_eq!(
        parse_term(Span::new("12345")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Integer(
                puppet_lang::expression::Integer {
                    extra: Location::new(0, 1, 1),
                    value: 12345
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse_term(Span::new("12345.1")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Float(puppet_lang::expression::Float {
                value: 12345.1,
                extra: Location::new(0, 1, 1)
            }),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse_term(Span::new("-12345.3")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Float(puppet_lang::expression::Float {
                value: -12345.3,
                extra: Location::new(0, 1, 1)
            }),
            extra: Location::new(0, 1, 1)
        }
    );
}

#[test]
fn test_bool() {
    assert_eq!(
        parse_term(Span::new("true")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Boolean(
                puppet_lang::expression::Boolean {
                    value: true,
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse_term(Span::new("false")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Boolean(
                puppet_lang::expression::Boolean {
                    value: false,
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
}

#[test]
fn test_array() {
    assert_eq!(
        parse_term(Span::new("[]")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Array(vec![]),
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        parse_term(Span::new("[false]")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Array(vec![
                puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Boolean(
                                puppet_lang::expression::Boolean {
                                    value: false,
                                    extra: Location::new(1, 1, 2)
                                }
                            ),
                            extra: Location::new(1, 1, 2)
                        }
                    ),
                    extra: Location::new(1, 1, 2)
                }
            ]),
            extra: Location::new(0, 1, 1)
        }
    );
}

#[test]
fn test_map() {
    assert_eq!(
        parse_term(Span::new("{}")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Map(vec![]),
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        parse_term(Span::new("{false => 1}")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Map(vec![(
                puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Boolean(
                                puppet_lang::expression::Boolean {
                                    value: false,
                                    extra: Location::new(1, 1, 2)
                                }
                            ),
                            extra: Location::new(1, 1, 2)
                        }
                    ),
                    extra: Location::new(1, 1, 2)
                },
                puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Integer(
                                puppet_lang::expression::Integer {
                                    value: 1,
                                    extra: Location::new(10, 1, 11)
                                }
                            ),
                            extra: Location::new(10, 1, 11)
                        }
                    ),
                    extra: Location::new(10, 1, 11)
                },
            )]),
            extra: Location::new(0, 1, 1)
        }
    );

    assert!(parse_term(Span::new("{'asdasd' => {}, 'a' => 'b', }")).is_ok());
}

#[test]
fn test_function_call() {
    assert_eq!(
        parse_term(Span::new("lookup('ask8s::docker::gpu_nvidia')"))
            .unwrap()
            .1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::FunctionCall(
                puppet_lang::expression::FunctionCall {
                    identifier: puppet_lang::identifier::LowerIdentifier {
                        name: vec!["lookup".to_owned()],
                        is_toplevel: false,
                        extra: Location::new(0, 1, 1)
                    },
                    args: vec![puppet_lang::expression::Expression {
                        value: puppet_lang::expression::ExpressionVariant::Term(
                            puppet_lang::expression::Term {
                                value: puppet_lang::expression::TermVariant::String(
                                    puppet_lang::expression::StringExpr {
                                        extra: Location::new(7, 1, 8),
                                        data: "ask8s::docker::gpu_nvidia".to_owned(),
                                        variant:
                                            puppet_lang::expression::StringVariant::SingleQuoted,
                                    }
                                ),
                                extra: Location::new(7, 1, 8)
                            }
                        ),
                        extra: Location::new(7, 1, 8)
                    },],
                    lambda: None,
                    extra: Location::new(0, 1, 1)
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
}

#[test]
fn test_variable() {
    assert_eq!(
        parse_variable(Span::new("$a")).unwrap().1,
        puppet_lang::expression::Variable {
            identifier: puppet_lang::identifier::LowerIdentifier {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                extra: Location::new(1, 1, 2)
            },
            accessor: vec![],
            extra: Location::new(1, 1, 2)
        }
    );
    assert_eq!(
        parse_variable(Span::new("$::a::b")).unwrap().1,
        puppet_lang::expression::Variable {
            identifier: puppet_lang::identifier::LowerIdentifier {
                name: vec!["a".to_owned(), "b".to_owned()],
                is_toplevel: true,
                extra: Location::new(1, 1, 2)
            },
            accessor: vec![],
            extra: Location::new(1, 1, 2)
        }
    );
    assert_eq!(
        parse_variable(Span::new("$a[ 1 ]['z']")).unwrap().1,
        puppet_lang::expression::Variable {
            identifier: puppet_lang::identifier::LowerIdentifier {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                extra: Location::new(1, 1, 2)
            },
            accessor: vec![
                vec![puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Integer(
                                puppet_lang::expression::Integer {
                                    extra: Location::new(4, 1, 5),
                                    value: 1,
                                }
                            ),
                            extra: Location::new(4, 1, 5)
                        }
                    ),
                    extra: Location::new(4, 1, 5)
                }],
                vec![puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::String(
                                puppet_lang::expression::StringExpr {
                                    extra: Location::new(8, 1, 9),
                                    data: "z".to_owned(),
                                    variant: puppet_lang::expression::StringVariant::SingleQuoted
                                }
                            ),
                            extra: Location::new(8, 1, 9)
                        }
                    ),
                    extra: Location::new(8, 1, 9)
                }]
            ],
            extra: Location::new(1, 1, 2)
        }
    );
}
