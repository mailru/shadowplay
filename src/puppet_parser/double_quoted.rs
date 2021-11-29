use super::parser::{IResult, Marked, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::{is_not, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::{map, map_opt, map_res, value, verify};
use nom::multi::fold_many0;
use nom::sequence::{delimited, preceded};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_literal(input: Span) -> IResult<&str> {
    let not_quote_slash = is_not("\"\\");
    verify(map(not_quote_slash, |s| Marked::new(&s, *s)), |s| {
        !s.data.is_empty()
    })(input)
}

fn parse_unicode(input: Span) -> IResult<char> {
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));

    let parse_u32 = map_res(parse_delimited_hex, move |hex: Span| {
        u32::from_str_radix(*hex, 16)
    });

    map_opt(parse_u32, |v| {
        std::char::from_u32(v).map(|v| Marked::new(&input, v))
    })(input)
    .map_err(|e| match e {
        nom::Err::Error(_) => nom::Err::Failure(ParseError::new(
            "unexpected sequence in UTF character".to_owned(),
            input,
        )),
        e => e,
    })
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
                    value('\"', char('\"')),
                )),
                |v: char| Marked::new(&input, v),
            ),
            parse_unicode,
            |s: Span| {
                Err(nom::Err::Failure(ParseError::new(
                    format!("Unexpected escape sequence \\{}", s),
                    input,
                )))
            },
        )),
    )(input)
}

fn parse_fragment(input: Span) -> IResult<StringFragment> {
    alt((
        map(parse_literal, |m| m.map(StringFragment::Literal)),
        map(parse_escaped_char, |m| m.map(StringFragment::EscapedChar)),
    ))(input)
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

    delimited(char('"'), build_string, char('"'))(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("\"\"")).unwrap().1,
        Marked {
            data: "".to_owned(),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        parse(Span::new("\"a\"")).unwrap().1,
        Marked {
            data: "a".to_owned(),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        parse(Span::new("\"\\\"\"")).unwrap().1,
        Marked {
            data: "\"".to_owned(),
            line: 1,
            column: 1
        }
    );
}
