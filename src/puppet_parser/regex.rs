use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, recognize, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair};
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(&'a str),
}

fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let not_quote_slash = is_not("/\\");
    verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    recognize(pair(tag("\\"), anychar))(input)
}

fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}

pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(s) => string.push_str(s),
        }
        string
    });

    delimited(char('/'), build_string, char('/'))(input)
}

#[test]
fn test() {
    assert_eq!(
        parse::<nom::error::Error<_>>("//").unwrap(),
        ("", "".to_owned())
    );
    assert_eq!(
        parse::<nom::error::Error<_>>("/aaa/").unwrap(),
        ("", "aaa".to_owned())
    );
    assert_eq!(
        parse::<nom::error::Error<_>>("/\\//").unwrap(),
        ("", "\\/".to_owned())
    );
    assert_eq!(
        parse::<nom::error::Error<_>>("/\\d/").unwrap(),
        ("", "\\d".to_owned())
    );
}
