use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{is_not, take_while_m_n};
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::combinator::{map, map_opt, map_res, recognize, value, verify};
use nom::error::{FromExternalError, ParseError};
use nom::multi::{fold_many0, many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    let not_quote_slash = is_not("'\\");
    verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

fn parse_unicode<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_u32 = map_res(parse_delimited_hex, move |hex| u32::from_str_radix(hex, 16));

    map_opt(parse_u32, std::char::from_u32)(input)
}

fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    preceded(
        char('\\'),
        alt((
            parse_unicode,
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('\'', char('\'')),
        )),
    )(input)
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

pub fn bareword<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let parser = verify(
        recognize(separated_list1(
            tag("-"),
            many1(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| s.chars().next().unwrap().is_ascii_lowercase(),
    );

    map(parser, |v: &str| v.to_string())(input)
}

// TODO возможно имеет смысл в будущем возвращать Vec<StringFragment>, чтобы парсить содержимое
pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    alt((delimited(char('\''), build_string, char('\'')), bareword))(input)
}

#[test]
fn test() {
    assert_eq!(
        parse::<nom::error::Error<_>>("''").unwrap(),
        ("", "".to_owned())
    );
    assert_eq!(
        parse::<nom::error::Error<_>>("'a'").unwrap(),
        ("", "a".to_owned())
    );
    assert_eq!(
        parse::<nom::error::Error<_>>("'\\''").unwrap(),
        ("", "'".to_owned())
    );
    assert_eq!(
        parse::<nom::error::Error<_>>("bARE-WORD_").unwrap(),
        ("", "bARE-WORD_".to_owned())
    );

    assert_eq!(
        parse::<nom::error::Error<_>>("bAREWORD-").unwrap(),
        ("-", "bAREWORD".to_owned())
    );
    assert!(parse::<nom::error::Error<_>>("-").is_err());
    assert!(parse::<nom::error::Error<_>>("BEDA").is_err());
}
