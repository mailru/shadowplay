use crate::parser::Location;
use crate::term::parse_accessor;

use super::parser::{IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, value, verify};
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, preceded};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_literal(input: Span) -> IResult<&str> {
    let not_quote_slash = is_not("\"\\");
    verify(map(not_quote_slash, |s: Span| *s), |s: &str| !s.is_empty())(input)
}

fn parse_escaped_char(input: Span) -> IResult<char> {
    preceded(
        char('\\'),
        alt((
            alt((
                value('\n', char('n')),
                value('\r', char('r')),
                value('\t', char('t')),
                value(' ', char('s')),
                value('$', char('$')),
                value('\u{08}', char('b')),
                value('\u{0C}', char('f')),
                value('\\', char('\\')),
                value('\"', char('\"')),
                value('\'', char('\'')),
            )),
            ParseError::protect_with_url(
                |s: Span| {
                    (format!(
                        "Unexpected escaped character {:?}.",
                        s.chars().next().unwrap()
                    ), "https://puppet.com/docs/puppet/7/lang_data_string.html#lang_data_string_double_quoted_strings-escape-sequences")
                },
                crate::single_quoted::parse_unicode,
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

// TODO возможно имеет смысл в будущем возвращать Vec<StringFragment>, чтобы парсить содержимое
pub fn parse(input: Span) -> IResult<puppet_lang::expression::StringExpr<Location>> {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    map(
        delimited(char('"'), pair(build_string, parse_accessor), char('"')),
        |(data, accessor)| puppet_lang::expression::StringExpr {
            data,
            variant: puppet_lang::expression::StringVariant::DoubleQuoted,
            accessor,
            extra: Location::from(input),
        },
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("\"\"")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "".to_owned(),
            variant: puppet_lang::expression::StringVariant::DoubleQuoted,
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("\"a\"")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "a".to_owned(),
            variant: puppet_lang::expression::StringVariant::DoubleQuoted,
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("\"\\\"\"")).unwrap().1,
        puppet_lang::expression::StringExpr {
            data: "\"".to_owned(),
            variant: puppet_lang::expression::StringVariant::DoubleQuoted,
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
}
