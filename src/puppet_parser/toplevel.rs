use nom::{branch::alt, combinator::map, IResult};

#[derive(Clone, Debug, PartialEq)]
pub enum Toplevel<'a> {
    Class(super::class::Class<'a>),
    Definition(super::class::Definition<'a>),
    Plan(super::class::Plan<'a>),
}

impl<'a> Toplevel<'a> {
    pub fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        super::common::space0_delimimited(alt((
            map(super::class::Class::parse, Self::Class),
            map(super::class::Definition::parse, Self::Definition),
            map(super::class::Plan::parse, Self::Plan),
        )))(input)
    }
}

#[test]
fn test_toplevel() {
    assert!(Toplevel::parse::<nom::error::Error<_>>(
        "# @summary Install and enroll client to freeipa cluster
#
# A description of what this class does
#
# @example
#   include freeipa::install::client
class freeipa::install::client {
    package { 'ipa-client' : ensure => 'present' }
}"
    )
    .is_ok())
}
