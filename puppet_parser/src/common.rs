use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{anychar, char, multispace1, newline},
    combinator::{opt, peek, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    Parser,
};

use crate::{IResult, ParseError, Span};

pub fn comment(input: Span) -> IResult<()> {
    let shell_comment_extractor = value(
        (),
        tuple((preceded(char('#'), opt(is_not("\n"))), opt(newline))),
    );
    let c_comment_extractor = value((), tuple((tag("/*"), take_until("*/"), tag("*/"))));

    alt((shell_comment_extractor, c_comment_extractor))(input)
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

pub fn round_brackets_delimimited<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        tuple((
            terminated(tag("("), separator0),
            parser,
            ParseError::protect(
                |_| "Closing ')' expected".to_string(),
                preceded(separator0, tag(")")),
            ),
        )),
    )
}

pub fn square_brackets_delimimited<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        tuple((
            terminated(tag("["), separator0),
            parser,
            ParseError::protect(
                |_| "Closing ']' expected".to_string(),
                preceded(separator0, tag("]")),
            ),
        )),
    )
}

pub fn curly_brackets_delimimited<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        tuple((
            terminated(tag("{"), separator0),
            parser,
            ParseError::protect(
                |_| "Closing '}' expected".to_string(),
                preceded(separator0, tag("}")),
            ),
        )),
    )
}

pub fn pipes_delimimited<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, O, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    preceded(
        separator0,
        tuple((
            terminated(tag("|"), separator0),
            parser,
            ParseError::protect(
                |_| "Closing '|' expected".to_string(),
                preceded(separator0, tag("|")),
            ),
        )),
    )
}

pub fn round_brackets_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
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

pub fn round_brackets_comma_separated1<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    round_brackets_delimimited(terminated(
        separated_list1(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

pub fn word<'a>(searchword: &'static str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>> {
    terminated(
        tag(searchword),
        peek(verify(anychar, |c| !c.is_alphanumeric() && *c != '_')),
    )
}

pub fn spaced_word<'a>(searchword: &'static str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>> {
    space0_delimimited(word(searchword))
}

pub fn square_brackets_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
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
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
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
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
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

pub fn pipes_comma_separated0<'a, O, F>(
    parser: F,
) -> impl FnMut(Span<'a>) -> IResult<(Span<'a>, Vec<O>, Span<'a>)>
where
    F: Parser<Span<'a>, O, ParseError<'a>>,
    O: Clone,
{
    pipes_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
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
        square_brackets_comma_separated0(tag("a"))(Span::new("[a]"))
            .unwrap()
            .1
             .1
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<_>>(),
        vec!["a"]
    );
    assert_eq!(
        square_brackets_comma_separated0(tag("a"))(Span::new("[a,]"))
            .unwrap()
            .1
             .1
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<_>>(),
        vec!["a"]
    )
}
