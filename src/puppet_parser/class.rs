use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
    IResult,
};

pub fn header_parser<'a, E>(
    input: &'a str,
) -> IResult<&'a str, (Vec<&'a str>, Vec<super::argument::Argument<'a>>), E>
where
    E: nom::error::ParseError<&'a str>
        + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
{
    let arguments_parser = map(
        opt(super::common::round_brackets_comma_separated0(
            super::argument::Argument::parse,
        )),
        |v: Option<Vec<super::argument::Argument>>| v.unwrap_or_default(),
    );

    tuple((
        super::common::lower_identifier_with_ns,
        preceded(super::common::separator0, arguments_parser),
    ))(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class<'a> {
    pub identifier: Vec<&'a str>,
    pub arguments: Vec<super::argument::Argument<'a>>,
    pub inherits: Option<(bool, Vec<&'a str>)>,
}

impl<'a> Class<'a> {
    pub fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map(
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
                identifier,
                arguments,
                inherits,
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition<'a> {
    pub identifier: Vec<&'a str>,
    pub arguments: Vec<super::argument::Argument<'a>>,
}

impl<'a> Definition<'a> {
    pub fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map(
            preceded(
                tag("define"),
                pair(
                    preceded(super::common::separator0, header_parser),
                    // TODO body
                    preceded(super::common::separator0, tag("{")),
                ),
            ),
            |((identifier, arguments), _body)| Self {
                identifier,
                arguments,
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan<'a> {
    pub identifier: Vec<&'a str>,
    pub arguments: Vec<super::argument::Argument<'a>>,
}

impl<'a> Plan<'a> {
    pub fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map(
            preceded(
                tag("plan"),
                pair(
                    preceded(super::common::separator0, header_parser),
                    // TODO body
                    preceded(super::common::separator0, tag("{")),
                ),
            ),
            |((identifier, arguments), _body)| Self {
                identifier,
                arguments,
            },
        )(input)
    }
}

#[test]
fn test_class() {
    assert_eq!(
        Class::parse::<nom::error::Error<_>>("class  abc::def () {\n TODO }\n").unwrap(),
        (
            "\n TODO }\n",
            Class {
                identifier: vec!["abc", "def"],
                arguments: Vec::new(),
                inherits: None,
            }
        )
    );

    assert!(Class::parse::<nom::error::Error<_>>(
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
    )
    .is_ok());

    assert_eq!(
        Class::parse::<nom::error::Error<_>>(
            "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) {TODO}"
        )
        .unwrap(),
        (
            "TODO}",
            Class {
                identifier: vec!["ab__c", "de11f"],
                arguments: vec![
                    super::argument::Argument {
                        name: "a",
                        type_spec: Some(super::typing::TypeSpecification::String(
                            super::typing::TypeString { min: 1, max: 10 }
                        )),
                        default: None,
                    },
                    super::argument::Argument {
                        name: "b",
                        type_spec: Some(super::typing::TypeSpecification::Custom(vec![
                            "Stdlib", "Unixpath"
                        ])),
                        default: None,
                    },
                    super::argument::Argument {
                        name: "c",
                        type_spec: None,
                        default: None,
                    },
                ],
                inherits: None,
            }
        )
    );
    assert!(Class::parse::<nom::error::Error<_>>(
        "class  ab__c::de11f ( String[1,10] $a, Stdlib::Unixpath $b  ,  $c) inherits aa::bb {TODO}"
    )
    .is_ok());

    assert!(Class::parse::<nom::error::Error<_>>("class a ( $a = INFO) {TODO}").is_err())
}
