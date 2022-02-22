use crate::{
    common::{space0_delimimited, space1_delimimited},
    parser::{IResult, Location, ParseError},
};

use super::parser::Span;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
};
use puppet_lang::toplevel::{FunctionDef, Toplevel};

pub fn parse_typedef(input: Span) -> IResult<puppet_lang::toplevel::TypeDef<Location>> {
    map(
        tuple((
            tag("type"),
            space1_delimimited(crate::identifier::camelcase_identifier_with_ns_located),
            ParseError::protect(|_| "'=' expected".to_string(), tag("=")),
            ParseError::protect(
                |_| "Type specification expected".to_string(),
                crate::typing::parse_type_specification,
            ),
        )),
        |(tag, identifier, _, value)| puppet_lang::toplevel::TypeDef {
            identifier,
            value,
            extra: Location::from(tag),
        },
    )(input)
}
pub fn parse_functiondef(input: Span) -> IResult<FunctionDef<Location>> {
    map(
        tuple((
            tag("function"),
            preceded(super::common::separator1, crate::class::parse_header),
            ParseError::protect(
                |_| "'{' or '>>' expected".to_string(),
                pair(
                    space0_delimimited(opt(preceded(
                        tag(">>"),
                        ParseError::protect(
                            |_| "Failed to parse return type".to_owned(),
                            space0_delimimited(crate::typing::parse_type_specification),
                        ),
                    ))),
                    crate::statement::parse_statement_block,
                ),
            ),
        )),
        |(tag, (identifier, arguments), (return_type, body))| FunctionDef {
            identifier,
            arguments,
            return_type,
            body,
            extra: Location::from(tag),
        },
    )(input)
}

pub fn parse(input: Span) -> IResult<Toplevel<Location>> {
    super::common::space0_delimimited(alt((
        map(super::class::parse_class, Toplevel::Class),
        map(super::class::parse_definition, Toplevel::Definition),
        map(super::class::parse_plan, Toplevel::Plan),
        map(parse_typedef, Toplevel::TypeDef),
        map(parse_functiondef, Toplevel::FunctionDef),
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
}"
    ))
    .is_ok())
}

#[test]
fn test_function() {
    assert!(parse(Span::new("function abc::def () {}")).is_ok());

    assert!(parse(Span::new("function abc::def ($a, $b) {}")).is_ok());

    assert!(parse(Span::new("function abc::def ($a, $b) >> String {}")).is_ok());
}
