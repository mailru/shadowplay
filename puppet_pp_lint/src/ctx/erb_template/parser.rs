use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_until},
    character::complete::{alpha1, alphanumeric1, anychar, char},
    combinator::{map, peek, recognize, verify},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
};

pub fn parse_literal(input: &str) -> nom::IResult<&str, &str, nom::error::Error<&str>> {
    verify(take_until("<%"), |s: &str| !s.is_empty())(input)
}

pub fn parse_variable(input: &str) -> nom::IResult<&str, &str, nom::error::Error<&str>> {
    preceded(
        tag("@"),
        recognize(tuple((
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))),
    )(input)
}

pub fn parse_single_quoted(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    let escaped = map(pair(char('\\'), anychar), |_| ());
    let literal = map(verify(is_not("\\'"), |s: &str| !s.is_empty()), |_| ());

    map(
        delimited(char('\''), many0(alt((literal, escaped))), char('\'')),
        |_| (),
    )(input)
}

pub fn parse_double_quoted(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    let escaped = map(pair(char('\\'), anychar), |_| ());
    let literal = map(verify(is_not("\\\""), |s: &str| !s.is_empty()), |_| ());

    map(
        delimited(char('"'), many0(alt((literal, escaped))), char('"')),
        |_| (),
    )(input)
}

pub fn parse_unwant_code_fragment(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    let (input, next_byte) = preceded(
        verify(is_not("%@'\""), |s: &str| !s.is_empty()),
        peek(take(1usize)),
    )(input)?;

    match next_byte {
        "%" => {
            let (_, next_two_bytes) = peek(take(2usize))(input)?;
            if next_two_bytes == "%>" {
                return Ok((input, ()));
            }
            map(take(1usize), |_| ())(input)
        }
        _ => Ok((input, ())),
    }
}

pub fn parse_code_fragment(
    input: &str,
) -> nom::IResult<&str, Option<String>, nom::error::Error<&str>> {
    alt((
        map(parse_unwant_code_fragment, |_| None),
        map(parse_variable, |v| Some(v.to_string())),
        map(parse_single_quoted, |_| None),
        map(parse_double_quoted, |_| None),
    ))(input)
}

pub fn parse_code_block(input: &str) -> nom::IResult<&str, Vec<String>, nom::error::Error<&str>> {
    let parser = delimited(tag("<%"), many0(parse_code_fragment), tag("%>"));

    map(parser, |list| list.into_iter().flatten().collect())(input)
}

pub fn parse_toplevel(
    input: &str,
) -> nom::IResult<&str, crate::ctx::erb_template::Template, nom::error::Error<&str>> {
    let parser = alt((map(parse_literal, |_| None), map(parse_code_block, Some)));

    map(many0(parser), |list| crate::ctx::erb_template::Template {
        referenced_variables: list.into_iter().flatten().flatten().collect(),
    })(input)
}

#[test]
fn test_code_fragment() {
    assert_eq!(parse_code_fragment("@asd ").unwrap().0, " ");
    assert_eq!(parse_code_fragment(" @asd").unwrap().0, "@asd");
    assert_eq!(parse_code_fragment(" @a_sd").unwrap().0, "@a_sd");
    assert_eq!(parse_code_fragment(" % ").unwrap().0, " ")
}

#[test]
fn test_code_block() {
    assert_eq!(
        parse_code_block("<% @a1  @a2  @a3 %>").unwrap().1,
        vec!["a1".to_string(), "a2".to_string(), "a3".to_string()]
    )
}

#[test]
fn test_toplevel() {
    let mut referenced_variables = std::collections::HashSet::new();
    let _ = referenced_variables.insert("a1".to_owned());
    let _ = referenced_variables.insert("a2".to_owned());
    let _ = referenced_variables.insert("a3".to_owned());
    let _ = referenced_variables.insert("a4".to_owned());
    let _ = referenced_variables.insert("a_5".to_owned());
    assert_eq!(
        parse_toplevel("literal <% @a1  ' '  @a2 %  @a3 \" \\\\ \\\" \" @a4 %> %%%% <%= @a_5 %>  ")
            .unwrap()
            .1,
        crate::ctx::erb_template::Template {
            referenced_variables,
        }
    )
}
