use super::parser::{IResult, Marked, ParseError, Span};
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
    verify(map(not_quote_slash, |s| Marked::new(&s, *s)), |s| {
        !s.data.is_empty()
    })(input)
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

    map_opt(parse_u32, |v| {
        std::char::from_u32(v).map(|v| Marked::new(&input, v))
    })(input)
}

fn parse_escaped_char(input: Span) -> IResult<char> {
    preceded(
        char('\\'),
        alt((
            map(
                alt((
                    value('\n', char('n')),
                    value('\r', char('r')),
                    value('\t', char('t')),
                    value('\u{08}', char('b')),
                    value('\u{0C}', char('f')),
                    value('\\', char('\\')),
                    value('\'', char('\'')),
                )),
                |v: char| Marked::new(&input, v),
            ),
            ParseError::protect(
                |s: Span| {
                    format!(
                        "Unexpected escaped character {:?}",
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
        map(parse_literal, |m| m.map(StringFragment::Literal)),
        map(parse_escaped_char, |m| m.map(StringFragment::EscapedChar)),
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

    map(parser, |v: Span| Marked::new(&input, v.to_string()))(input)
}

// TODO возможно имеет смысл в будущем возвращать Vec<StringFragment>, чтобы парсить содержимое
pub fn parse(input: Span) -> IResult<String> {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment.data {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    });

    let build_string = map(build_string, |s: String| Marked::new(&input, s));

    alt((
        preceded(
            char('\''),
            ParseError::protect(
                |_| "Unterminated quoted string".to_string(),
                terminated(build_string, char('\'')),
            ),
        ),
        bareword,
    ))(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("''")).unwrap().1,
        Marked {
            data: "".to_owned(),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        parse(Span::new("'a'")).unwrap().1,
        Marked {
            data: "a".to_owned(),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        parse(Span::new("'\\''")).unwrap().1,
        Marked {
            data: "'".to_owned(),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        parse(Span::new("bARE-WORD_")).unwrap().1,
        Marked {
            data: "bARE-WORD_".to_owned(),
            line: 1,
            column: 1
        }
    );

    assert_eq!(
        parse(Span::new("bAREWORD-")).unwrap().1,
        Marked {
            data: "bAREWORD".to_owned(),
            line: 1,
            column: 1
        }
    );
    assert!(parse(Span::new("-")).is_err());
    assert!(parse(Span::new("BEDA")).is_err());
}
