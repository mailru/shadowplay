use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace1, newline},
    combinator::{opt, recognize, value},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    Parser,
};

use super::parser::{IResult, ParseError, Span};

pub fn comment(input: Span) -> IResult<()> {
    let comment_extractor = preceded(char('#'), recognize(many0(is_not("\n\r"))));

    value((), terminated(comment_extractor, opt(newline)))(input)
}

#[test]
fn test_comment() {
    let (_, res) = comment.parse(Span::new("# hello world\n")).unwrap();
    assert_eq!(res, ())
}

pub fn separator1(input: Span) -> IResult<()> {
    value(
        (),
        many1(nom::branch::alt((value((), multispace1), comment))),
    )(input)
}

pub fn separator0(input: Span) -> IResult<()> {
    value(
        (),
        many0(nom::branch::alt((value((), multispace1), comment))),
    )(input)
}

#[test]
fn test_separator() {
    let (_, res) = delimited(separator1, tag("aaa"), separator1)
        .parse(Span::new("#sdfsdf\n#sdfsdf\naaa# hello world\n#sdfsdf"))
        .unwrap();
    assert_eq!(*res, "aaa")
}

pub fn space0_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    delimited(separator0, parser, separator0)
}

pub fn spaced0_separator<'a>(
    separator_tag: &'static str,
) -> impl FnMut(Span<'a>) -> IResult<'a, ()> {
    value((), space0_delimimited(tag(separator_tag)))
}

pub fn comma_separator(input: Span) -> IResult<()> {
    spaced0_separator(",")(input)
}

pub fn round_brackets_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        delimited(
            pair(tag("("), separator0),
            parser,
            ParseError::protect(
                |_| "Closing ')' expected".to_string(),
                pair(separator0, tag(")")),
            ),
        ),
    )
}

pub fn square_brackets_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        delimited(
            pair(tag("["), separator0),
            parser,
            ParseError::protect(
                |_| "Closing ']' expected".to_string(),
                pair(separator0, tag("]")),
            ),
        ),
    )
}

pub fn curly_brackets_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        opt(separator1),
        delimited(
            pair(tag("{"), separator0),
            parser,
            ParseError::protect(
                |_| "Closing '}' expected".to_string(),
                pair(separator0, tag("}")),
            ),
        ),
    )
}

pub fn round_brackets_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    round_brackets_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

#[test]
fn test_round_brackets_comma_separated0() {
    let input = Span::new("( a,a ,a, a,)");
    assert_eq!(
        round_brackets_comma_separated0(tag("a"))(input)
            .unwrap()
            .1
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<_>>(),
        vec!["a", "a", "a", "a"]
    )
}

pub fn square_brackets_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    square_brackets_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

pub fn square_brackets_comma_separated1<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    square_brackets_delimimited(terminated(
        separated_list1(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

pub fn curly_brackets_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<Vec<O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    curly_brackets_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}