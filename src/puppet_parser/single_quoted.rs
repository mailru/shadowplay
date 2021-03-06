use crate::puppet_parser::{range::Range, IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{is_not, take_while_m_n};
use nom::character::complete::char;
use nom::character::complete::{alphanumeric1, anychar};
use nom::combinator::{map, map_opt, map_res, recognize, verify};
use nom::multi::{fold_many0, many1, separated_list1};
use nom::sequence::{pair, preceded, tuple};
use crate::puppet_lang::string::{Escaped, Literal, StringFragment, StringVariant};

fn parse_literal(input: Span) -> IResult<StringFragment<Range>> {
    let not_quote_slash = is_not("\'\\");
    map(
        verify(not_quote_slash, |s: &Span| !(*s).is_empty()),
        |data: Span| {
            StringFragment::Literal(Literal {
                extra: Range::from((data, data)),
                data: data.to_string(),
            })
        },
    )(input)
}

pub fn parse_unicode(input: Span) -> IResult<StringFragment<Range>> {
    let parse_hex = ParseError::protect(
        |_| "unexpected sequence in UTF character".to_owned(),
        take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
    );

    let parse_delimited_hex = preceded(tag("u{"), pair(parse_hex, recognize(char('}'))));

    let parse_u32 = map_res(parse_delimited_hex, move |(hex, end_tag): (Span, Span)| {
        u32::from_str_radix(*hex, 16).map(|r| (r, end_tag))
    });

    let (input, tag) = tag("\\")(input)?;

    map(
        map_opt(parse_u32, |(char_code, end_tag)| {
            std::char::from_u32(char_code).map(|r| (r, end_tag))
        }),
        move |(data, end_tag)| {
            StringFragment::EscapedUTF(Escaped {
                data,
                extra: Range::from((tag, end_tag)),
            })
        },
    )(input)
}

pub fn parse_escaped(input: Span) -> IResult<StringFragment<Range>> {
    map(
        pair(recognize(char('\\')), recognize(anychar)),
        |(tag, data): (Span, Span)| {
            StringFragment::Escaped(Escaped {
                data: (*data).chars().next().unwrap(),
                extra: Range::from((tag, data)),
            })
        },
    )(input)
}

fn parse_fragment(input: Span) -> IResult<StringFragment<Range>> {
    alt((parse_literal, parse_unicode, parse_escaped))(input)
}

pub fn bareword(input: Span) -> IResult<Literal<Range>> {
    let parser = recognize(separated_list1(
        tag("-"),
        many1(alt((alphanumeric1, tag("_")))),
    ));

    map(parser, |data: Span| Literal {
        extra: Range::from((data, data)),
        data: (*data).to_owned(),
    })(input)
}

pub fn parse(input: Span) -> IResult<crate::puppet_lang::string::StringExpr<Range>> {
    let build_string = fold_many0(parse_fragment, Vec::new, |mut list, fragment| {
        list.push(fragment);
        list
    });

    alt((
        map(
            tuple((
                tag("'"),
                ParseError::protect(|_| "Unterminated quoted string".to_string(), build_string),
                tag("'"),
            )),
            |(left_tag, data, right_tag)| crate::puppet_lang::string::StringExpr {
                extra: Range::from((&left_tag, &right_tag)),
                data: StringVariant::SingleQuoted(data),
            },
        ),
        map(bareword, |word| crate::puppet_lang::string::StringExpr {
            extra: word.extra.clone(),
            data: StringVariant::SingleQuoted(vec![crate::puppet_lang::string::StringFragment::Literal(
                word,
            )]),
        }),
    ))(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("''")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::SingleQuoted(Vec::new()),
            extra: Range::new(0, 1, 1, 1, 1, 2)
        }
    );
    assert_eq!(
        parse(Span::new("'a'")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::SingleQuoted(vec![
                crate::puppet_lang::string::StringFragment::Literal(crate::puppet_lang::string::Literal {
                    data: "a".to_owned(),
                    extra: Range::new(1, 1, 2, 1, 1, 2)
                })
            ]),
            extra: Range::new(0, 1, 1, 2, 1, 3)
        }
    );
    assert_eq!(
        parse(Span::new("'\\''")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::SingleQuoted(vec![
                crate::puppet_lang::string::StringFragment::Escaped(Escaped {
                    data: '\'',
                    extra: Range::new(1, 1, 2, 2, 1, 3)
                })
            ]),
            extra: Range::new(0, 1, 1, 3, 1, 4)
        }
    );
    assert_eq!(
        parse(Span::new("bARE-WORD_")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::SingleQuoted(vec![
                crate::puppet_lang::string::StringFragment::Literal(crate::puppet_lang::string::Literal {
                    data: "bARE-WORD_".to_owned(),
                    extra: Range::new(0, 1, 1, 9, 1, 10)
                })
            ]),
            extra: Range::new(0, 1, 1, 9, 1, 10)
        }
    );

    assert_eq!(
        parse(Span::new("bAREWORD-")).unwrap().1,
        crate::puppet_lang::string::StringExpr {
            data: crate::puppet_lang::string::StringVariant::SingleQuoted(vec![
                crate::puppet_lang::string::StringFragment::Literal(crate::puppet_lang::string::Literal {
                    data: "bAREWORD".to_owned(),
                    extra: Range::new(0, 1, 1, 7, 1, 8)
                })
            ]),
            extra: Range::new(0, 1, 1, 7, 1, 8)
        }
    );
    assert!(parse(Span::new("-")).is_err());

    // Puppet bug. According to specification, it CANNOT be a bare-word
    // https://puppet.com/docs/puppet/7/lang_data_string.html#lang_data_string_bare_words
    assert!(parse(Span::new("BEDA")).is_ok());
}
