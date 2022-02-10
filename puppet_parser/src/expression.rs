use nom::sequence::tuple;
///
/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_expressions-order-of-operations
///
use nom::{bytes::complete::tag, sequence::pair};

use crate::parser::Location;

use crate::parser::{IResult, Span};

use nom::{branch::alt, combinator::map};

use crate::parser::ParseError;

fn fold_many0<'a, F, G, O, R>(mut f: F, init: R, g: G) -> impl FnMut(Span<'a>) -> IResult<R>
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

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_comparison_operators-comparison-regex-or-data-type-match
pub(crate) fn parse_match_variant(
    input: Span,
) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, (left_term, tag_variant)) = pair(
        crate::common::space0_delimimited(crate::term::parse_term),
        alt((tag("=~"), tag("!~"))),
    )(input)?;

    let left_expr = puppet_lang::expression::Expression {
        extra: left_term.extra.clone(),
        value: puppet_lang::expression::ExpressionVariant::Term(left_term),
    };

    let parser_match_regex = map(crate::regex::parse, |regex| match *tag_variant {
        "=~" => puppet_lang::expression::ExpressionVariant::MatchRegex((
            Box::new(left_expr.clone()),
            regex,
        )),
        "!~" => puppet_lang::expression::ExpressionVariant::NotMatchRegex((
            Box::new(left_expr.clone()),
            regex,
        )),
        _ => unreachable!(),
    });

    let parser_match_type = map(
        crate::typing::parse_type_specification,
        |t| match *tag_variant {
            "=~" => puppet_lang::expression::ExpressionVariant::MatchType((
                Box::new(left_expr.clone()),
                Box::new(t),
            )),
            "!~" => puppet_lang::expression::ExpressionVariant::NotMatchType((
                Box::new(left_expr.clone()),
                Box::new(t),
            )),
            _ => unreachable!(),
        },
    );

    let r = alt((
        map(parser_match_regex, |value| {
            puppet_lang::expression::Expression {
                value,
                extra: Location::from(tag_variant),
            }
        }),
        map(parser_match_type, |value| {
            puppet_lang::expression::Expression {
                value,
                extra: Location::from(tag_variant),
            }
        }),
    ))(input);

    r
}

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_comparison_operators-comparison-in
fn parse_in_expr(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let parser = tuple((
        crate::term::parse_term,
        crate::common::space0_delimimited(tag("in")),
        crate::term::parse_term,
    ));

    map(parser, |(left, op, right)| {
        puppet_lang::expression::Expression {
            extra: Location::from(op),
            value: puppet_lang::expression::ExpressionVariant::In((left, right)),
        }
    })(input)
}

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_boolean-boolean-not
fn parse_not(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let parser = pair(
        crate::common::space0_delimimited(tag("!")),
        crate::term::parse_term,
    );

    map(parser, |(op, term)| puppet_lang::expression::Expression {
        extra: Location::from(op),
        value: puppet_lang::expression::ExpressionVariant::Not(term),
    })(input)
}

fn parse_l0(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    crate::common::space0_delimimited(alt((
        parse_not,
        parse_in_expr,
        parse_match_variant,
        map(crate::term::parse_term, |term| {
            puppet_lang::expression::Expression {
                extra: term.extra.clone(),
                value: puppet_lang::expression::ExpressionVariant::Term(term),
            }
        }),
    )))(input)
}

pub(crate) fn parse_l1(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = parse_l0(input)?;
    let mut parser = fold_many0(
        pair(
            alt((tag("*"), tag("/"), tag("%"))),
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
            "%" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Modulo((
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

fn parse_l2(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = crate::common::space0_delimimited(parse_l1)(input)?;
    let mut parser = fold_many0(
        pair(
            alt((tag("+"), tag("-"))),
            crate::common::space0_delimimited(ParseError::protect(
                |_| "Second argument of operator is expected".to_string(),
                parse_l1,
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

fn parse_l3(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = crate::common::space0_delimimited(parse_l2)(input)?;
    let mut parser = fold_many0(
        pair(
            alt((tag("<<"), tag(">>"))),
            crate::common::space0_delimimited(ParseError::protect(
                |_| "Second argument of operator is expected".to_string(),
                parse_l2,
            )),
        ),
        left_expr,
        |prev, (op, cur)| match *op {
            "<<" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::ShiftLeft((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            ">>" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::ShiftRight((
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

fn parse_l4(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = crate::common::space0_delimimited(parse_l3)(input)?;
    let mut parser = fold_many0(
        pair(
            alt((
                tag("=="),
                tag("!="),
                tag(">"),
                tag("<"),
                tag(">="),
                tag("<="),
            )),
            crate::common::space0_delimimited(ParseError::protect(
                |_| "Second argument of operator is expected".to_string(),
                parse_l3,
            )),
        ),
        left_expr,
        |prev, (op, cur)| match *op {
            "==" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Equal((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            "!=" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::NotEqual((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            ">" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Gt((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            ">=" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::GtEq((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            "<" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Lt((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            "<=" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::LtEq((
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

fn parse_l5(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = crate::common::space0_delimimited(parse_l4)(input)?;
    let mut parser = fold_many0(
        pair(
            alt((tag("and"), tag("or"))),
            crate::common::space0_delimimited(ParseError::protect(
                |_| "Second argument of operator is expected".to_string(),
                parse_l4,
            )),
        ),
        left_expr,
        |prev, (op, cur)| match *op {
            "and" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::And((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            "or" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Or((
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

pub fn parse_expression(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = super::common::space0_delimimited(parse_l5)(input)?;
    let mut parser = fold_many0(
        pair(tag("="), super::common::space0_delimimited(parse_l5)),
        left_expr,
        |prev, (op, cur)| match *op {
            "=" => puppet_lang::expression::Expression {
                value: puppet_lang::expression::ExpressionVariant::Assign((
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
