use super::parser::{IResult, Marked, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, value, verify};
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
            ParseError::protect(
                |s: Span| {
                    format!(
                        "Unexpected escaped character {:?}",
                        s.chars().next().unwrap()
                    )
                },
                super::single_quoted::parse_unicode,
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
