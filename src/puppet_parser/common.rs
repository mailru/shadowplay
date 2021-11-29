use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, anychar, char, digit1, multispace1, newline},
    combinator::{map, opt, recognize, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Parser,
};

use super::parser::{IResult, IResultUnit, IResultUnmarked, Marked, ParseError, Span};

pub fn comment(input: Span) -> IResultUnit {
    let comment_extractor = preceded(char('#'), recognize(many0(is_not("\n\r"))));

    value((), terminated(comment_extractor, opt(newline)))(input)
}

#[test]
fn test_comment() {
    let (_, res) = comment.parse(Span::new("# hello world\n")).unwrap();
    assert_eq!(res, ())
}

pub fn separator1(input: Span) -> IResult<()> {
    Marked::parse(value(
        (),
        many1(nom::branch::alt((value((), multispace1), comment))),
    ))(input)
}

pub fn separator0(input: Span) -> IResult<()> {
    Marked::parse(value(
        (),
        many0(nom::branch::alt((value((), multispace1), comment))),
    ))(input)
}

#[test]
fn test_separator() {
    let (_, res) = delimited(separator1, tag("aaa"), separator1)
        .parse(Span::new("#sdfsdf\n#sdfsdf\naaa# hello world\n#sdfsdf"))
        .unwrap();
    assert_eq!(
        Marked::from(res),
        Marked {
            data: "aaa",
            line: 3,
            column: 1
        }
    )
}

pub fn char_lower(input: Span) -> IResult<char> {
    map(verify(anychar, |c| c.is_ascii_lowercase()), |c| {
        Marked::new(&input, c)
    })(input)
}

pub fn char_upper(input: Span) -> IResult<char> {
    map(verify(anychar, |c| c.is_ascii_uppercase()), |c| {
        Marked::new(&input, c)
    })(input)
}

pub fn identifier(input: Span) -> IResult<&str> {
    map(
        recognize(tuple((alpha1, many0(alt((alphanumeric1, tag("_"))))))),
        Marked::from,
    )(input)
}

pub fn lowercase_identifier(input: Span) -> IResult<&str> {
    map(
        recognize(tuple((
            char_lower,
            many0(alt((recognize(char_lower), digit1, tag("_")))),
        ))),
        Marked::from,
    )(input)
}

pub fn camel_case_identifier(input: Span) -> IResult<&str> {
    map(
        recognize(tuple((char_upper, many0(alt((alphanumeric1, tag("_"))))))),
        Marked::from,
    )(input)
}

pub fn space0_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResultUnmarked<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    delimited(separator0, parser, separator0)
}

pub fn spaced0_separator<'a>(
    separator_tag: &'static str,
) -> impl FnMut(Span<'a>) -> IResultUnit<'a> {
    value(
        (),
        space0_delimimited(map(tag(separator_tag), Marked::from)),
    )
}

pub fn comma_separator(input: Span) -> IResultUnit {
    spaced0_separator(",")(input)
}

pub fn lower_identifier_with_ns(input: Span) -> IResult<Vec<Marked<&str>>> {
    map(
        nom::multi::separated_list1(tag("::"), lowercase_identifier),
        |v| Marked::new(&input, v),
    )(input)
}

#[test]
fn test_lower_case_identifier_with_ns() {
    assert_eq!(
        lower_identifier_with_ns(Span::new("asd")).unwrap().1,
        Marked {
            data: vec![Marked {
                data: "asd",
                line: 1,
                column: 1
            }],
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        lower_identifier_with_ns(Span::new("asd::def")).unwrap().1,
        Marked {
            data: vec![
                Marked {
                    data: "asd",
                    line: 1,
                    column: 1
                },
                Marked {
                    data: "def",
                    line: 1,
                    column: 6
                }
            ],
            line: 1,
            column: 1
        }
    );
    assert!(lower_identifier_with_ns(Span::new("")).is_err())
}

pub fn camelcase_identifier_with_ns(input: Span) -> IResult<Vec<Marked<&str>>> {
    map(
        nom::multi::separated_list1(tag("::"), camel_case_identifier),
        |v| Marked::new(&input, v),
    )(input)
}

pub fn round_brackets_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        delimited(
            pair(tag("("), separator0),
            parser,
            pair(separator0, tag(")")),
        ),
    )
}

pub fn square_brackets_delimimited<'a, O, F>(mut parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        delimited(
            pair(tag("["), separator0),
            move |i| parser.parse(i),
            pair(separator0, tag("]")),
        ),
    )
}

pub fn curly_brackets_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    preceded(
        opt(separator1),
        delimited(
            pair(tag("{"), separator0),
            parser,
            pair(separator0, tag("}")),
        ),
    )
}

pub fn round_brackets_comma_separated0<'a, O, F>(
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<Marked<O>>>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    move |input: Span| {
        round_brackets_delimimited(map(
            terminated(
                separated_list0(comma_separator, |i| parser.parse(i)),
                // В конце не обязательная запятая
                opt(comma_separator),
            ),
            |v| Marked::new(&input, v),
        ))(input)
    }
}

#[test]
fn test_round_brackets_comma_separated0() {
    let input = Span::new("( a,a ,a, a,)");
    assert_eq!(
        round_brackets_comma_separated0(map(tag("a"), Marked::from))(input)
            .unwrap()
            .1,
        Marked {
            data: vec![
                Marked {
                    data: "a",
                    line: 1,
                    column: 3,
                },
                Marked {
                    data: "a",
                    line: 1,
                    column: 5,
                },
                Marked {
                    data: "a",
                    line: 1,
                    column: 8,
                },
                Marked {
                    data: "a",
                    line: 1,
                    column: 11,
                },
            ],
            line: 1,
            column: 1
        }
    )
}

pub fn square_brackets_comma_separated0<'a, O, F>(
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<Marked<O>>>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    move |input: Span| {
        square_brackets_delimimited(map(
            terminated(
                separated_list0(comma_separator, |i| parser.parse(i)),
                // В конце не обязательная запятая
                opt(comma_separator),
            ),
            |v| Marked::new(&input, v),
        ))(input)
    }
}

pub fn square_brackets_comma_separated1<'a, O, F>(
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<Marked<O>>>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    move |input: Span| {
        square_brackets_delimimited(map(
            terminated(
                separated_list1(comma_separator, |i| parser.parse(i)),
                // В конце не обязательная запятая
                opt(comma_separator),
            ),
            |v| Marked::new(&input, v),
        ))(input)
    }
}

pub fn curly_brackets_comma_separated0<'a, O, F>(
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<Marked<O>>>
where
    F: Parser<Span<'a>, Marked<O>, ParseError<'a>>,
    O: Clone,
{
    move |input: Span| {
        curly_brackets_delimimited(map(
            terminated(
                separated_list0(comma_separator, |i| parser.parse(i)),
                // В конце не обязательная запятая
                opt(comma_separator),
            ),
            |v| Marked::new(&input, v),
        ))(input)
    }
}
