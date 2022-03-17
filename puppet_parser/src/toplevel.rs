use crate::{
    common::{space0_delimimited, space1_delimimited},
    {range::Range, IResult, ParseError, Span},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{eof, map, opt},
    sequence::{pair, preceded, terminated, tuple},
};
use puppet_lang::toplevel::{FunctionDef, Toplevel, ToplevelVariant};

pub fn parse_typedef(input: Span) -> IResult<puppet_lang::toplevel::TypeDef<Range>> {
    map(
        tuple((
            tag("type"),
            space1_delimimited(crate::identifier::camelcase_identifier_with_ns_located),
            ParseError::protect(|_| "'=' expected".to_string(), tag("=")),
            ParseError::protect(
                |_| "Type specification expected".to_string(),
                space0_delimimited(crate::typing::parse_type_specification),
            ),
        )),
        |(keyword, identifier, _, value)| puppet_lang::toplevel::TypeDef {
            extra: Range::from((keyword, &value.extra)),
            identifier,
            value,
        },
    )(input)
}
pub fn parse_functiondef(input: Span) -> IResult<FunctionDef<Range>> {
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
        |(keyword, (identifier, arguments), (return_type, (_left_curly, body, right_curly)))| {
            FunctionDef {
                identifier,
                arguments,
                return_type,
                body,
                extra: Range::from((keyword, right_curly)),
            }
        },
    )(input)
}

pub fn parse(input: Span) -> IResult<Toplevel<Range>> {
    crate::common::space0_delimimited(alt((
        map(crate::class::parse_class, |v| Toplevel {
            extra: v.extra.clone(),
            data: ToplevelVariant::Class(v),
        }),
        map(crate::class::parse_definition, |v| Toplevel {
            extra: v.extra.clone(),
            data: ToplevelVariant::Definition(v),
        }),
        map(crate::class::parse_plan, |v| Toplevel {
            extra: v.extra.clone(),
            data: ToplevelVariant::Plan(v),
        }),
        map(parse_typedef, |v| Toplevel {
            extra: v.extra.clone(),
            data: ToplevelVariant::TypeDef(v),
        }),
        map(parse_functiondef, |v| Toplevel {
            extra: v.extra.clone(),
            data: ToplevelVariant::FunctionDef(v),
        }),
    )))(input)
}

pub fn parse_file(
    input: Span,
) -> IResult<puppet_lang::List<Range, puppet_lang::statement::Statement<Range>>> {
    terminated(crate::statement::parse_statement_list, eof)(input)
}

#[test]
fn test_toplevel() {
    assert!(crate::statement::parse_statement_list(Span::new(
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
