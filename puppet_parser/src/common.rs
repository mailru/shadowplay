use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{anychar, multispace0, multispace1, newline},
    combinator::{eof, map, opt, peek, recognize, value, verify},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Parser,
};

use crate::{range::Range, IResult, ParseError, Span};

pub fn shell_comment(input: Span) -> IResult<(Span, Span, Span)> {
    tuple((
        tag("#"),
        recognize(many0(is_not("\n"))),
        alt((recognize(newline), eof)),
    ))(input)
}

pub fn capture_comment(input: Span) -> IResult<Vec<puppet_lang::comment::Comment<Range>>> {
    let (input, _) = multispace0(input)?;

    let c_comment_extractor = tuple((tag("/*"), take_until("*/"), tag("*/")));

    map(
        many0(delimited(
            multispace0,
            alt((shell_comment, c_comment_extractor)),
            multispace0,
        )),
        |v: Vec<(Span, Span, Span)>| {
            v.iter()
                .map(|elt| puppet_lang::comment::Comment {
                    extra: Range::from((elt.0, elt.2)),
                    value: elt.1.to_string(),
                })
                .collect()
        },
    )(input)
}

pub fn list_with_last_comment<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<puppet_lang::List<Range, O>>
where
    F: Parser<Span<'a>, Vec<O>, ParseError<'a>>,
    O: Clone,
{
    map(pair(parser, capture_comment), |(value, last_comment)| {
        puppet_lang::List {
            value,
            last_comment,
        }
    })
}

pub fn comma_separated_list0_with_last_comment<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<puppet_lang::List<Range, O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    list_with_last_comment(terminated(
        separated_list0(comma_separator, parser),
        opt(comma_separator),
    ))
}

pub fn comma_separated_list1_with_last_comment<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<puppet_lang::List<Range, O>>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    list_with_last_comment(terminated(
        separated_list1(comma_separator, parser),
        opt(comma_separator),
    ))
}

#[test]
fn test_comment() {
    let (_, res) = shell_comment.parse(Span::new("# hello world\n")).unwrap();
    assert_eq!(*res.1, " hello world")
}

pub fn separator1(input: Span) -> IResult<()> {
    value((), multispace1)(input)
}

pub fn separator0(input: Span) -> IResult<()> {
    value((), multispace0)(input)
}

pub fn space0_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    delimited(separator0, parser, separator0)
}

pub fn space1_delimimited<'a, O, F>(parser: F) -> impl FnMut(Span<'a>) -> IResult<O>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    delimited(separator1, parser, separator1)
}

pub fn spaced0_separator<'a>(
    separator_tag: &'static str,
) -> impl FnMut(Span<'a>) -> IResult<'a, ()> {
    value((), space0_delimimited(tag(separator_tag)))
}

pub fn comma_separator(input: Span) -> IResult<()> {
    spaced0_separator(",")(input)
}

pub fn round_parens_delimimited<'a, O, F>(
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: FnMut(Span<'a>) -> IResult<O>,
    O: Clone,
{
    move |input| {
        let (input, left_paren) = tag("(")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, inner) = parser(input)?;
        let (input, right_paren) = ParseError::protect(
            |_| {
                let left_paren_location = crate::range::Location::from(left_paren);
                format!(
                    "Closing ')' expected, which was opened at line {} col {}",
                    left_paren_location.line(),
                    left_paren_location.column()
                )
            },
            preceded(multispace0, tag(")")),
        )(input)?;
        Ok((input, (left_paren, inner, right_paren)))
    }
}

pub fn square_brackets_delimimited<'a, O, F>(
    fatal_on_missing_closing_bracket: bool,
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: FnMut(Span<'a>) -> IResult<O>,
    O: Clone,
{
    move |input| {
        let (input, left_bracket) = tag("[")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, inner) = parser(input)?;
        let (input, right_bracket) = if fatal_on_missing_closing_bracket {
            ParseError::protect(
                |_| {
                    let left_bracket_location = crate::range::Location::from(left_bracket);
                    format!(
                        "Closing ']' expected, which was opened at line {} col {}",
                        left_bracket_location.line(),
                        left_bracket_location.column()
                    )
                },
                preceded(multispace0, tag("]")),
            )(input)?
        } else {
            preceded(multispace0, tag("]"))(input)?
        };
        Ok((input, (left_bracket, inner, right_bracket)))
    }
}

pub fn curly_brackets_delimimited<'a, O, F>(
    fatal_on_missing_closing_curly: bool,
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: FnMut(Span<'a>) -> IResult<O>,
    O: Clone,
{
    move |input| {
        let (input, left_curly) = tag("{")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, inner) = parser(input)?;
        let (input, right_curly) = if fatal_on_missing_closing_curly {
            ParseError::protect(
                |_| {
                    let left_curly_location = crate::range::Location::from(left_curly);
                    format!(
                        "Closing '}}' expected, which was opened at line {} col {}",
                        left_curly_location.line(),
                        left_curly_location.column()
                    )
                },
                preceded(multispace0, tag("}")),
            )(input)?
        } else {
            preceded(multispace0, tag("}"))(input)?
        };
        Ok((input, (left_curly, inner, right_curly)))
    }
}

pub fn pipes_delimimited<'a, O, F>(
    mut parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: FnMut(Span<'a>) -> IResult<O>,
    O: Clone,
{
    move |input| {
        let (input, left_pipe) = tag("|")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, inner) = parser(input)?;
        let (input, right_pipe) = ParseError::protect(
            |_| {
                let left_pipe_location = crate::range::Location::from(left_pipe);
                format!(
                    "Closing '|' expected, which was opened at line {} col {}",
                    left_pipe_location.line(),
                    left_pipe_location.column()
                )
            },
            preceded(multispace0, tag("|")),
        )(input)?;
        Ok((input, (left_pipe, inner, right_pipe)))
    }
}

pub fn round_brackets_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    round_parens_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // Optional comma at the end
        opt(comma_separator),
    ))
}

pub fn round_brackets_comma_separated1<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    round_parens_delimimited(terminated(
        separated_list1(comma_separator, parser),
        // Optional comma at the end
        opt(comma_separator),
    ))
}

pub fn word<'a>(searchword: &'static str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>> {
    terminated(
        tag(searchword),
        alt((
            map(
                peek(verify(anychar, |c| !c.is_alphanumeric() && *c != '_')),
                |_| (),
            ),
            map(eof, |_| ()),
        )),
    )
}

pub fn spaced_word<'a>(searchword: &'static str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>> {
    space0_delimimited(word(searchword))
}

pub fn square_brackets_comma_separated0<'a, O, F>(
    fatal_on_missing_closing_bracket: bool,
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    square_brackets_delimimited(
        fatal_on_missing_closing_bracket,
        terminated(
            separated_list0(comma_separator, parser),
            // Optional comma at the end
            opt(comma_separator),
        ),
    )
}

pub fn square_brackets_comma_separated1<'a, O, F>(
    fatal_on_missing_closing_bracket: bool,
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    square_brackets_delimimited(
        fatal_on_missing_closing_bracket,
        terminated(
            separated_list1(comma_separator, parser),
            // Optional comma at the end
            opt(comma_separator),
        ),
    )
}

pub fn curly_brackets_comma_separated0<'a, O, F>(
    fatal_on_missing_closing_curly: bool,
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    curly_brackets_delimimited(
        fatal_on_missing_closing_curly,
        terminated(
            separated_list0(comma_separator, parser),
            // Optional comma at the end
            opt(comma_separator),
        ),
    )
}

pub fn pipes_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    pipes_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // Optional comma at the end
        opt(comma_separator),
    ))
}

pub fn fold_many0_with_const_init<'a, F, G, O, R>(
    mut f: F,
    init: R,
    g: G,
) -> impl FnMut(Span<'a>) -> IResult<R>
where
    F: nom::Parser<Span<'a>, O, ParseError<'a>>,
    G: Fn(R, O) -> R,
    R: Clone,
{
    let mut res = init;
    move |i: Span| {
        let mut input = i;

        loop {
            let i_ = input;
            let len = input.len();
            match f.parse(i_) {
                Ok((i, o)) => {
                    // infinite loop check: the parser must always consume
                    if i.len() == len {
                        return Err(nom::Err::Error(ParseError::new(
                            "Parsed empty token in list".to_string(),
                            input,
                            None,
                        )));
                    }

                    res = g(res.clone(), o);
                    input = i;
                }
                Err(nom::Err::Error(_)) => {
                    return Ok((input, res.clone()));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

#[test]
fn test_round_brackets_comma_separated0() {
    assert_eq!(
        round_brackets_comma_separated0(tag("a"))(Span::new("( a,a ,a, a,)"))
            .unwrap()
            .1
             .1
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<_>>(),
        vec!["a", "a", "a", "a"]
    );
    assert_eq!(
        round_brackets_comma_separated0(round_brackets_comma_separated0(tag("a")))(Span::new(
            "( (a) , (a) ,(   a   ), ( a ) )"
        ))
        .unwrap()
        .1
         .1
        .into_iter()
        .map(|v| v.1.into_iter().map(|v| *v).collect::<Vec<_>>())
        .collect::<Vec<_>>(),
        vec![vec!["a"], vec!["a"], vec!["a"], vec!["a"]]
    )
}

#[test]
fn test_square_brackets_comma_separated0() {
    assert_eq!(
        square_brackets_comma_separated0(true, tag("a"))(Span::new("[a]"))
            .unwrap()
            .1
             .1
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<_>>(),
        vec!["a"]
    );
    assert_eq!(
        square_brackets_comma_separated0(true, tag("a"))(Span::new("[a,]"))
            .unwrap()
            .1
             .1
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<_>>(),
        vec!["a"]
    )
}
