use crate::puppet_lang::string::{
    DoubleQuotedFragment, Literal, StringExpr, StringFragment, StringVariant,
};
use crate::puppet_parser::{range::Range, IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::combinator::{map, success, verify};
use nom::multi::fold_many0;
use nom::sequence::{pair, tuple};

fn parse_literal(input: Span) -> IResult<StringFragment<Range>> {
    let not_quote_slash = is_not("\"\\$");
    map(
        verify(not_quote_slash, |s: &Span| !(*s).is_empty()),
        |data: Span| {
            StringFragment::Literal(Literal {
                extra: Range::from((data, data)),
                data: data.to_string(),
            })
        },
    )(input)
}

fn parse_interpolation(input: Span) -> IResult<DoubleQuotedFragment<Range>> {
    let parser_variable = || {
        map(
            pair(
                crate::puppet_parser::identifier::identifier_with_toplevel,
                crate::puppet_parser::expression::parse_accessor,
            ),
            |(identifier, accessor)| crate::puppet_lang::expression::Expression {
                extra: (&identifier.extra, &accessor, &identifier.extra).into(),
                value: crate::puppet_lang::expression::ExpressionVariant::Term(
                    crate::puppet_lang::expression::Term {
                        extra: (&identifier.extra, &accessor, &identifier.extra).into(),
                        value: crate::puppet_lang::expression::TermVariant::Variable(
                            crate::puppet_lang::expression::Variable {
                                extra: (&identifier.extra, &accessor, &identifier.extra).into(),
                                is_local_scope: identifier.name.last().unwrap().starts_with('_'),
                                identifier,
                            },
                        ),
                    },
                ),
                accessor,
                // Comments are not possible for interpolated expressions
                comment: vec![],
            },
        )
    };

    let (input, dollar_tag) = tag("$")(input)?;

    let mut parser = alt((
        map(
            tuple((tag("{"), parser_variable(), tag("}"))),
            |(_left_bracket, expr, _right_bracket)| {
                DoubleQuotedFragment::Expression(crate::puppet_lang::string::Expression {
                    data: expr,
                })
            },
        ),
        map(
            tuple((
                tag("{"),
                crate::puppet_parser::expression::parse_expression,
                tag("}"),
            )),
            |(_left_bracket, expr, _right_bracket)| {
                DoubleQuotedFragment::Expression(crate::puppet_lang::string::Expression {
                    data: expr,
                })
            },
        ),
        map(parser_variable(), |expr| {
            DoubleQuotedFragment::Expression(crate::puppet_lang::string::Expression { data: expr })
        }),
        map(success(()), |()| {
            DoubleQuotedFragment::StringFragment(StringFragment::Literal(
                crate::puppet_lang::string::Literal {
                    extra: Range::from((dollar_tag, dollar_tag)),
                    data: (*dollar_tag).to_owned(),
                },
            ))
        }),
    ));

    parser(input)
}

fn parse_fragment(input: Span) -> IResult<DoubleQuotedFragment<Range>> {
    alt((
        map(parse_literal, DoubleQuotedFragment::StringFragment),
        parse_interpolation,
        map(
            crate::puppet_parser::single_quoted::parse_unicode,
            DoubleQuotedFragment::StringFragment,
        ),
        map(
            crate::puppet_parser::single_quoted::parse_escaped,
            DoubleQuotedFragment::StringFragment,
        ),
    ))(input)
}

pub fn parse(input: Span) -> IResult<StringExpr<Range>> {
    let build_string = fold_many0(parse_fragment, Vec::new, |mut list, fragment| {
        list.push(fragment);
        list
    });

    let double_quoted_parser = tuple((
        tag("\""),
        build_string,
        ParseError::protect(
            |_| "Unterminated double quoted string".to_string(),
            tag("\""),
        ),
    ));

    map(double_quoted_parser, |(left_quote, data, right_quote)| {
        StringExpr {
            data: StringVariant::DoubleQuoted(data),
            extra: Range::from((&left_quote, &right_quote)),
        }
    })(input)
}

#[test]
fn test_simple() {
    assert_eq!(
        parse(Span::new("\"\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![]),
            extra: Range::new(0, 1, 1, 1, 1, 2)
        }
    );
    assert_eq!(
        parse(Span::new("\"a\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(
                    crate::puppet_lang::string::StringFragment::Literal(
                        crate::puppet_lang::string::Literal {
                            data: "a".to_owned(),
                            extra: Range::new(1, 1, 2, 1, 1, 2)
                        }
                    )
                )
            ]),
            extra: Range::new(0, 1, 1, 2, 1, 3)
        }
    );
    assert_eq!(
        parse(Span::new("\"\\\"\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(
                    crate::puppet_lang::string::StringFragment::Escaped(
                        crate::puppet_lang::string::Escaped {
                            data: '"',
                            extra: Range::new(1, 1, 2, 2, 1, 3)
                        }
                    )
                )
            ]),
            extra: Range::new(0, 1, 1, 3, 1, 4)
        }
    );
}

#[test]
fn test_no_interpolation() {
    assert_eq!(
        parse(Span::new("\"$\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(
                    crate::puppet_lang::string::StringFragment::Literal(
                        crate::puppet_lang::string::Literal {
                            data: "$".to_owned(),
                            extra: Range::new(1, 1, 2, 1, 1, 2)
                        }
                    )
                )
            ]),
            extra: Range::new(0, 1, 1, 2, 1, 3)
        }
    );
    assert_eq!(
        parse(Span::new("\"$(\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(
                    crate::puppet_lang::string::StringFragment::Literal(
                        crate::puppet_lang::string::Literal {
                            data: "$".to_owned(),
                            extra: Range::new(1, 1, 2, 1, 1, 2)
                        }
                    )
                ),
                DoubleQuotedFragment::StringFragment(
                    crate::puppet_lang::string::StringFragment::Literal(
                        crate::puppet_lang::string::Literal {
                            data: "(".to_owned(),
                            extra: Range::new(2, 1, 3, 2, 1, 3)
                        }
                    )
                )
            ]),
            extra: Range::new(0, 1, 1, 3, 1, 4)
        }
    );
}

#[test]
fn test_interpolatad_variable() {
    assert_eq!(
        parse(Span::new("\"${varname}\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![
                crate::puppet_lang::string::DoubleQuotedFragment::Expression(
                    crate::puppet_lang::string::Expression {
                        data: crate::puppet_lang::expression::Expression {
                            value: crate::puppet_lang::expression::ExpressionVariant::Term(
                                crate::puppet_lang::expression::Term {
                                    value: crate::puppet_lang::expression::TermVariant::Variable(
                                        crate::puppet_lang::expression::Variable {
                                            identifier:
                                                crate::puppet_lang::identifier::LowerIdentifier {
                                                    name: vec!["varname".to_string()],
                                                    is_toplevel: false,
                                                    extra: Range::new(3, 1, 4, 9, 1, 10)
                                                },
                                            is_local_scope: false,
                                            extra: Range::new(3, 1, 4, 9, 1, 10)
                                        }
                                    ),
                                    extra: Range::new(3, 1, 4, 9, 1, 10)
                                }
                            ),
                            extra: Range::new(3, 1, 4, 9, 1, 10),
                            accessor: None,
                            comment: vec![],
                        },
                    }
                )
            ]),
            extra: Range::new(0, 1, 1, 11, 1, 12)
        }
    );
}

#[test]
fn test_interpolatad_expression() {
    assert_eq!(
        parse(Span::new("\"${funcall()}\"")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::DoubleQuoted(vec![
                crate::puppet_lang::string::DoubleQuotedFragment::Expression(
                    crate::puppet_lang::string::Expression {
                        data: crate::puppet_lang::expression::Expression {
                            value: crate::puppet_lang::expression::ExpressionVariant::FunctionCall(
                                crate::puppet_lang::expression::FunctionCall {
                                    extra: Range::new(3, 1, 4, 11, 1, 12),
                                    identifier: crate::puppet_lang::identifier::LowerIdentifier {
                                        name: vec!["funcall".to_string()],
                                        is_toplevel: false,
                                        extra: Range::new(3, 1, 4, 9, 1, 10)
                                    },
                                    args: vec![],
                                    lambda: None,
                                }
                            ),
                            extra: Range::new(3, 1, 4, 11, 1, 12),
                            accessor: None,
                            comment: vec![],
                        },
                    }
                )
            ]),
            extra: Range::new(0, 1, 1, 13, 1, 14)
        }
    );
}
