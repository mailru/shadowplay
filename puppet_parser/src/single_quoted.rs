use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{is_not, take_while_m_n};
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::combinator::{map, map_opt, map_res, recognize, value, verify};
use nom::multi::{fold_many0, many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_literal(input: Span) -> IResult<&str> {
    let not_quote_slash = is_not("'\\");
    verify(map(not_quote_slash, |s: Span| *s), |s: &str| !s.is_empty())(input)
}

pub fn parse_unicode(input: Span) -> IResult<char> {
    let parse_hex = ParseError::protect(
        |_| "unexpected sequence in UTF character".to_owned(),
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
    );

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_u32 = map_res(parse_delimited_hex, move |hex: Span| {
        u32::from_str_radix(*hex, 16)
    });

    map_opt(parse_u32, std::char::from_u32)(input)
}

fn parse_escaped_char(input: Span) -> IResult<char> {
    preceded(
        char('\\'),
        alt((
            alt((value('\\', char('\\')), value('\'', char('\'')))),
            ParseError::protect(
                |s: Span| {
                    format!(
                        "Unexpected escaped character {:?}. See https://puppet.com/docs/puppet/7/lang_data_string.html#lang_data_string_single_quoted_strings-escape-sequences",
                        s.chars().next().unwrap()
                    )
                },
                parse_unicode,
            ),
        )),
    )(input)
}

fn parse_fragment(input: Span) -> IResult<StringFragment> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}

pub fn bareword(input: Span) -> IResult<String> {
    let parser = verify(
        recognize(separated_list1(
            tag("-"),
            many1(alt((alphanumeric1, tag("_")))),
        )),
        |s: &Span| s.chars().next().unwrap().is_ascii_lowercase(),
    );

    map(parser, |v: Span| v.to_string())(input)
}

// TODO возможно имеет смысл в будущем возвращать Vec<StringFragment>, чтобы парсить содержимое
pub fn parse(input: Span) -> IResult<puppet_lang::expression::StringExpr<Location>> {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    let single_quoted_parser = alt((
        preceded(
            char('\''),
            ParseError::protect(
                |_| "Unterminated quoted string".to_string(),
                terminated(build_string, char('\'')),
            ),
        ),
        bareword,
    ));

    map(single_quoted_parser, |data: String| {
        puppet_lang::expression::StringExpr {
            data,
            variant: puppet_lang::expression::StringVariant::SingleQuoted,
            extra: Location::from(input),
        }
    })(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("''")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "".to_owned(),
            variant: puppet_lang::expression::StringVariant::SingleQuoted,
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("'a'")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "a".to_owned(),
            variant: puppet_lang::expression::StringVariant::SingleQuoted,
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("'\\''")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "'".to_owned(),
            variant: puppet_lang::expression::StringVariant::SingleQuoted,
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("bARE-WORD_")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "bARE-WORD_".to_owned(),
            variant: puppet_lang::expression::StringVariant::SingleQuoted,
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        parse(Span::new("bAREWORD-")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "bAREWORD".to_owned(),
            variant: puppet_lang::expression::StringVariant::SingleQuoted,
            extra: Location::new(0, 1, 1)
        }
    );
    assert!(parse(Span::new("-")).is_err());
    assert!(parse(Span::new("BEDA")).is_err());
}
