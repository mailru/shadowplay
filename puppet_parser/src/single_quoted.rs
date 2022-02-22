use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{is_not, take_while_m_n};
use nom::character::complete::char;
use nom::character::complete::{alphanumeric1, anychar};
use nom::combinator::{map, map_opt, map_res, recognize, verify};
use nom::multi::{fold_many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use puppet_lang::string::{Escaped, StringFragment, StringVariant};

fn parse_literal(input: Span) -> IResult<StringFragment<Location>> {
    let not_quote_slash = is_not("'\\");
    map(
        verify(map(not_quote_slash, |s: Span| *s), |s: &str| !s.is_empty()),
        |data| StringFragment::Literal(data.to_string()),
    )(input)
}

pub fn parse_unicode(input: Span) -> IResult<StringFragment<Location>> {
    let parse_hex = ParseError::protect(
        |_| "unexpected sequence in UTF character".to_owned(),
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
    );

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_u32 = map_res(parse_delimited_hex, move |hex: Span| {
        u32::from_str_radix(*hex, 16)
    });

    map(
        pair(
            recognize(char('\\')),
            map_opt(parse_u32, std::char::from_u32),
        ),
        |(tag, data)| {
            StringFragment::EscapedUTF(Escaped {
                data,
                extra: Location::from(tag),
            })
        },
    )(input)
}

pub fn parse_escaped(input: Span) -> IResult<StringFragment<Location>> {
    map(pair(recognize(char('\\')), anychar), |(tag, data)| {
        StringFragment::Escaped(Escaped {
            data,
            extra: Location::from(tag),
        })
    })(input)
}

fn parse_fragment(input: Span) -> IResult<StringFragment<Location>> {
    alt((parse_literal, parse_unicode, parse_escaped))(input)
}

pub fn bareword(input: Span) -> IResult<StringFragment<Location>> {
    let parser = verify(
        recognize(separated_list1(
            tag("-"),
            many1(alt((alphanumeric1, tag("_")))),
        )),
        |s: &Span| s.chars().next().unwrap().is_ascii_lowercase(),
    );

    map(parser, |data: Span| {
        StringFragment::Literal(data.to_string())
    })(input)
}

pub fn parse(input: Span) -> IResult<puppet_lang::string::StringExpr<Location>> {
    let build_string = fold_many0(parse_fragment, Vec::new, |mut list, fragment| {
        list.push(fragment);
        list
    });

    let single_quoted_parser = alt((
        preceded(
            char('\''),
            ParseError::protect(
                |_| "Unterminated quoted string".to_string(),
                terminated(build_string, char('\'')),
            ),
        ),
        map(bareword, |word| vec![word]),
    ));

    map(
        pair(single_quoted_parser, crate::term::parse_accessor),
        |(data, accessor)| puppet_lang::string::StringExpr {
            data: StringVariant::SingleQuoted(data),
            accessor,
            extra: Location::from(input),
        },
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("''")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::SingleQuoted(Vec::new()),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("'a'")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                puppet_lang::string::StringFragment::Literal("a".to_owned())
            ]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("'\\''")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                puppet_lang::string::StringFragment::Escaped(Escaped {
                    data: '\'',
                    extra: Location::new(1, 1, 2)
                })
            ]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("bARE-WORD_")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                puppet_lang::string::StringFragment::Literal("bARE-WORD_".to_owned())
            ]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );

    assert_eq!(
        parse(Span::new("bAREWORD-")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::SingleQuoted(vec![
                puppet_lang::string::StringFragment::Literal("bAREWORD".to_owned())
            ]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert!(parse(Span::new("-")).is_err());
    assert!(parse(Span::new("BEDA")).is_err());
}
