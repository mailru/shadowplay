use crate::common::space0_delimimited;
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

pub fn parse_header(input: Span) -> IResult<(LowerIdentifier<Range>, Vec<Argument<Range>>)> {
    let arguments_parser = map(
        opt(super::common::round_brackets_comma_separated0(
            crate::argument::parse,
        )),
        |v: Option<(Span, Vec<Argument<Range>>, Span)>| v.map(|v| v.1).unwrap_or_default(),
    );

    tuple((
        ParseError::protect(
            |_| "Invalid name".to_owned(),
            super::identifier::identifier_with_toplevel,
        ),
        preceded(super::common::separator0, arguments_parser),
    ))(input)
}

pub fn parse_class(input: Span) -> IResult<Class<Range>> {
    let mut parser = map(
        tuple((
            tag("class"),
            preceded(
                super::common::separator1,
                ParseError::protect(|_| "Failed to parse class header".to_owned(), parse_header),
            ),
            ParseError::protect(
                |_| "'{' or 'inherits' expected".to_string(),
                pair(
                    space0_delimimited(opt(preceded(
                        tag("inherits"),
                        ParseError::protect(
                            |_| "Failed to parse what class inherits".to_owned(),
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
            tag("define"),
            preceded(super::common::separator1, parse_header),
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
            tag("plan"),
            preceded(super::common::separator1, parse_header),
            crate::statement::parse_statement_block,
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
            arguments: Vec::new(),
            body: vec![],
            inherits: None,
            extra: Range::new(0, 1, 1, 23, 2, 3),
        }
    );

    assert!(parse_class(Span::new(
        "class tarantool2::add (
    $prefix                 = 'tarantool_box',
    $root_dir               = '/var',
    $snaps_dir              = 'snaps',
    $xlogs_dir              = 'xlogs',
    $logs_dir               = 'logs',
    $script_dir             = false,
    $snaps_prefix           = '/snaps',
    $xlogs_prefix           = '/xlogs',
    $user                   = 'tarantool',
    $group                  = 'tarantool',
    $mode                   = '0645',
    $snap_io_rate_limit     = false,
    $rows_per_wal           = false,
    $group_name,
    $slab_alloc_factor      = false,
    $log_level              = false,
    $wal_fsync_delay        = false,
    $wal_mode               = false,
    $memcached_port         = false,
    $memcached_space        = false,
    $memcached_expire       = false,
    $spaces                 = false,
    $enabled                = true,
    $lua_links              = false,
    $lua_files              = false,
    $service_init_target    = '/etc/init.d/tarantool_box',
    $service_wrapper_target = '/usr/bin/tarantool_multi.sh',
    $mon_arena_usage_crit   = '80',
    $mon_arena_usage_warn   = '75',
    $mon_replication_lag    = '30',
    $mon_fibers             = undef,
    $too_long_threshold     = false,
    $coredump               = false,
    $wal_writer_inbox_size  = false,
    String $create_snapshot_time           = \"0  1  *  *  *\",
    Boolean $create_snapshot_use_lock      = true,
    Integer $create_snapshot_delay_maximum = 150 * 60, # use -1 for no-delay
    $clean_logs_time        = '12 6 * * *',
) {  }"
    ))
    .is_ok());

    // assert_eq!(
    //     parse_class(Span::new(
    //         "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) {TODO}"
    //     ))
    //     .unwrap()
    //     .1,
    //     Marked {
    //         line: 1,
    //         column: 1,
    //         data: Class {
    //             identifier: Marked {
    //                 line: 1,
    //                 column: 8,
    //                 data: vec!["ab__c".to_owned(), "de11f".to_owned()]
    //             },
    //             arguments: vec![
    //                 Marked {
    //                     line: 1,
    //                     column: 23,
    //                     data: super::argument::Argument {
    //                         name: "a".to_owned(),
    //                         type_spec: Some(Marked {
    //                             line: 1,
    //                             column: 23,
    //                             data: super::typing::TypeSpecification::String(
    //                                 super::typing::TypeString {
    //                                     min: Marked {
    //                                         line: 1,
    //                                         column: 30,
    //                                         data: 1
    //                                     },
    //                                     max: Marked {
    //                                         line: 1,
    //                                         column: 32,
    //                                         data: 10
    //                                     }
    //                                 }
    //                             )
    //                         }),
    //                         default: None,
    //                     }
    //                 },
    //                 Marked {
    //                     line: 1,
    //                     column: 40,
    //                     data: super::argument::Argument {
    //                         name: "b".to_owned(),
    //                         type_spec: Some(Marked {
    //                             line: 1,
    //                             column: 40,
    //                             data: super::typing::TypeSpecification::Custom(vec![
    //                                 "Stdlib".to_owned(),
    //                                 "Unixpath".to_owned()
    //                             ])
    //                         }),
    //                         default: None,
    //                     }
    //                 },
    //                 Marked {
    //                     line: 1,
    //                     column: 64,
    //                     data: super::argument::Argument {
    //                         name: "c".to_owned(),
    //                         type_spec: None,
    //                         default: None,
    //                     }
    //                 },
    //             ],
    //             inherits: None,
    //         }
    //     }
    // );

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
            arguments: Vec::new(),
            body: vec![puppet_lang::statement::Statement {
                value: puppet_lang::statement::StatementVariant::BuiltinFunction(
                    puppet_lang::statement::BuiltinFunction {
                        name: "tag".to_owned(),
                        extra: Range::new(22, 2, 2, 42, 2, 22),
                        args: vec![
                            puppet_lang::expression::Expression {
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
                                                accessor: None,
                                                extra: Range::new(26, 2, 6, 28, 2, 8)
                                            },
                                        ),
                                        extra: Range::new(26, 2, 6, 28, 2, 8)
                                    }
                                ),
                                extra: Range::new(26, 2, 6, 28, 2, 8)
                            },
                            puppet_lang::expression::Expression {
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
                                                accessor: None,
                                                extra: Range::new(31, 2, 11, 35, 2, 15)
                                            },
                                        ),
                                        extra: Range::new(31, 2, 11, 35, 2, 15)
                                    }
                                ),
                                extra: Range::new(31, 2, 11, 35, 2, 15)
                            },
                            puppet_lang::expression::Expression {
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
                                                accessor: None,
                                                extra: Range::new(38, 2, 18, 42, 2, 22)
                                            },
                                        ),
                                        extra: Range::new(38, 2, 18, 42, 2, 22)
                                    }
                                ),
                                extra: Range::new(38, 2, 18, 42, 2, 22)
                            },
                        ]
                    }
                ),
            }],
            inherits: None,
            extra: Range::new(0, 1, 1, 44, 2, 24),
        }
    );
}

#[test]
fn test_body_require() {
    assert_eq!(
        parse_class(Span::new(
            // 0123456789012345678901234567890123456789012345678901234567890
            // 0         10        20        30        40        50
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
            arguments: Vec::new(),
            body: vec![
                puppet_lang::statement::Statement {
                    value: puppet_lang::statement::StatementVariant::BuiltinFunction(
                        puppet_lang::statement::BuiltinFunction {
                            name: "require".to_owned(),
                            extra: Range::new(22, 2, 2, 37, 2, 17),
                            args: vec![puppet_lang::expression::Expression {
                                value: puppet_lang::expression::ExpressionVariant::Term(
                                    puppet_lang::expression::Term {
                                        value: puppet_lang::expression::TermVariant::Identifier(
                                            LowerIdentifier {
                                                name: vec!["abc".to_owned(), "def".to_owned()],
                                                is_toplevel: false,
                                                extra: Range::new(30, 2, 10, 37, 2, 17),
                                            }
                                        ),
                                        extra: Range::new(30, 2, 10, 37, 2, 17),
                                    }
                                ),
                                extra: Range::new(30, 2, 10, 37, 2, 17),
                            }]
                        }
                    ),
                },
                puppet_lang::statement::Statement {
                    value: puppet_lang::statement::StatementVariant::BuiltinFunction(
                        puppet_lang::statement::BuiltinFunction {
                            name: "require".to_owned(),
                            extra: Range::new(39, 2, 19, 49, 2, 29),
                            args: vec![puppet_lang::expression::Expression {
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
                                                accessor: None,
                                                extra: Range::new(47, 2, 27, 49, 2, 29),
                                            }
                                        ),
                                        extra: Range::new(47, 2, 27, 49, 2, 29)
                                    }
                                ),
                                extra: Range::new(47, 2, 27, 49, 2, 29)
                            }]
                        }
                    ),
                }
            ],
            inherits: None,
            extra: Range::new(0, 1, 1, 51, 2, 31),
        }
    );
}
