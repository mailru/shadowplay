use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, alphanumeric1, anychar, char, digit1, multispace1, newline},
    combinator::{opt, recognize, value, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

pub fn comment<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: nom::error::ParseError<&'a str>,
{
    let comment_extractor = preceded(char('#'), recognize(many0(is_not("\n\r"))));

    terminated(comment_extractor, opt(newline))(input)
}

#[test]
fn test_comment() {
    let (_, res) = comment::<nom::error::Error<_>>
        .parse("# hello world\n")
        .unwrap();
    assert_eq!(res, " hello world")
}

pub fn separator1<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: nom::error::ParseError<&'a str>,
{
    value((), many1(nom::branch::alt((multispace1, comment))))(input)
}

pub fn separator0<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: nom::error::ParseError<&'a str>,
{
    value((), many0(nom::branch::alt((multispace1, comment))))(input)
}

#[test]
fn test_separator() {
    let (_, res) =
        delimited::<_, _, _, _, nom::error::Error<_>, _, _, _>(separator1, tag("aaa"), separator1)
            .parse("#sdfsdf\n#sdfsdf\naaa# hello world\n#sdfsdf")
            .unwrap();
    assert_eq!(res, "aaa")
}

pub fn char_lower<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: nom::error::ParseError<&'a str>,
{
    verify(anychar, |c| c.is_ascii_lowercase())(input)
}

pub fn char_upper<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: nom::error::ParseError<&'a str>,
{
    verify(anychar, |c| c.is_ascii_uppercase())(input)
}

pub fn identifier<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: nom::error::ParseError<&'a str>,
{
    recognize(tuple((alpha1, many0(alt((alphanumeric1, tag("_")))))))(input)
}

pub fn lowercase_identifier<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: nom::error::ParseError<&'a str>,
{
    recognize(tuple((
        char_lower,
        many0(alt((recognize(char_lower), digit1, tag("_")))),
    )))(input)
}

pub fn camel_case_identifier<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: nom::error::ParseError<&'a str>,
{
    recognize(tuple((char_upper, many0(alt((alphanumeric1, tag("_")))))))(input)
}

pub fn space0_delimimited<'a, O, F, E>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    delimited(separator0, parser, separator0)
}

pub fn spaced0_separator<'a, E>(
    separator_tag: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
where
    E: nom::error::ParseError<&'a str>,
{
    value((), space0_delimimited(tag(separator_tag)))
}

pub fn comma_separator<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: nom::error::ParseError<&'a str>,
{
    spaced0_separator(",")(input)
}

pub fn lower_identifier_with_ns<'a, E>(input: &'a str) -> IResult<&'a str, Vec<&'a str>, E>
where
    E: nom::error::ParseError<&'a str>,
{
    nom::multi::separated_list1(tag("::"), lowercase_identifier)(input)
}

#[test]
fn test_lower_case_identifier_with_ns() {
    assert_eq!(
        lower_identifier_with_ns::<nom::error::Error<_>>("asd").unwrap(),
        ("", vec!["asd"])
    );
    assert_eq!(
        lower_identifier_with_ns::<nom::error::Error<_>>("asd::def").unwrap(),
        ("", vec!["asd", "def"])
    );
    assert_eq!(
        lower_identifier_with_ns(""),
        Err(nom::Err::Error(nom::error::Error::new(
            "",
            nom::error::ErrorKind::Eof
        )))
    )
}

pub fn camelcase_identifier_with_ns<'a, E>(input: &'a str) -> IResult<&'a str, Vec<&'a str>, E>
where
    E: nom::error::ParseError<&'a str>,
{
    nom::multi::separated_list1(tag("::"), camel_case_identifier)(input)
}

pub fn round_brackets_delimimited<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
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

pub fn square_brackets_delimimited<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    preceded(
        separator0,
        delimited(
            pair(tag("["), separator0),
            parser,
            pair(separator0, tag("]")),
        ),
    )
}

pub fn curly_brackets_delimimited<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
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

pub fn round_brackets_comma_separated0<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    round_brackets_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

#[test]
fn test_round_brackets_comma_separated0() {
    assert_eq!(
        round_brackets_comma_separated0::<_, _, nom::error::Error<_>>(tag("a"))("( a,a ,a, a,)")
            .unwrap(),
        ("", vec!["a", "a", "a", "a"])
    )
}

pub fn square_brackets_comma_separated0<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    square_brackets_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

pub fn square_brackets_comma_separated1<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    square_brackets_delimimited(terminated(
        separated_list1(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}

pub fn curly_brackets_comma_separated0<'a, O, F, E>(
    parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: nom::error::ParseError<&'a str>,
{
    curly_brackets_delimimited(terminated(
        separated_list0(comma_separator, parser),
        // В конце не обязательная запятая
        opt(comma_separator),
    ))
}
