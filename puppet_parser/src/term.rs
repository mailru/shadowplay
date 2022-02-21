use crate::common::{
    comma_separator, round_brackets_delimimited, space0_delimimited,
    square_brackets_comma_separated1,
};
use crate::parser::Location;
use crate::parser::{IResult, ParseError, Span};
use nom::combinator::{map_res, value};
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
};
use puppet_lang::expression::StringExpr;

pub fn parse_accessor(
    input: Span,
) -> IResult<Vec<Vec<Box<puppet_lang::expression::Expression<Location>>>>> {
    many0(square_brackets_comma_separated1(map(
        crate::expression::parse_expression,
        Box::new,
    )))(input)
}

pub fn parse_variable(input: Span) -> IResult<puppet_lang::expression::Variable<Location>> {
    map(
        pair(
            preceded(tag("$"), crate::identifier::identifier_with_toplevel),
            parse_accessor,
        ),
        |(identifier, accessor)| puppet_lang::expression::Variable {
            extra: identifier.extra.clone(),
            accessor,
            identifier,
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
        tuple((
            space0_delimimited(tag("{")),
            terminated(
                separated_list0(comma_separator, kv_parser),
                opt(space0_delimimited(tag(","))),
            ),
            space0_delimimited(tag("}")),
            parse_accessor,
        )),
        |(tag_left, value, _tag_right_, accessor)| {
            puppet_lang::expression::TermVariant::Map(puppet_lang::expression::Map {
                extra: Location::from(tag_left),
                value,
                accessor,
            })
        },
    )(input)
}

pub fn parse_string_variant(input: Span) -> IResult<StringExpr<Location>> {
    alt((crate::double_quoted::parse, crate::single_quoted::parse))(input)
}

pub fn parse_parens(input: Span) -> IResult<puppet_lang::expression::Parens<Location>> {
    map(
        tuple((
            space0_delimimited(tag("(")),
            crate::expression::parse_expression,
            space0_delimimited(tag(")")),
            parse_accessor,
        )),
        |(left_tag, value, _right_tag, accessor)| puppet_lang::expression::Parens {
            value: Box::new(value),
            accessor,
            extra: Location::from(left_tag),
        },
    )(input)
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
        parse_type_specification,
        map(
            crate::identifier::identifier_with_toplevel,
            puppet_lang::expression::TermVariant::Identifier,
        ),
        map(
            parse_string_variant,
            puppet_lang::expression::TermVariant::String,
        ),
        map(
            crate::common::square_brackets_comma_separated0(crate::expression::parse_expression),
            puppet_lang::expression::TermVariant::Array,
        ),
        map(parse_parens, puppet_lang::expression::TermVariant::Parens),
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
                    accessor: Vec::new(),
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
                                                        accessor: Vec::new(),
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
                    accessor: Vec::new(),
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
            value: puppet_lang::expression::TermVariant::Map(puppet_lang::expression::Map {
                value: Vec::new(),
                accessor: Vec::new(),
                extra: Location::new(0, 1, 1)
            }),
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        parse_term(Span::new("{false => 1}")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Map(puppet_lang::expression::Map {
                value: vec![(
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
                )],
                accessor: Vec::new(),
                extra: Location::new(0, 1, 1)
            }),
            extra: Location::new(0, 1, 1)
        }
    );

    assert!(parse_term(Span::new("{'asdasd' => {}, 'a' => 'b', }")).is_ok());
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
            accessor: Vec::new(),
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
            accessor: Vec::new(),
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
                vec![Box::new(puppet_lang::expression::Expression {
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
                })],
                vec![Box::new(puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::String(
                                puppet_lang::expression::StringExpr {
                                    extra: Location::new(8, 1, 9),
                                    data: "z".to_owned(),
                                    variant: puppet_lang::expression::StringVariant::SingleQuoted,
                                    accessor: Vec::new()
                                }
                            ),
                            extra: Location::new(8, 1, 9)
                        }
                    ),
                    extra: Location::new(8, 1, 9)
                })]
            ],
            extra: Location::new(1, 1, 2)
        }
    );
}
