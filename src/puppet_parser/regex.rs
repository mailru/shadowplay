use super::parser::{IResult, IResultUnmarked, Marked, Span};
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

fn parse_literal(input: Span) -> IResultUnmarked<Span> {
    let not_quote_slash = is_not("/\\");
    verify(not_quote_slash, |s: &Span| !s.is_empty())(input)
}

fn parse_escaped_char(input: Span) -> IResultUnmarked<Span> {
    recognize(pair(tag("\\"), anychar))(input)
}

fn parse_fragment(input: Span) -> IResultUnmarked<StringFragment> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(input)
}

pub fn parse(input: Span) -> IResult<String> {
    let build_string = fold_many0(parse_fragment, String::new, |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(&s),
            StringFragment::EscapedChar(s) => string.push_str(&s),
        }
        string
    });

    Marked::parse(delimited(char('/'), build_string, char('/')))(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("//")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: "".to_owned()
        }
    );
    assert_eq!(
        parse(Span::new("/aaa/")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: "aaa".to_owned()
        }
    );
    assert_eq!(
        parse(Span::new("/\\//")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: "\\/".to_owned()
        }
    );
    assert_eq!(
        parse(Span::new("/\\d/")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: "\\d".to_owned()
        }
    );
}
