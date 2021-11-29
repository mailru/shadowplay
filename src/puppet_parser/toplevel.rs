use super::parser::{IResultUnmarked, Marked, Span};
use nom::{branch::alt, combinator::map};

#[derive(Clone, Debug, PartialEq)]
pub enum Toplevel {
    Class(Marked<super::class::Class>),
    Definition(Marked<super::class::Definition>),
    Plan(Marked<super::class::Plan>),
}

impl Toplevel {
    pub fn parse(input: Span) -> IResultUnmarked<Self> {
        super::common::space0_delimimited(alt((
            map(super::class::Class::parse, Self::Class),
            map(super::class::Definition::parse, Self::Definition),
            map(super::class::Plan::parse, Self::Plan),
        )))(input)
    }
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub input: String,
    pub data: Toplevel,
}

impl Ast {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let input = input.to_string();
        match Toplevel::parse(Span::new(&input)) {
            Ok((_remaining, data)) => Ok(Self { data, input }),
            Err(nom::Err::Failure(err)) => return Err(anyhow::format_err!("{}", err.to_string())),
            Err(err) => return Err(anyhow::format_err!("{}", err)),
        }
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
