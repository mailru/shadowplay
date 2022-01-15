use crate::parser::{IResult, Location};

use super::parser::Span;
use nom::{branch::alt, combinator::map};
use puppet_lang::toplevel::Toplevel;

pub fn parse(input: Span) -> IResult<Toplevel<Location>> {
    super::common::space0_delimimited(alt((
        map(super::class::parse_class, Toplevel::Class),
        map(super::class::parse_definition, Toplevel::Definition),
        map(super::class::parse_plan, Toplevel::Plan),
    )))(input)
}

#[test]
fn test_toplevel() {
    assert!(parse(Span::new(
        "# @summary Install and enroll client to freeipa cluster
#
# A description of what this class does
#
# @example
#   include freeipa::install::client
class freeipa::install::client {
    package { 'ipa-client' : ensure => 'present' }
}"
    ))
    .is_ok())
}
