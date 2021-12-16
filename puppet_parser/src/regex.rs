use crate::parser::Location;

use super::parser::{IResult, Span};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, recognize, verify};
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(Span<'a>),
    EscapedChar(Span<'a>),
}

fn parse_literal(input: Span) -> IResult<Span> {
    let not_quote_slash = is_not("/\\");
    verify(not_quote_slash, |s: &Span| !s.is_empty())(input)
}

fn parse_escaped_char(input: Span) -> IResult<Span> {
    recognize(pair(tag("\\"), anychar))(input)
}

fn parse_fragment(input: Span) -> IResult<StringFragment> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}

pub fn parse(input: Span) -> IResult<puppet_lang::expression::Regexp<Location>> {
    let build_regex = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(&s),
            StringFragment::EscapedChar(s) => string.push_str(&s),
        }
        string
    });

    map(delimited(char('/'), build_regex, char('/')), |data| {
        puppet_lang::expression::Regexp {
            data,
            extra: Location::from(input),
        }
    })(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("//")).unwrap().1,
        puppet_lang::expression::Regexp {
            data: "".to_owned(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("/aaa/")).unwrap().1,
        puppet_lang::expression::Regexp {
            data: "aaa".to_owned(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("/\\//")).unwrap().1,
        puppet_lang::expression::Regexp {
            data: "\\/".to_owned(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("/\\d/")).unwrap().1,
        puppet_lang::expression::Regexp {
            data: "\\d".to_owned(),
            extra: Location::new(0, 1, 1)
        }
    );
}
