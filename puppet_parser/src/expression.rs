use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
///
/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_expressions-order-of-operations
///
use nom::{bytes::complete::tag, sequence::pair};
use puppet_lang::expression::CaseVariant;

use crate::common::{comma_separator, fold_many0_with_const_init, space0_delimimited, spaced_word};

use crate::term::{parse_accessor, parse_term};
use crate::{IResult, Location, ParseError, Span};

use nom::{branch::alt, combinator::map};

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

    let parser = alt((
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

    let mut parser = ParseError::protect(
        |_| "Regex or type specification expected after match operator".to_string(),
        parser,
    );

    parser(input)
}

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_boolean-boolean-not
fn parse_not(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let parser = pair(space0_delimimited(tag("!")), parse_expression);

    map(parser, |(op, expr)| puppet_lang::expression::Expression {
        extra: Location::from(op),
        value: puppet_lang::expression::ExpressionVariant::Not(Box::new(expr)),
    })(input)
}

pub fn parse_case_variant(input: Span) -> IResult<CaseVariant<Location>> {
    map(parse_term, |t| {
        if matches!(
            &t.value,
            puppet_lang::expression::TermVariant::String(s) if puppet_ast_tool::string::raw_content(s) == "default"
        ) {
            CaseVariant::Default(puppet_lang::expression::Default { extra: t.extra })
        } else {
            CaseVariant::Term(t)
        }
    })(input)
}

fn parse_term_expr(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    map(crate::term::parse_term, |term| {
        puppet_lang::expression::Expression {
            extra: term.extra.clone(),
            value: puppet_lang::expression::ExpressionVariant::Term(term),
        }
    })(input)
}

pub fn parse_lambda(input: Span) -> IResult<puppet_lang::expression::Lambda<Location>> {
    map(
        pair(
            crate::common::pipes_comma_separated0(crate::argument::parse),
            space0_delimimited(ParseError::protect(
                |_| "'{' expected".to_string(),
                crate::statement::parse_statement_block,
            )),
        ),
        |(args, body)| puppet_lang::expression::Lambda { args, body },
    )(input)
}

fn parse_funcall(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    map(
        tuple((
            crate::identifier::anycase_identifier_with_ns,
            space0_delimimited(crate::common::round_brackets_comma_separated0(
                crate::expression::parse_expression,
            )),
            opt(space0_delimimited(parse_lambda)),
            parse_accessor,
        )),
        |(identifier, args, lambda, accessor)| puppet_lang::expression::Expression {
            extra: identifier.extra.clone(),
            value: puppet_lang::expression::ExpressionVariant::FunctionCall(
                puppet_lang::expression::FunctionCall {
                    extra: identifier.extra.clone(),
                    identifier,
                    args,
                    lambda,
                    accessor,
                },
            ),
        },
    )(input)
}

fn parse_l0(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    space0_delimimited(alt((
        parse_not,
        parse_match_variant,
        parse_funcall,
        parse_term_expr,
    )))(input)
}

fn parse_chain_call_right(input: Span) -> IResult<puppet_lang::expression::FunctionCall<Location>> {
    let parse_just_identifier = map(crate::identifier::lowercase_identifier, |identifier| {
        puppet_lang::identifier::LowerIdentifier {
            extra: Location::from(identifier),
            name: vec![identifier.to_string()],
            is_toplevel: false,
        }
    });

    map(
        tuple((
            parse_just_identifier,
            opt(space0_delimimited(
                crate::common::round_brackets_comma_separated0(crate::expression::parse_expression),
            )),
            opt(space0_delimimited(parse_lambda)),
            parse_accessor,
        )),
        |(identifier, args, lambda, accessor)| puppet_lang::expression::FunctionCall {
            extra: identifier.extra.clone(),
            identifier,
            args: args.unwrap_or_default(),
            lambda,
            accessor,
        },
    )(input)
}

/// https://puppet.com/docs/puppet/7/lang_functions.html#chained-function-calls
fn parse_chain_call(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = parse_l0(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(
            space0_delimimited(tag(".")),
            ParseError::protect(
                |_| "Second argument of chain operator is expected".to_string(),
                parse_chain_call_right,
            ),
        ),
        left_expr,
        |left, (op, right)| puppet_lang::expression::Expression {
            value: puppet_lang::expression::ExpressionVariant::ChainCall(
                puppet_lang::expression::ChainCall {
                    left: Box::new(left),
                    right: Box::new(right),
                    extra: Location::from(op),
                },
            ),
            extra: Location::from(op),
        },
    );
    parser(input)
}

/// https://puppet.com/docs/puppet/6/lang_expressions.html#lang_exp_comparison_operators-comparison-in
fn parse_in_expr(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let parser = pair(
        parse_chain_call,
        opt(pair(
            spaced_word("in"),
            ParseError::protect(
                |_| "Expression expected after 'in'".to_string(),
                parse_chain_call,
            ),
        )),
    );

    map(parser, |(left, tail)| match tail {
        Some((op, right)) => puppet_lang::expression::Expression {
            extra: Location::from(op),
            value: puppet_lang::expression::ExpressionVariant::In((
                Box::new(left),
                Box::new(right),
            )),
        },
        None => left,
    })(input)
}

/// https://puppet.com/docs/puppet/7/lang_conditional.html#lang_condition_selector
fn parse_selector_case(input: Span) -> IResult<puppet_lang::expression::SelectorCase<Location>> {
    let parser = tuple((
        parse_case_variant,
        space0_delimimited(tag("=>")),
        ParseError::protect(
            |_| "A value for selector case is expected".to_string(),
            parse_expression,
        ),
    ));

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
    let parser = pair(
        parse_in_expr,
        opt(tuple((
            space0_delimimited(tag("?")),
            ParseError::protect(
                |_| "Opening '{' of selector is expected".to_string(),
                tag("{"),
            ),
            space0_delimimited(separated_list1(
                space0_delimimited(comma_separator),
                space0_delimimited(parse_selector_case),
            )),
            space0_delimimited(opt(comma_separator)),
            ParseError::protect(
                |_| "Closing '}' of selector is expected".to_string(),
                tag("}"),
            ),
        ))),
    );

    map(parser, |(condition, tail)| match tail {
        Some((op, _, cases, _, _)) => puppet_lang::expression::Expression {
            extra: Location::from(op),
            value: puppet_lang::expression::ExpressionVariant::Selector(
                puppet_lang::expression::Selector {
                    condition: Box::new(condition),
                    cases,
                    extra: Location::from(op),
                },
            ),
        },
        None => condition,
    })(input)
}

pub(crate) fn parse_l1(input: Span) -> IResult<puppet_lang::expression::Expression<Location>> {
    let (input, left_expr) = parse_selector(input)?;
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
                                value: TermVariant::Parens(puppet_lang::expression::Parens {
                                    value: Box::new(Expression {
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
                                    }),
                                    accessor: Vec::new(),
                                    extra: Location::new(0, 1, 1)
                                }),
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

#[test]
fn test_function_call() {
    assert_eq!(
        parse_funcall(Span::new("lookup('ask8s::docker::gpu_nvidia')"))
            .unwrap()
            .1,
        puppet_lang::expression::Expression {
            value: puppet_lang::expression::ExpressionVariant::FunctionCall(
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
                                    puppet_lang::string::StringExpr {
                                        data: puppet_lang::string::StringVariant::SingleQuoted(
                                            vec![puppet_lang::string::StringFragment::Literal(
                                                "ask8s::docker::gpu_nvidia".to_owned()
                                            )]
                                        ),
                                        extra: Location::new(7, 1, 8),
                                        accessor: Vec::new()
                                    }
                                ),
                                extra: Location::new(7, 1, 8)
                            }
                        ),
                        extra: Location::new(7, 1, 8)
                    },],
                    lambda: None,
                    extra: Location::new(0, 1, 1),
                    accessor: Vec::new()
                }
            ),
            extra: Location::new(0, 1, 1)
        }
    );
}

#[test]
fn test_in_with_parens() {
    assert_eq!(
        parse_expression(Span::new("(1 in $a)")).unwrap().1,
        puppet_lang::expression::Expression {
            value: puppet_lang::expression::ExpressionVariant::Term(
                puppet_lang::expression::Term {
                    value: puppet_lang::expression::TermVariant::Parens(puppet_lang::expression::Parens {
                        value: Box::new(puppet_lang::expression::Expression {
                            value: puppet_lang::expression::ExpressionVariant::In((
                                Box::new(puppet_lang::expression::Expression {
                                    value: puppet_lang::expression::ExpressionVariant::Term(
                                        puppet_lang::expression::Term {
                                            value: puppet_lang::expression::TermVariant::Integer(
                                                puppet_lang::expression::Integer {
                                                    value: 1,
                                                    extra: Location::new(1,1,2)
                                                }
                                            ),
                                            extra: Location::new(1,1,2)
                                        }
                                    ),
                                    extra: Location::new(1,1,2)
                                }),
                                Box::new(puppet_lang::expression::Expression {
                                    value: puppet_lang::expression::ExpressionVariant::Term(
                                        puppet_lang::expression::Term {
                                            value: puppet_lang::expression::TermVariant::Variable(
                                                puppet_lang::expression::Variable {
                                                    identifier:
                                                        puppet_lang::identifier::LowerIdentifier {
                                                            name: vec!["a".to_owned()],
                                                            is_toplevel: false,
                                                            extra: Location::new(7,1,8)
                                                        },
                                                    accessor: vec![],
                                                    extra: Location::new(7,1,8)
                                                }
                                            ),
                                            extra: Location::new(6,1,7)
                                        }
                                    ),
                                    extra: Location::new(6,1,7)
                                })
                            )),
                            extra: Location::new(3,1,4)
                        }),
                        accessor: vec![],
                        extra: Location::new(0,1,1)
                    }),
                    extra: Location::new(0,1,1)
                }
            ),
            extra: Location::new(0,1,1)
        }
    );
}
