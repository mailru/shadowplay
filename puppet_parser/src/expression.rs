use nom::{branch::alt, bytes::complete::tag, sequence::pair};

use crate::parser::Location;

use crate::parser::{IResult, ParseError, Span};

pub mod term {
    use crate::common::{round_brackets_delimimited, square_brackets_delimimited};
    use crate::parser::Location;
    use crate::parser::{IResult, ParseError, Span};
    use nom::combinator::value;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, opt, recognize},
        multi::many0,
        sequence::{delimited, pair, preceded},
    };

    pub fn parse_variable(input: Span) -> IResult<puppet_lang::expression::Variable<Location>> {
        let accessor_parser = many0(square_brackets_delimimited(
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

    fn parse_funcall(input: Span) -> IResult<puppet_lang::expression::FunctionCall<Location>> {
        map(
            pair(
                crate::identifier::identifier_with_toplevel,
                crate::common::round_brackets_comma_separated0(crate::expression::parse_expression),
            ),
            |(identifier, args)| puppet_lang::expression::FunctionCall {
                extra: identifier.extra.clone(),
                identifier,
                args,
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
                        |_| "Expected single quoted string".to_string(),
                        crate::single_quoted::parse,
                    )),
                ),
                |value| {
                    puppet_lang::expression::TermVariant::Sensitive(
                        puppet_lang::expression::Sensitive {
                            value,
                            extra: Location::from(input),
                        },
                    )
                },
            ),
        )(input)
    }

    fn parse_map(input: Span) -> IResult<puppet_lang::expression::TermVariant<Location>> {
        let kv_parser = pair(
            crate::common::space0_delimimited(crate::expression::parse_expression),
            preceded(
                tag("=>"),
                crate::common::space0_delimimited(crate::expression::parse_expression),
            ),
        );

        map(
            crate::common::curly_brackets_comma_separated0(kv_parser),
            puppet_lang::expression::TermVariant::Map,
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
            parse_type_specification,
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
            map(
                crate::double_quoted::parse,
                puppet_lang::expression::TermVariant::String,
            ),
            map(
                crate::single_quoted::parse,
                puppet_lang::expression::TermVariant::String,
            ),
            map(
                crate::common::square_brackets_comma_separated0(
                    crate::expression::parse_expression,
                ),
                puppet_lang::expression::TermVariant::Array,
            ),
            parse_map,
            map(
                parse_variable,
                puppet_lang::expression::TermVariant::Variable,
            ),
        ));

        map(parser, |value| puppet_lang::expression::Term {
            value,
            extra: Location::from(input),
        })(input)
    }
}

pub(crate) mod expr {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::pair};

    use crate::parser::{IResult, Location, ParseError, Span};

    pub(crate) fn fold_many0<'a, F, G, O, R>(
        mut f: F,
        init: R,
        g: G,
    ) -> impl FnMut(Span<'a>) -> IResult<R>
    where
        F: nom::Parser<Span<'a>, O, ParseError<'a>>,
        G: Fn(R, O) -> R,
        R: Clone,
    {
        let mut res = init;
        move |i: Span| {
            let mut input = i;

            loop {
                let i_ = input;
                let len = input.len();
                match f.parse(i_) {
                    Ok((i, o)) => {
                        // infinite loop check: the parser must always consume
                        if i.len() == len {
                            return Err(nom::Err::Error(ParseError::new(
                                "Parsed empty token in list".to_string(),
                                input,
                            )));
                        }

                        res = g(res.clone(), o);
                        input = i;
                    }
                    Err(nom::Err::Error(_)) => {
                        return Ok((input, res.clone()));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    pub(crate) fn parse_l1(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
        let (input, left_expr) =
            crate::common::space0_delimimited(map(crate::expression::term::parse_term, |term| {
                puppet_lang::expression::Expression {
                    extra: term.extra.clone(),
                    value: puppet_lang::expression::ExpressionVariant::Term(term),
                }
            }))(input)?;
        let mut parser = fold_many0(
            pair(
                alt((tag("*"), tag("/"))),
                crate::common::space0_delimimited(ParseError::protect(
                    |_| "Second argument of operator is expected".to_string(),
                    parse_l1,
                )),
            ),
            left_expr,
            |prev, (op, cur)| match *op {
                "*" => puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Multiply((
                        Box::new(prev),
                        Box::new(cur),
                    )),
                    extra: Location::from(op),
                },
                "/" => puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Divide((
                        Box::new(prev),
                        Box::new(cur),
                    )),
                    extra: Location::from(op),
                },
                _ => unreachable!(),
            },
        );
        parser(input)
    }
}

pub fn parse_expression(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = super::common::space0_delimimited(expr::parse_l1)(input)?;
    let mut parser = expr::fold_many0(
        pair(
            alt((tag("+"), tag("-"))),
            super::common::space0_delimimited(ParseError::protect(
                |_| "Second argument of operator is expected".to_string(),
                expr::parse_l1,
            )),
        ),
        left_expr,
        |prev, (op, cur)| match *op {
            "+" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Plus((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            "-" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Minus((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            _ => unreachable!(),
        },
    );
    parser(input)
}

#[test]
fn test_single_quoted() {
    assert_eq!(
        term::parse_term(Span::new("'aaa'")).unwrap().1,
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
fn test_double_quoted() {
    assert_eq!(
        term::parse_term(Span::new("\"aaa\"")).unwrap().1,
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
        term::parse_term(Span::new("12345")).unwrap().1,
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
        term::parse_term(Span::new("12345.1")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Float(puppet_lang::expression::Float {
                value: 12345.1,
                extra: Location::new(0, 1, 1)
            }),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        term::parse_term(Span::new("-12345.3")).unwrap().1,
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
        term::parse_term(Span::new("true")).unwrap().1,
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
        term::parse_term(Span::new("false")).unwrap().1,
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
        term::parse_term(Span::new("[]")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Array(vec![]),
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        term::parse_term(Span::new("[false]")).unwrap().1,
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
        term::parse_term(Span::new("{}")).unwrap().1,
        puppet_lang::expression::Term {
            value: puppet_lang::expression::TermVariant::Map(vec![]),
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        term::parse_term(Span::new("{false => 1}")).unwrap().1,
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

    assert!(term::parse_term(Span::new("{'asdasd' => {}, 'a' => 'b', }")).is_ok());
}

#[test]
fn test_function_call() {
    assert_eq!(
        term::parse_term(Span::new("lookup('ask8s::docker::gpu_nvidia')"))
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
        term::parse_variable(Span::new("$a")).unwrap().1,
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
        term::parse_variable(Span::new("$::a::b")).unwrap().1,
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
        term::parse_variable(Span::new("$a[ 1 ]['z']")).unwrap().1,
        puppet_lang::expression::Variable {
            identifier: puppet_lang::identifier::LowerIdentifier {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                extra: Location::new(1, 1, 2)
            },
            accessor: vec![
                puppet_lang::expression::Expression {
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
                },
                puppet_lang::expression::Expression {
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
                }
            ],
            extra: Location::new(1, 1, 2)
        }
    );
}

#[test]
fn test_multiply() {
    assert_eq!(
        parse_expression(Span::new("2*3")).unwrap().1,
        puppet_lang::expression::Expression {
            value: puppet_lang::expression::ExpressionVariant::Multiply((
                Box::new(puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Integer(
                                puppet_lang::expression::Integer {
                                    extra: Location::new(0, 1, 1),
                                    value: 2,
                                }
                            ),
                            extra: Location::new(0, 1, 1)
                        }
                    ),
                    extra: Location::new(0, 1, 1)
                }),
                Box::new(puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Integer(
                                puppet_lang::expression::Integer {
                                    extra: Location::new(2, 1, 3),
                                    value: 3,
                                }
                            ),
                            extra: Location::new(2, 1, 3)
                        }
                    ),
                    extra: Location::new(2, 1, 3)
                }),
            )),
            extra: Location::new(1, 1, 2)
        }
    );
}

#[test]
fn test_operators_precendence() {
    assert_eq!(
        parse_expression(Span::new("1 +2 * 3* 4 - 10")).unwrap().1,
        puppet_lang::expression::Expression {
            value: puppet_lang::expression::ExpressionVariant::Minus((
                Box::new(puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Plus((
                        Box::new(puppet_lang::expression::Expression {
                            value: puppet_lang::expression::ExpressionVariant::Term(
                                puppet_lang::expression::Term {
                                    value: puppet_lang::expression::TermVariant::Integer(
                                        puppet_lang::expression::Integer {
                                            extra: Location::new(0, 1, 1),
                                            value: 1,
                                        }
                                    ),
                                    extra: Location::new(0, 1, 1)
                                }
                            ),
                            extra: Location::new(0, 1, 1)
                        }),
                        Box::new(puppet_lang::expression::Expression {
                            value: puppet_lang::expression::ExpressionVariant::Multiply((
                                Box::new(puppet_lang::expression::Expression {
                                    value: puppet_lang::expression::ExpressionVariant::Term(
                                        puppet_lang::expression::Term {
                                            value: puppet_lang::expression::TermVariant::Integer(
                                                puppet_lang::expression::Integer {
                                                    extra: Location::new(3, 1, 4),
                                                    value: 2,
                                                }
                                            ),
                                            extra: Location::new(3, 1, 4)
                                        }
                                    ),
                                    extra: Location::new(3, 1, 4)
                                }),
                                Box::new(puppet_lang::expression::Expression {
                                    value: puppet_lang::expression::ExpressionVariant::Multiply((
                                        Box::new(puppet_lang::expression::Expression {
                                            value: puppet_lang::expression::ExpressionVariant::Term(
                                                puppet_lang::expression::Term {
                                                    value: puppet_lang::expression::TermVariant::Integer(
                                                        puppet_lang::expression::Integer {
                                                            extra: Location::new(7, 1, 8),
                                                            value: 3,
                                                        }
                                                    ),
                                                    extra: Location::new(7, 1, 8)
                                                }
                                            ),
                                            extra: Location::new(7, 1, 8)
                                        }),
                                        Box::new(puppet_lang::expression::Expression {
                                            value: puppet_lang::expression::ExpressionVariant::Term(
                                                puppet_lang::expression::Term {
                                                    value: puppet_lang::expression::TermVariant::Integer(
                                                        puppet_lang::expression::Integer {
                                                            extra: Location::new(10, 1, 11),
                                                            value: 4
                                                        }
                                                    ),
                                                    extra: Location::new(10, 1, 11)
                                                }
                                            ),
                                            extra: Location::new(10, 1, 11)
                                        }),
                                    )),
                                    extra: Location::new(8, 1, 9)
                                }),
                            )),
                            extra: Location::new(5, 1, 6)
                        }),
                    )),
                    extra: Location::new(2, 1, 3)
                }),
                Box::new(puppet_lang::expression::Expression {
                    value: puppet_lang::expression::ExpressionVariant::Term(
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Integer(
                                puppet_lang::expression::Integer {
                                    extra: Location::new(14, 1, 15),
                                    value: 10,
                                }
                            ),
                            extra: Location::new(14, 1, 15)
                        }
                    ),
                    extra: Location::new(14, 1, 15)
                }),
            )),
            extra: Location::new(12, 1, 13)
        }
    );
}
