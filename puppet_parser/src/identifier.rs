use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, anychar, digit1},
    combinator::{map, opt, recognize, verify},
    multi::many0,
    sequence::{pair, tuple},
};
use puppet_lang::identifier::LowerIdentifier;

use crate::parser::{IResult, Location, Span};

pub fn char_lower(input: Span) -> IResult<char> {
    verify(anychar, |c| c.is_ascii_lowercase())(input)
}

pub fn char_upper(input: Span) -> IResult<char> {
    verify(anychar, |c| c.is_ascii_uppercase())(input)
}

pub fn identifier(input: Span) -> IResult<Span> {
    recognize(tuple((alpha1, many0(alt((alphanumeric1, tag("_")))))))(input)
}

pub fn lowercase_identifier(input: Span) -> IResult<Span> {
    recognize(tuple((
        char_lower,
        many0(alt((recognize(char_lower), digit1, tag("_")))),
    )))(input)
}

pub fn camel_case_identifier(input: Span) -> IResult<&str> {
    map(
        recognize(tuple((char_upper, many0(alt((alphanumeric1, tag("_"))))))),
        |v: Span| *v,
    )(input)
}

pub fn lower_identifier_with_ns(input: Span) -> IResult<Vec<&str>> {
    nom::multi::separated_list1(tag("::"), map(lowercase_identifier, |v: Span| *v))(input)
}

pub fn camelcase_identifier_with_ns(input: Span) -> IResult<Vec<&str>> {
    nom::multi::separated_list1(tag("::"), camel_case_identifier)(input)
}

pub fn identifier_with_toplevel(input: Span) -> IResult<LowerIdentifier<Location>> {
    map(
        pair(
            map(opt(tag("::")), |v| v.is_some()),
            lower_identifier_with_ns,
        ),
        |(is_toplevel, name)| LowerIdentifier {
            name: name.iter().map(|v| v.to_string()).collect(),
            is_toplevel,
            extra: Location::from(input),
        },
    )(input)
}

#[test]
fn test_lower_case_identifier_with_ns() {
    assert_eq!(
        lower_identifier_with_ns(Span::new("asd")).unwrap().1,
        vec!["asd"]
    );
    assert_eq!(
        lower_identifier_with_ns(Span::new("asd::def")).unwrap().1,
        vec!["asd", "def",]
    );
    assert!(lower_identifier_with_ns(Span::new("")).is_err())
}

#[test]
fn test_identifier_with_toplevel() {
    assert_eq!(
        identifier_with_toplevel(Span::new("::asd")).unwrap().1,
        LowerIdentifier {
            name: vec!["asd".to_owned()],
            is_toplevel: true,
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        identifier_with_toplevel(Span::new("asd")).unwrap().1,
        LowerIdentifier {
            name: vec!["asd".to_owned()],
            is_toplevel: false,
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        identifier_with_toplevel(Span::new("asd::def")).unwrap().1,
        LowerIdentifier {
            name: vec!["asd".to_owned(), "def".to_owned()],
            is_toplevel: false,
            extra: Location::new(0, 1, 1)
        }
    );
}
