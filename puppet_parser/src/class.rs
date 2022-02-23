use crate::common::space0_delimimited;
use crate::{IResult, Location, ParseError, Span};
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

pub fn parse_header(input: Span) -> IResult<(LowerIdentifier<Location>, Vec<Argument<Location>>)> {
    let arguments_parser = map(
        opt(super::common::round_brackets_comma_separated0(
            crate::argument::parse,
        )),
        |v: Option<Vec<Argument<Location>>>| v.unwrap_or_default(),
    );

    tuple((
        ParseError::protect(
            |_| "Invalid name".to_owned(),
            super::identifier::identifier_with_toplevel,
        ),
        preceded(super::common::separator0, arguments_parser),
    ))(input)
}

pub fn parse_class(input: Span) -> IResult<Class<Location>> {
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
        |(tag, (identifier, arguments), (inherits, body))| Class {
            identifier,
            arguments,
            body,
            inherits,
            extra: Location::from(tag),
        },
    );

    parser(input)
}

pub fn parse_definition(input: Span) -> IResult<Definition<Location>> {
    map(
        tuple((
            tag("define"),
            preceded(super::common::separator1, parse_header),
            space0_delimimited(ParseError::protect(
                |_| "'{' expected".to_string(),
                crate::statement::parse_statement_block,
            )),
        )),
        |(tag, (identifier, arguments), body)| Definition {
            identifier,
            arguments,
            body,
            extra: Location::from(tag),
        },
    )(input)
}

pub fn parse_plan(input: Span) -> IResult<Plan<Location>> {
    map(
        tuple((
            tag("plan"),
            preceded(super::common::separator1, parse_header),
            crate::statement::parse_statement_block,
        )),
        |(tag, (identifier, arguments), body)| Plan {
            identifier,
            arguments,
            body,
            extra: Location::from(tag),
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
                extra: Location::new(7, 1, 8),
            },
            arguments: Vec::new(),
            body: vec![],
            inherits: None,
            extra: Location::new(0, 1, 1),
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
                extra: Location::new(7, 1, 8),
            },
            arguments: Vec::new(),
            body: vec![puppet_lang::statement::Statement {
                value: puppet_lang::statement::StatementVariant::Tag(vec![
                    puppet_lang::string::StringExpr {
                        data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                            puppet_lang::string::StringFragment::Literal("aaa".to_owned())
                        ]),
                        accessor: Vec::new(),
                        extra: Location::new(26, 2, 6)
                    },
                    puppet_lang::string::StringExpr {
                        data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                            puppet_lang::string::StringFragment::Literal("bbb".to_owned())
                        ]),
                        accessor: Vec::new(),
                        extra: Location::new(31, 2, 11)
                    },
                    puppet_lang::string::StringExpr {
                        data: puppet_lang::string::StringVariant::DoubleQuoted(vec![
                            puppet_lang::string::DoubleQuotedFragment::StringFragment(
                                puppet_lang::string::StringFragment::Literal("ccc".to_owned())
                            )
                        ]),
                        accessor: Vec::new(),
                        extra: Location::new(38, 2, 18)
                    }
                ]),
                extra: Location::new(22, 2, 2),
            }],
            inherits: None,
            extra: Location::new(0, 1, 1),
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
                extra: Location::new(7, 1, 8),
            },
            arguments: Vec::new(),
            body: vec![
                puppet_lang::statement::Statement {
                    value: puppet_lang::statement::StatementVariant::Require(vec![
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Identifier(
                                LowerIdentifier {
                                    name: vec!["abc".to_owned(), "def".to_owned()],
                                    is_toplevel: false,
                                    extra: Location::new(30, 2, 10),
                                }
                            ),
                            extra: Location::new(30, 2, 10),
                        }
                    ]),
                    extra: Location::new(22, 2, 2),
                },
                puppet_lang::statement::Statement {
                    value: puppet_lang::statement::StatementVariant::Require(vec![
                        puppet_lang::expression::Term {
                            value: puppet_lang::expression::TermVariant::Identifier(
                                LowerIdentifier {
                                    name: vec!["zzz".to_owned()],
                                    is_toplevel: false,
                                    extra: Location::new(47, 2, 27)
                                }
                            ),
                            extra: Location::new(47, 2, 27)
                        }
                    ]),
                    extra: Location::new(39, 2, 19),
                }
            ],
            inherits: None,
            extra: Location::new(0, 1, 1),
        }
    );
}
