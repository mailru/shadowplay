use anyhow::Result;
use nom::{branch::alt, combinator::map, IResult};

#[derive(Clone, Debug, PartialEq)]
pub enum Toplevel {
    Class(super::class::Class),
    Definition(super::class::Definition),
    Plan(super::class::Plan),
}

impl Toplevel {
    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
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

pub struct Ast {
    pub input: String,
    pub data: Toplevel,
}

impl Ast {
    pub fn parse(input: &str) -> Result<Self> {
        let input = input.to_string();
        let (_, data) = Toplevel::parse::<nom::error::Error<_>>(&input)
            .map_err(|err| anyhow::format_err!("Parsing error: {:?}", err))?;
        Ok(Self { data, input })
    }
}

#[test]
fn test_toplevel() {
    assert!(Ast::parse(
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
