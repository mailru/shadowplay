use super::parser::{IResult, IResultUnmarked, Marked, Span};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
};

pub type ArgumentList = Vec<Marked<super::argument::Argument>>;

pub fn header_parser(input: Span) -> IResultUnmarked<(Marked<Vec<&str>>, ArgumentList)> {
    let arguments_parser = map(
        opt(map(
            super::common::round_brackets_comma_separated0(super::argument::Argument::parse),
            |v| v.data,
        )),
        |v: Option<Vec<Marked<super::argument::Argument>>>| v.unwrap_or_default(),
    );

    tuple((
        Marked::parse(map(super::common::lower_identifier_with_ns, |v| {
            v.data.into_iter().map(|v| v.data).collect()
        })),
        preceded(super::common::separator0, arguments_parser),
    ))(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub identifier: Marked<Vec<String>>,
    pub arguments: Vec<Marked<super::argument::Argument>>,
    pub inherits: Option<Marked<(bool, Vec<String>)>>,
}

impl Class {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = map(
            preceded(
                tag("class"),
                tuple((
                    preceded(super::common::separator1, header_parser),
                    super::common::space0_delimimited(opt(preceded(
                        tag("inherits"),
                        super::common::space0_delimimited(
                            super::expression::identifier_with_toplevel,
                        ),
                    ))),
                    // TODO body
                    tag("{"),
                )),
            ),
            |((identifier, arguments), inherits, _body)| Self {
                identifier: identifier.map(|v| v.into_iter().map(String::from).collect()),
                arguments,
                inherits: inherits.map(|v| {
                    v.map(|(is_toplevel, id)| {
                        (is_toplevel, id.into_iter().map(String::from).collect())
                    })
                }),
            },
        );

        Marked::parse(parser)(input)
    }

    pub fn get_argument(&self, name: &str) -> Option<&Marked<super::argument::Argument>> {
        self.arguments.iter().find(|v| v.data.name == name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub identifier: Marked<Vec<String>>,
    pub arguments: Vec<Marked<super::argument::Argument>>,
}

impl Definition {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = map(
            preceded(
                tag("define"),
                pair(
                    preceded(super::common::separator0, header_parser),
                    // TODO body
                    preceded(super::common::separator0, tag("{")),
                ),
            ),
            |((identifier, arguments), _body)| Self {
                identifier: identifier.map(|v| v.into_iter().map(String::from).collect()),
                arguments,
            },
        );

        Marked::parse(parser)(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub identifier: Marked<Vec<String>>,
    pub arguments: Vec<Marked<super::argument::Argument>>,
}

impl Plan {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = map(
            preceded(
                tag("plan"),
                pair(
                    preceded(super::common::separator0, header_parser),
                    // TODO body
                    preceded(super::common::separator0, tag("{")),
                ),
            ),
            |((identifier, arguments), _body)| Self {
                identifier: identifier.map(|v| v.into_iter().map(String::from).collect()),
                arguments,
            },
        );

        Marked::parse(parser)(input)
    }
}

#[test]
fn test_class() {
    assert_eq!(
        Class::parse(Span::new("class  abc::def () {\n TODO }\n"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: Class {
                identifier: Marked {
                    line: 1,
                    column: 8,
                    data: vec!["abc".to_owned(), "def".to_owned()]
                },
                arguments: Vec::new(),
                inherits: None,
            }
        }
    );

    assert!(Class::parse(Span::new(
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

    assert_eq!(
        Class::parse(Span::new(
            "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) {TODO}"
        ))
        .unwrap()
        .1,
        Marked {
            line: 1,
            column: 1,
            data: Class {
                identifier: Marked {
                    line: 1,
                    column: 8,
                    data: vec!["ab__c".to_owned(), "de11f".to_owned()]
                },
                arguments: vec![
                    Marked {
                        line: 1,
                        column: 23,
                        data: super::argument::Argument {
                            name: "a".to_owned(),
                            type_spec: Some(Marked {
                                line: 1,
                                column: 23,
                                data: super::typing::TypeSpecification::String(
                                    super::typing::TypeString {
                                        min: Marked {
                                            line: 1,
                                            column: 30,
                                            data: 1
                                        },
                                        max: Marked {
                                            line: 1,
                                            column: 32,
                                            data: 10
                                        }
                                    }
                                )
                            }),
                            default: None,
                        }
                    },
                    Marked {
                        line: 1,
                        column: 40,
                        data: super::argument::Argument {
                            name: "b".to_owned(),
                            type_spec: Some(Marked {
                                line: 1,
                                column: 40,
                                data: super::typing::TypeSpecification::Custom(vec![
                                    "Stdlib".to_owned(),
                                    "Unixpath".to_owned()
                                ])
                            }),
                            default: None,
                        }
                    },
                    Marked {
                        line: 1,
                        column: 64,
                        data: super::argument::Argument {
                            name: "c".to_owned(),
                            type_spec: None,
                            default: None,
                        }
                    },
                ],
                inherits: None,
            }
        }
    );
    assert!(Class::parse(Span::new(
        "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) inherits aa::bb {TODO}"
    ))
    .is_ok());

    assert!(Class::parse(Span::new("class a ( $a = INFO) {TODO}")).is_err())
}
