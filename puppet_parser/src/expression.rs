use nom::multi::separated_list1;
use nom::sequence::tuple;
///
/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_expressions-order-of-operations
///
use nom::{bytes::complete::tag, sequence::pair};

use crate::common::{comma_separator, fold_many0_with_const_init, space0_delimimited};
use crate::parser::Location;

use crate::parser::{IResult, Span};
use crate::term::parse_term;

use nom::{branch::alt, combinator::map};

use crate::parser::ParseError;

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_comparison_operators-comparison-regex-or-data-type-match
pub(crate) fn parse_match_variant(
    input: Span,
) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, (left_term, tag_variant)) = pair(
        space0_delimimited(crate::term::parse_term),
        space0_delimimited(alt((tag("=~"), tag("!~")))),
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

    let mut parser = alt((
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
    ));

    parser(input)
}

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_comparison_operators-comparison-in
fn parse_in_expr(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let parser = tuple((
        crate::term::parse_term,
        space0_delimimited(tag("in")),
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
    let parser = pair(space0_delimimited(tag("!")), parse_expression);

    map(parser, |(op, expr)| puppet_lang::expression::Expression {
        extra: Location::from(op),
        value: puppet_lang::expression::ExpressionVariant::Not(Box::new(expr)),
    })(input)
}

/// https://puppet.com/docs/puppet/7/lang_conditional.html#lang_condition_selector
fn parse_selector_case(input: Span) -> IResult<puppet_lang::expression::SelectorCase<Location>> {
    let parser = tuple((parse_term, space0_delimimited(tag("=>")), parse_expression));

    map(parser, |(case, tag, body)| {
        puppet_lang::expression::SelectorCase {
            case,
            body: Box::new(body),
            extra: Location::from(tag),
        }
    })(input)
}

/// https://puppet.com/docs/puppet/7/lang_conditional.html#lang_condition_selector
fn parse_selector(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let parser = tuple((
        space0_delimimited(parse_term),
        tag("?"),
        space0_delimimited(tag("{")),
        separated_list1(comma_separator, space0_delimimited(parse_selector_case)),
        space0_delimimited(tag("}")),
    ));

    map(parser, |(condition, op, _, cases, _)| {
        puppet_lang::expression::Expression {
            extra: Location::from(op),
            value: puppet_lang::expression::ExpressionVariant::Selector(
                puppet_lang::expression::Selector {
                    condition,
                    cases,
                    extra: Location::from(op),
                },
            ),
        }
    })(input)
}

fn parse_l0(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    space0_delimimited(alt((
        parse_not,
        parse_in_expr,
        parse_selector,
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
    let mut parser = fold_many0_with_const_init(
        pair(
            alt((tag("*"), tag("/"), tag("%"))),
            space0_delimimited(ParseError::protect(
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
    let (input, left_expr) = space0_delimimited(parse_l1)(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(
            alt((tag("+"), tag("-"))),
            space0_delimimited(ParseError::protect(
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
    let (input, left_expr) = space0_delimimited(parse_l2)(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(
            alt((tag("<<"), tag(">>"))),
            space0_delimimited(ParseError::protect(
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
    let (input, left_expr) = space0_delimimited(parse_l3)(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(
            alt((
                tag("=="),
                tag("!="),
                tag(">="),
                tag("<="),
                tag(">"),
                tag("<"),
            )),
            space0_delimimited(ParseError::protect(
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
    let (input, left_expr) = space0_delimimited(parse_l4)(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(
            alt((tag("and"), tag("or"))),
            space0_delimimited(ParseError::protect(
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
    let (input, left_expr) = space0_delimimited(parse_l5)(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(tag("="), space0_delimimited(parse_l5)),
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
    use puppet_lang::expression::Expression;
    use puppet_lang::expression::ExpressionVariant;
    use puppet_lang::expression::Integer;
    use puppet_lang::expression::Term;
    use puppet_lang::expression::TermVariant;
    assert_eq!(
        parse_expression(Span::new("(1 +2) * 3* 4 - 10")).unwrap().1,
        Expression {
            value: ExpressionVariant::Minus((
                Box::new(Expression {
                    value: ExpressionVariant::Multiply((
                        Box::new(Expression {
                            value: ExpressionVariant::Term(Term {
                                value: TermVariant::Parens(Box::new(Expression {
                                    value: ExpressionVariant::Plus((
                                        Box::new(Expression {
                                            value: ExpressionVariant::Term(Term {
                                                value: TermVariant::Integer(Integer {
                                                    value: 1,
                                                    extra: Location::new(1, 1, 2)
                                                }),
                                                extra: Location::new(1, 1, 2)
                                            }),
                                            extra: Location::new(1, 1, 2)
                                        }),
                                        Box::new(Expression {
                                            value: ExpressionVariant::Term(Term {
                                                value: TermVariant::Integer(Integer {
                                                    value: 2,
                                                    extra: Location::new(4, 1, 5)
                                                }),
                                                extra: Location::new(4, 1, 5)
                                            }),
                                            extra: Location::new(4, 1, 5)
                                        })
                                    )),
                                    extra: Location::new(3, 1, 4)
                                })),
                                extra: Location::new(0, 1, 1)
                            }),
                            extra: Location::new(0, 1, 1)
                        }),
                        Box::new(Expression {
                            value: ExpressionVariant::Multiply((
                                Box::new(Expression {
                                    value: ExpressionVariant::Term(Term {
                                        value: TermVariant::Integer(Integer {
                                            value: 3,
                                            extra: Location::new(9, 1, 10)
                                        }),
                                        extra: Location::new(9, 1, 10)
                                    }),
                                    extra: Location::new(9, 1, 10)
                                }),
                                Box::new(Expression {
                                    value: ExpressionVariant::Term(Term {
                                        value: TermVariant::Integer(Integer {
                                            value: 4,
                                            extra: Location::new(12, 1, 13)
                                        }),
                                        extra: Location::new(12, 1, 13)
                                    }),
                                    extra: Location::new(12, 1, 13)
                                })
                            )),
                            extra: Location::new(10, 1, 11)
                        })
                    )),
                    extra: Location::new(7, 1, 8)
                }),
                Box::new(Expression {
                    value: ExpressionVariant::Term(Term {
                        value: TermVariant::Integer(Integer {
                            value: 10,
                            extra: Location::new(16, 1, 17)
                        }),
                        extra: Location::new(16, 1, 17)
                    }),
                    extra: Location::new(16, 1, 17)
                })
            )),
            extra: Location::new(14, 1, 15)
        }
    );
}
