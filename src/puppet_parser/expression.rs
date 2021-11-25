use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    error::FromExternalError,
    multi::many0,
    number::complete::float,
    sequence::{pair, preceded},
    IResult,
};

use super::common::square_brackets_delimimited;

pub fn identifier_with_toplevel<'a, E>(input: &'a str) -> IResult<&'a str, (bool, Vec<&'a str>), E>
where
    E: nom::error::ParseError<&'a str>,
{
    pair(
        map(opt(tag("::")), |v| v.is_some()),
        super::common::lower_identifier_with_ns,
    )(input)
}

fn variable_base<'a, E>(input: &'a str) -> IResult<&'a str, (bool, Vec<&'a str>), E>
where
    E: nom::error::ParseError<&'a str>,
{
    preceded(tag("$"), identifier_with_toplevel)(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: Vec<String>,
    is_toplevel: bool,
    accessor: Vec<Expression>,
}

impl Variable {
    pub fn parser<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let accessor_parser = many0(square_brackets_delimimited(Expression::parse));

        map(
            pair(variable_base, accessor_parser),
            |((is_toplevel, name), accessor)| Self {
                is_toplevel,
                name: name.into_iter().map(String::from).collect(),
                accessor,
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    is_toplevel: bool,
    name: Vec<String>,
    args: Vec<Term>,
}

impl FunctionCall {
    fn parser<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map(
            pair(
                identifier_with_toplevel,
                super::common::round_brackets_comma_separated0(Term::parse),
            ),
            |((is_toplevel, name), args)| Self {
                is_toplevel,
                name: name.into_iter().map(String::from).collect(),
                args,
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    SingleQuoted(String),
    DoubleQuoted(String),
    Float(f32),
    Boolean(bool),
    Array(Vec<Term>),
    Map(Vec<(Term, Term)>),
    Undef,
    Variable(Variable),
    FunctionCall(FunctionCall),
    TypeSpecitifaction(super::typing::TypeSpecification),
}

impl Term {
    fn map_parser<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let kv_parser = pair(
            super::common::space0_delimimited(Self::parse),
            preceded(tag("=>"), super::common::space0_delimimited(Self::parse)),
        );

        let parser = super::common::curly_brackets_comma_separated0(kv_parser);

        map(parser, Self::Map)(input)
    }

    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        alt((
            map(tag("undef"), |_| Self::Undef),
            map(tag("true"), |_| Self::Boolean(true)),
            map(tag("false"), |_| Self::Boolean(false)),
            map(float, Self::Float),
            map(FunctionCall::parser, Self::FunctionCall),
            map(super::double_quoted::parse, Self::DoubleQuoted),
            map(super::single_quoted::parse, Self::SingleQuoted),
            map(
                super::common::square_brackets_comma_separated0(Term::parse),
                Self::Array,
            ),
            Self::map_parser,
            map(Variable::parser, Self::Variable),
            map(
                super::typing::TypeSpecification::parse,
                Self::TypeSpecitifaction,
            ),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Multiply((Box<Expression>, Box<Expression>)),
    Divide((Box<Expression>, Box<Expression>)),
    Plus((Box<Expression>, Box<Expression>)),
    Minus((Box<Expression>, Box<Expression>)),
    Term(Term),
}

impl Expression {
    pub fn fold_many0<'a, E, F, G, O, R>(
        mut f: F,
        init: R,
        g: G,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, R, E>
    where
        F: nom::Parser<&'a str, O, E>,
        G: Fn(R, O) -> R,
        E: nom::error::ParseError<&'a str>,
        R: Clone,
    {
        let mut res = init;
        move |i: &'a str| {
            let mut input = i;

            loop {
                let i_ = input;
                let len = input.len();
                match f.parse(i_) {
                    Ok((i, o)) => {
                        // infinite loop check: the parser must always consume
                        if i.len() == len {
                            return Err(nom::Err::Error(E::from_error_kind(
                                input,
                                nom::error::ErrorKind::Many0,
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

    fn parse_l1<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let (input, term) = map(super::common::space0_delimimited(Term::parse), |v| {
            Self::Term(v)
        })(input)?;
        Self::fold_many0(
            pair(
                alt((tag("*"), tag("/"))),
                super::common::space0_delimimited(Self::parse_l1),
            ),
            term,
            |prev, (op, cur)| match op {
                "*" => Expression::Multiply((Box::new(prev), Box::new(cur))),
                "/" => Expression::Divide((Box::new(prev), Box::new(cur))),
                _ => unreachable!(),
            },
        )(input)
    }

    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let (input, l1) = super::common::space0_delimimited(Self::parse_l1)(input)?;
        Self::fold_many0(
            pair(
                alt((tag("+"), tag("-"))),
                super::common::space0_delimimited(Self::parse_l1),
            ),
            l1,
            |prev, (op, cur)| match op {
                "+" => Expression::Plus((Box::new(prev), Box::new(cur))),
                "-" => Expression::Minus((Box::new(prev), Box::new(cur))),
                _ => unreachable!(),
            },
        )(input)
    }
}

#[test]
fn test_single_quoted() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("'aaa'").unwrap(),
        ("", Term::SingleQuoted("aaa".to_owned()))
    );
}

#[test]
fn test_double_quoted() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("\"aaa\"").unwrap(),
        ("", Term::DoubleQuoted("aaa".to_owned()))
    );
}

#[test]
fn test_float() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("12345").unwrap(),
        ("", Term::Float(12345.0))
    );
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("12345.1").unwrap(),
        ("", Term::Float(12345.1))
    );
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("-12345.3").unwrap(),
        ("", Term::Float(-12345.3))
    );
}

#[test]
fn test_bool() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("true").unwrap(),
        ("", Term::Boolean(true))
    );
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("false").unwrap(),
        ("", Term::Boolean(false))
    );
}

#[test]
fn test_array() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("[]").unwrap(),
        ("", Term::Array(vec![]))
    );
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("[false]").unwrap(),
        ("", Term::Array(vec![Term::Boolean(false)]))
    );
}

#[test]
fn test_map() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("{}").unwrap(),
        ("", Term::Map(vec![]))
    );
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("{false => 1}").unwrap(),
        (
            "",
            Term::Map(vec![(Term::Boolean(false), Term::Float(1.0))])
        )
    );
    assert!(Term::parse::<nom::error::Error<_>>("{'asdasd' => {}, 'a' => 'b', }").is_ok());
}

#[test]
fn test_function_call() {
    assert_eq!(
        Term::parse::<nom::error::Error<_>>("lookup('ask8s::docker::gpu_nvidia')").unwrap(),
        (
            "",
            Term::FunctionCall(FunctionCall {
                is_toplevel: false,
                name: vec!["lookup".to_owned()],
                args: vec![Term::SingleQuoted("ask8s::docker::gpu_nvidia".to_owned())]
            })
        )
    );
}

#[test]
fn test_variable() {
    assert_eq!(
        Variable::parser::<nom::error::Error<_>>("$a").unwrap(),
        (
            "",
            Variable {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                accessor: Vec::new()
            }
        )
    );
    assert_eq!(
        Variable::parser::<nom::error::Error<_>>("$::a::b").unwrap(),
        (
            "",
            Variable {
                name: vec!["a".to_owned(), "b".to_owned()],
                is_toplevel: true,
                accessor: Vec::new()
            }
        )
    );
    assert_eq!(
        Variable::parser::<nom::error::Error<_>>("$a[ 1 ]['z']").unwrap(),
        (
            "",
            Variable {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                accessor: vec![
                    Expression::Term(Term::Float(1.0)),
                    Expression::Term(Term::SingleQuoted("z".to_owned()))
                ]
            }
        )
    );
}

#[test]
fn test_multiply() {
    assert_eq!(
        Expression::parse::<nom::error::Error<_>>("2*3").unwrap(),
        (
            "",
            Expression::Multiply((
                Box::new(Expression::Term(Term::Float(2.0))),
                Box::new(Expression::Term(Term::Float(3.0)))
            ))
        )
    );
}

#[test]
fn test_operators_precendence() {
    assert_eq!(
        Expression::parse::<nom::error::Error<_>>("1 +2 * 3* 4 - 10").unwrap(),
        (
            "",
            Expression::Minus((
                Box::new(Expression::Plus((
                    Box::new(Expression::Term(Term::Float(1.0))),
                    Box::new(Expression::Multiply((
                        Box::new(Expression::Term(Term::Float(2.0))),
                        Box::new(Expression::Multiply((
                            Box::new(Expression::Term(Term::Float(3.0))),
                            Box::new(Expression::Term(Term::Float(4.0)))
                        )))
                    )))
                ))),
                Box::new(Expression::Term(Term::Float(10.0)))
            ))
        )
    );
}
