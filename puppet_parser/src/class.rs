use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
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

pub fn header_parser(input: Span) -> IResult<(LowerIdentifier<Location>, Vec<Argument<Location>>)> {
    let arguments_parser = map(
        opt(super::common::round_brackets_comma_separated0(
            crate::argument::parse,
        )),
        |v: Option<Vec<Argument<Location>>>| v.unwrap_or_default(),
    );

    tuple((
        ParseError::protect(
            |_| "Invalid class name".to_owned(),
            super::identifier::identifier_with_toplevel,
        ),
        preceded(super::common::separator0, arguments_parser),
    ))(input)
}

pub fn parse_class(input: Span) -> IResult<Class<Location>> {
    let mut parser = map(
        preceded(
            tag("class"),
            tuple((
                preceded(
                    super::common::separator1,
                    ParseError::protect(
                        |_| "Failed to parse class header".to_owned(),
                        header_parser,
                    ),
                ),
                ParseError::protect(
                    |_| "'{' or 'inherits' expected".to_string(),
                    pair(
                        super::common::space0_delimimited(opt(preceded(
                            tag("inherits"),
                            ParseError::protect(
                                |_| "Failed to parse what class inherits".to_owned(),
                                super::common::space0_delimimited(
                                    crate::identifier::identifier_with_toplevel,
                                ),
                            ),
                        ))),
                        // TODO body
                        tag("{"),
                    ),
                ),
            )),
        ),
        |((identifier, arguments), (inherits, _body))| Class {
            identifier,
            arguments,
            inherits,
            extra: Location::from(input),
        },
    );

    parser(input)
}

pub fn parse_definition(input: Span) -> IResult<Definition<Location>> {
    map(
        preceded(
            tag("define"),
            pair(
                preceded(super::common::separator0, header_parser),
                // TODO body
                preceded(super::common::separator0, tag("{")),
            ),
        ),
        |((identifier, arguments), _body)| Definition {
            identifier,
            arguments,
        },
    )(input)
}

pub fn parse_plan(input: Span) -> IResult<Plan<Location>> {
    map(
        preceded(
            tag("plan"),
            pair(
                preceded(super::common::separator0, header_parser),
                // TODO body
                preceded(super::common::separator0, tag("{")),
            ),
        ),
        |((identifier, arguments), _body)| Plan {
            identifier,
            arguments,
        },
    )(input)
}

#[test]
fn test_class() {
    assert_eq!(
        parse_class(Span::new("class  abc::def () {\n TODO }\n"))
            .unwrap()
            .1,
        Class {
            identifier: LowerIdentifier {
                name: vec!["abc".to_owned(), "def".to_owned()],
                is_toplevel: false,
                extra: Location::new(1, 1, 1),
            },
            arguments: Vec::new(),
            inherits: None,
            extra: Location::new(1, 1, 1),
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
) {
TODO}"
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
        "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) inherits aa::bb {TODO}"
    ))
    .is_ok());

    assert!(parse_class(Span::new("class a ( $a = ,) {TODO}")).is_err())
}
