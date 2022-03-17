use crate::common::{space0_delimimited, spaced_word};
use crate::{range::Range, IResult, ParseError, Span};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
};
use puppet_lang::{
    argument::Argument,
    identifier::LowerIdentifier,
    toplevel::{Class, Definition, Plan},
};

type Header = (
    LowerIdentifier<Range>,
    puppet_lang::List<Range, Argument<Range>>,
);

pub fn parse_header(input: Span) -> IResult<Header> {
    let arguments_parser = map(
        opt(crate::common::round_parens_delimimited(
            crate::common::comma_separated_list0_with_last_comment(crate::argument::parse),
        )),
        |v: Option<(Span, puppet_lang::List<Range, Argument<Range>>, Span)>| {
            v.map(|v| v.1).unwrap_or_else(puppet_lang::List::default)
        },
    );

    tuple((
        ParseError::protect(
            |_| "Invalid name".to_owned(),
            crate::identifier::identifier_with_toplevel,
        ),
        preceded(super::common::separator0, arguments_parser),
    ))(input)
}

pub fn parse_class(input: Span) -> IResult<Class<Range>> {
    let mut parser = map(
        tuple((
            spaced_word("class"),
            space0_delimimited(ParseError::protect(
                |_| "Failed to parse class header".to_owned(),
                parse_header,
            )),
            ParseError::protect(
                |_| "'{' or 'inherits' expected".to_string(),
                pair(
                    space0_delimimited(opt(preceded(
                        tag("inherits"),
                        ParseError::protect(
                            |_| "Failed to parse what class is inherited".to_owned(),
                            space0_delimimited(crate::identifier::identifier_with_toplevel),
                        ),
                    ))),
                    crate::statement::parse_statement_block,
                ),
            ),
        )),
        |(kw, (identifier, arguments), (inherits, (_left_bracket, body, right_bracket)))| Class {
            identifier,
            arguments,
            body,
            inherits,
            extra: (kw, right_bracket).into(),
        },
    );

    parser(input)
}

pub fn parse_definition(input: Span) -> IResult<Definition<Range>> {
    map(
        tuple((
            spaced_word("define"),
            space0_delimimited(ParseError::protect(
                |_| "Failed to parse definition header".to_owned(),
                parse_header,
            )),
            space0_delimimited(ParseError::protect(
                |_| "'{' expected".to_string(),
                crate::statement::parse_statement_block,
            )),
        )),
        |(kw, (identifier, arguments), (_left_bracket, body, right_bracket))| Definition {
            identifier,
            arguments,
            body,
            extra: (kw, right_bracket).into(),
        },
    )(input)
}

pub fn parse_plan(input: Span) -> IResult<Plan<Range>> {
    map(
        tuple((
            spaced_word("plan"),
            space0_delimimited(ParseError::protect(
                |_| "Failed to parse plan header".to_owned(),
                parse_header,
            )),
            space0_delimimited(ParseError::protect(
                |_| "'{' expected".to_string(),
                crate::statement::parse_statement_block,
            )),
        )),
        |(kw, (identifier, arguments), (_left_bracket, body, right_bracket))| Plan {
            identifier,
            arguments,
            body,
            extra: (kw, right_bracket).into(),
        },
    )(input)
}

#[test]
fn test_class() {
    assert_eq!(
        parse_class(Span::new("class  abc::def () {\n  }\n"))
            .unwrap()
            .1,
        Class {
            identifier: LowerIdentifier {
                name: vec!["abc".to_owned(), "def".to_owned()],
                is_toplevel: false,
                extra: Range::new(7, 1, 8, 14, 1, 15),
            },
            arguments: puppet_lang::List::default(),
            body: puppet_lang::List::default(),
            inherits: None,
            extra: Range::new(0, 1, 1, 23, 2, 3),
        }
    );

    assert!(parse_class(Span::new(
        "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) inherits aa::bb { }"
    ))
    .is_ok());

    assert!(parse_class(Span::new("class a ( $a = ,) {}")).is_err());

    assert!(parse_class(Span::new("class a () { &&&&& UNKNOWN((STATEMENT}")).is_err())
}

#[test]
fn test_body_tag() {
    assert_eq!(
        parse_class(Span::new(
            "class  abc::def () {\n tag aaa, 'bbb', \"ccc\" }\n"
        ))
        .unwrap()
        .1,
        Class {
            identifier: LowerIdentifier {
                name: vec!["abc".to_owned(), "def".to_owned()],
                is_toplevel: false,
                extra: Range::new(7, 1, 8, 14, 1, 15),
            },
            arguments: puppet_lang::List::default(),
            body: puppet_lang::List {
                last_comment: vec![],
                value: vec![puppet_lang::statement::Statement {
                    value: puppet_lang::statement::StatementVariant::Expression(
                        puppet_lang::expression::Expression {
                            accessor: None,
                            comment: vec![],
                            value: puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                                puppet_lang::builtin::BuiltinVariant::Tag(
                                    puppet_lang::builtin::Many1 {
                                        lambda: None,
                                        args: vec![
                                            puppet_lang::expression::Expression {
                                                accessor: None,
                                                comment: vec![],
                                                value: puppet_lang::expression::ExpressionVariant::Term(
                                                    puppet_lang::expression::Term {
                                                        value: puppet_lang::expression::TermVariant::String(
                                                            puppet_lang::string::StringExpr {
                                                                data:
                                                                puppet_lang::string::StringVariant::SingleQuoted(
                                                                    vec![
                                                                        puppet_lang::string::StringFragment::Literal(
                                                                            puppet_lang::string::Literal {
                                                                            data: "aaa".to_owned(),
                                                                                extra: Range::new(26, 2, 6, 28, 2, 8)
                                                                            }
                                                                        )
                                                                    ]
                                                                ),
                                                                extra: Range::new(26, 2, 6, 28, 2, 8)
                                                            },
                                                        ),
                                                        extra: Range::new(26, 2, 6, 28, 2, 8)
                                                    }
                                                ),
                                                extra: Range::new(26, 2, 6, 28, 2, 8)
                                            },
                                            puppet_lang::expression::Expression {
                                                accessor: None,
                                                comment: vec![],
                                                value: puppet_lang::expression::ExpressionVariant::Term(
                                                    puppet_lang::expression::Term {
                                                        value: puppet_lang::expression::TermVariant::String(
                                                            puppet_lang::string::StringExpr {
                                                                data:
                                                                puppet_lang::string::StringVariant::SingleQuoted(
                                                                    vec![
                                                                        puppet_lang::string::StringFragment::Literal(
                                                                            puppet_lang::string::Literal {
                                                                                data: "bbb".to_owned(),
                                                                                extra: Range::new(32, 2, 12, 34, 2, 14)
                                                                            }
                                                                        )
                                                                    ]
                                                                ),
                                                                extra: Range::new(31, 2, 11, 35, 2, 15)
                                                            },
                                                        ),
                                                        extra: Range::new(31, 2, 11, 35, 2, 15)
                                                    }
                                                ),
                                                extra: Range::new(31, 2, 11, 35, 2, 15)
                                            },
                                            puppet_lang::expression::Expression {
                                                accessor: None,
                                                comment: vec![],
                                                value: puppet_lang::expression::ExpressionVariant::Term(
                                                    puppet_lang::expression::Term {
                                                        value: puppet_lang::expression::TermVariant::String(
                                                            puppet_lang::string::StringExpr {
                                                                data: puppet_lang::string::StringVariant::DoubleQuoted(
                                                                    vec![
                                                                        puppet_lang::string::DoubleQuotedFragment::StringFragment(
                                                                            puppet_lang::string::StringFragment::Literal(
                                                                                puppet_lang::string::Literal {
                                                                                    data: "ccc".to_owned(),
                                                                                    extra: Range::new(39, 2, 19, 41, 2, 21)
                                                                                }
                                                                            ))
                                                                    ]),
                                                                extra: Range::new(38, 2, 18, 42, 2, 22)
                                                            },
                                                        ),
                                                        extra: Range::new(38, 2, 18, 42, 2, 22)
                                                    }
                                                ),
                                                extra: Range::new(38, 2, 18, 42, 2, 22)
                                            },
                                        ],
                                    }
                                )
                            ),
                            extra: Range::new(22, 2, 2, 42, 2, 22),
                        }
                    ),
                comment: vec![],
            }]},
            inherits: None,
            extra: Range::new(0, 1, 1, 44, 2, 24),
        }
    );
}

#[test]
fn test_body_require() {
    assert_eq!(
        parse_class(Span::new(
            "class  abc::def () {\n require abc::def require zzz }\n"
        ))
            .unwrap()
            .1,
        Class {
            identifier: LowerIdentifier {
                name: vec!["abc".to_owned(), "def".to_owned()],
                is_toplevel: false,
                extra: Range::new(7, 1, 8, 14, 1, 15),
            },
            arguments: puppet_lang::List::default(),
            body: puppet_lang::List {
                last_comment: vec![],
                value: vec![
                    puppet_lang::statement::Statement {
                        value: puppet_lang::statement::StatementVariant::Expression(
                            puppet_lang::expression::Expression {
                                accessor: None,
                                comment: vec![],
                                value: puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                                    puppet_lang::builtin::BuiltinVariant::Require(puppet_lang::builtin::Many1 {
                                        args: vec![puppet_lang::expression::Expression {
                                            accessor: None,
                                            comment: vec![],
                                            value: puppet_lang::expression::ExpressionVariant::Term(
                                                puppet_lang::expression::Term {
                                                    value:
                                                    puppet_lang::expression::TermVariant::Identifier(
                                                        LowerIdentifier {
                                                            name: vec![
                                                                "abc".to_owned(),
                                                                "def".to_owned()
                                                            ],
                                                            is_toplevel: false,
                                                            extra: Range::new(30, 2, 10, 37, 2, 17),
                                                        }
                                                    ),
                                                    extra: Range::new(30, 2, 10, 37, 2, 17),
                                                }
                                            ),
                                            extra: Range::new(30, 2, 10, 37, 2, 17),
                                        }],
                                        lambda: None,
                                    },
                                    )),
                                extra: Range::new(22, 2, 2, 37, 2, 17),
                            }
                        ),
                        comment: vec![],
                    },
                    puppet_lang::statement::Statement {
                        value: puppet_lang::statement::StatementVariant::Expression(
                            puppet_lang::expression::Expression {
                                accessor: None,
                                comment: vec![],
                                value: puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                                    puppet_lang::builtin::BuiltinVariant::Require(puppet_lang::builtin::Many1 {
                                        args: vec![puppet_lang::expression::Expression {
                                            accessor: None,
                                            comment: vec![],
                                            value: puppet_lang::expression::ExpressionVariant::Term(
                                                puppet_lang::expression::Term {
                                                    value: puppet_lang::expression::TermVariant::String(
                                                        puppet_lang::string::StringExpr {
                                                            data:
                                                            puppet_lang::string::StringVariant::SingleQuoted(
                                                                vec![
                                                                    puppet_lang::string::StringFragment::Literal(
                                                                        puppet_lang::string::Literal {
                                                                            data: "zzz".to_owned(),
                                                                            extra: Range::new(47,2, 27, 49, 2, 29)
                                                                        }
                                                                    )
                                                                ]
                                                            ),
                                                            extra: Range::new(47, 2, 27, 49, 2, 29),
                                                        }
                                                    ),
                                                    extra: Range::new(47, 2, 27, 49, 2, 29)
                                                }
                                            ),
                                            extra: Range::new(47, 2, 27, 49, 2, 29)
                                        }],
                                        lambda: None,
                                    })),
                                extra: Range::new(39, 2, 19, 49, 2, 29)
                            }
                        ),
                        comment: vec![],
                    }
                ]},
            inherits: None,
            extra: Range::new(0, 1, 1, 51, 2, 31),
        }
    );
}
