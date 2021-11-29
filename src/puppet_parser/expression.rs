use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded},
};

use crate::puppet_parser::common::round_brackets_delimimited;

use super::parser::{IResult, IResultUnmarked, Marked, ParseError, Span};

use super::common::square_brackets_delimimited;

pub fn identifier_with_toplevel(input: Span) -> IResult<(bool, Vec<&str>)> {
    Marked::parse(pair(
        map(opt(tag("::")), |v| v.is_some()),
        map(super::common::lower_identifier_with_ns, |v| {
            v.data.into_iter().map(|v| v.data).collect()
        }),
    ))(input)
}

fn variable_base(input: Span) -> IResult<(bool, Vec<&str>)> {
    preceded(tag("$"), identifier_with_toplevel)(input)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: Vec<String>,
    is_toplevel: bool,
    accessor: Vec<Marked<Expression>>,
}

impl Variable {
    pub fn parser(input: Span) -> IResult<Self> {
        let accessor_parser = many0(square_brackets_delimimited(Expression::parse));

        map(
            pair(variable_base, accessor_parser),
            |(variable_base, accessor)| {
                let (is_toplevel, name) = variable_base.data;
                Marked::new(
                    &input,
                    Self {
                        is_toplevel,
                        name: name.into_iter().map(String::from).collect(),
                        accessor,
                    },
                )
            },
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    is_toplevel: bool,
    name: Vec<String>,
    args: Vec<Marked<Expression>>,
}

impl FunctionCall {
    fn parser(input: Span) -> IResult<Self> {
        map(
            pair(
                identifier_with_toplevel,
                super::common::round_brackets_comma_separated0(Expression::parse),
            ),
            |(identifier, args)| {
                let (is_toplevel, name) = identifier.data;
                Marked::new(
                    &input,
                    Self {
                        is_toplevel,
                        name: name.into_iter().map(String::from).collect(),
                        args: args.data,
                    },
                )
            },
        )(input)
    }
}

pub struct Float;

impl Float {
    pub fn plain_parse(input: Span) -> IResultUnmarked<f32> {
        let number = delimited(digit1, alt((tag("e"), tag("E"), tag("."))), digit1);
        let (tail, s) = Marked::parse(recognize(pair(opt(tag("-")), number)))(input)?;

        let f = match s.data.parse::<f32>() {
            Ok(v) => v,
            Err(err) => return ParseError::fatal(format!("{}", err), input),
        };

        Ok((tail, f))
    }

    pub fn parse(input: Span) -> IResultUnmarked<Term> {
        map(Self::plain_parse, Term::Float)(input)
    }
}

pub struct Integer;

impl Integer {
    pub fn parse(input: Span) -> IResultUnmarked<Term> {
        let (tail, s) = Marked::parse(recognize(pair(opt(tag("-")), digit1)))(input)?;

        let n = match s.data.parse::<i64>() {
            Ok(v) => v,
            Err(err) => return ParseError::fatal(format!("{}", err), input),
        };

        Ok((tail, Term::Integer(n)))
    }
}

pub struct Sensitive;

impl Sensitive {
    pub fn parse(input: Span) -> IResultUnmarked<Term> {
        preceded(
            tag("Sensitive"),
            map(
                ParseError::protect(
                    |_| "Expected round brackets after Sensitive value".to_string(),
                    round_brackets_delimimited(ParseError::protect(
                        |_| "Expected single quoted string".to_string(),
                        super::single_quoted::parse,
                    )),
                ),
                |v| Term::Sensitive(v.data),
            ),
        )(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    SingleQuoted(String),
    DoubleQuoted(String),
    Float(f32),
    Integer(i64),
    Boolean(bool),
    Array(Vec<Marked<Expression>>),
    Map(Vec<(Marked<Expression>, Marked<Expression>)>),
    Undef,
    Variable(Variable),
    FunctionCall(FunctionCall),
    Sensitive(String),
    TypeSpecitifaction(super::typing::TypeSpecification),
}

impl Term {
    fn map_parser(input: Span) -> IResultUnmarked<Self> {
        let kv_parser = move |input| {
            map(
                pair(
                    super::common::space0_delimimited(Expression::parse),
                    preceded(
                        tag("=>"),
                        super::common::space0_delimimited(Expression::parse),
                    ),
                ),
                |v| Marked::new(&input, v),
            )(input)
        };

        map(
            super::common::curly_brackets_comma_separated0(kv_parser),
            |v| Self::Map(v.data.into_iter().map(|v| v.data).collect()),
        )(input)
    }

    pub fn parse(input: Span) -> IResult<Self> {
        let parser = alt((
            map(tag("undef"), |_| Self::Undef),
            map(tag("true"), |_| Self::Boolean(true)),
            map(tag("false"), |_| Self::Boolean(false)),
            Sensitive::parse,
            map(super::typing::TypeSpecification::parse, |v| {
                Self::TypeSpecitifaction(v.data)
            }),
            Float::parse,
            Integer::parse,
            map(FunctionCall::parser, |v| Self::FunctionCall(v.data)),
            map(super::double_quoted::parse, |v| Self::DoubleQuoted(v.data)),
            map(super::single_quoted::parse, |v| Self::SingleQuoted(v.data)),
            map(
                super::common::square_brackets_comma_separated0(Expression::parse),
                |v| Self::Array(v.data),
            ),
            Self::map_parser,
            map(Variable::parser, |v| Self::Variable(v.data)),
        ));

        map(parser, |v| Marked::new(&input, v))(input)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Multiply((Box<Marked<Expression>>, Box<Marked<Expression>>)),
    Divide((Box<Marked<Expression>>, Box<Marked<Expression>>)),
    Plus((Box<Marked<Expression>>, Box<Marked<Expression>>)),
    Minus((Box<Marked<Expression>>, Box<Marked<Expression>>)),
    Term(Term),
}

impl Expression {
    pub fn fold_many0<'a, F, G, O, R>(mut f: F, init: R, g: G) -> impl FnMut(Span<'a>) -> IResult<R>
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
                            )));
                        }

                        res = g(res.clone(), o);
                        input = i;
                    }
                    Err(nom::Err::Error(_)) => {
                        return Ok((input, Marked::new(&i, res.clone())));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn parse_l1(input: Span) -> IResult<Self> {
        let (input, term) = map(super::common::space0_delimimited(Term::parse), |v| {
            v.map(Self::Term)
        })(input)?;
        let parser = Self::fold_many0(
            pair(
                alt((tag("*"), tag("/"))),
                super::common::space0_delimimited(ParseError::protect(
                    |_| "Second argument of operator is expected".to_string(),
                    Self::parse_l1,
                )),
            ),
            term,
            |prev, (op, cur)| {
                let r = match *op {
                    "*" => Expression::Multiply((Box::new(prev), Box::new(cur))),
                    "/" => Expression::Divide((Box::new(prev), Box::new(cur))),
                    _ => unreachable!(),
                };
                Marked::new(&op, r)
            },
        );
        map(parser, |v| v.data)(input)
    }

    pub fn parse(input: Span) -> IResult<Self> {
        let (input, l1) = super::common::space0_delimimited(Self::parse_l1)(input)?;
        let parser = Self::fold_many0(
            pair(
                alt((tag("+"), tag("-"))),
                super::common::space0_delimimited(ParseError::protect(
                    |_| "Second argument of operator is expected".to_string(),
                    Self::parse_l1,
                )),
            ),
            l1,
            |prev, (op, cur)| {
                let r = match *op {
                    "+" => Expression::Plus((Box::new(prev), Box::new(cur))),
                    "-" => Expression::Minus((Box::new(prev), Box::new(cur))),
                    _ => unreachable!(),
                };
                Marked::new(&op, r)
            },
        );
        map(parser, |v| v.data)(input)
    }
}

#[test]
fn test_single_quoted() {
    assert_eq!(
        Term::parse(Span::new("'aaa'")).unwrap().1,
        Marked {
            data: Term::SingleQuoted("aaa".to_owned()),
            line: 1,
            column: 1
        }
    );
}

#[test]
fn test_double_quoted() {
    assert_eq!(
        Term::parse(Span::new("\"aaa\"")).unwrap().1,
        Marked {
            data: Term::DoubleQuoted("aaa".to_owned()),
            line: 1,
            column: 1
        }
    );
}

#[test]
fn test_numbers() {
    assert_eq!(
        Term::parse(Span::new("12345")).unwrap().1,
        Marked {
            data: Term::Integer(12345),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        Term::parse(Span::new("12345.1")).unwrap().1,
        Marked {
            data: Term::Float(12345.1),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        Term::parse(Span::new("-12345.3")).unwrap().1,
        Marked {
            data: Term::Float(-12345.3),
            line: 1,
            column: 1
        }
    );
}

#[test]
fn test_bool() {
    assert_eq!(
        Term::parse(Span::new("true")).unwrap().1,
        Marked {
            data: Term::Boolean(true),
            line: 1,
            column: 1
        }
    );
    assert_eq!(
        Term::parse(Span::new("false")).unwrap().1,
        Marked {
            data: Term::Boolean(false),
            line: 1,
            column: 1
        }
    );
}

#[test]
fn test_array() {
    assert_eq!(
        Term::parse(Span::new("[]")).unwrap().1,
        Marked {
            data: Term::Array(vec![]),
            line: 1,
            column: 1
        }
    );

    assert_eq!(
        Term::parse(Span::new("[false]")).unwrap().1,
        Marked {
            data: Term::Array(vec![Marked {
                data: Expression::Term(Term::Boolean(false)),
                line: 1,
                column: 2
            }]),
            line: 1,
            column: 1
        }
    );
}

#[test]
fn test_map() {
    assert_eq!(
        Term::parse(Span::new("{}")).unwrap().1,
        Marked {
            data: Term::Map(vec![]),
            line: 1,
            column: 1
        }
    );

    assert_eq!(
        Term::parse(Span::new("{false => 1}")).unwrap().1,
        Marked {
            data: Term::Map(vec![(
                Marked {
                    data: Expression::Term(Term::Boolean(false)),
                    line: 1,
                    column: 2
                },
                Marked {
                    data: Expression::Term(Term::Integer(1)),
                    line: 1,
                    column: 11
                }
            )]),
            line: 1,
            column: 1
        }
    );

    assert!(Term::parse(Span::new("{'asdasd' => {}, 'a' => 'b', }")).is_ok());
}

#[test]
fn test_function_call() {
    assert_eq!(
        Term::parse(Span::new("lookup('ask8s::docker::gpu_nvidia')"))
            .unwrap()
            .1,
        Marked {
            line: 1,
            column: 1,
            data: Term::FunctionCall(FunctionCall {
                is_toplevel: false,
                name: vec!["lookup".to_owned()],
                args: vec![Marked {
                    data: Expression::Term(Term::SingleQuoted(
                        "ask8s::docker::gpu_nvidia".to_owned()
                    )),
                    line: 1,
                    column: 8
                }]
            })
        }
    );
}

#[test]
fn test_variable() {
    assert_eq!(
        Variable::parser(Span::new("$a")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: Variable {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                accessor: Vec::new()
            }
        }
    );
    assert_eq!(
        Variable::parser(Span::new("$::a::b")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: Variable {
                name: vec!["a".to_owned(), "b".to_owned()],
                is_toplevel: true,
                accessor: Vec::new()
            }
        }
    );
    assert_eq!(
        Variable::parser(Span::new("$a[ 1 ]['z']")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: Variable {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                accessor: vec![
                    Marked {
                        line: 1,
                        column: 5,
                        data: Expression::Term(Term::Integer(1))
                    },
                    Marked {
                        line: 1,
                        column: 9,
                        data: Expression::Term(Term::SingleQuoted("z".to_owned()))
                    },
                ]
            }
        }
    );
}

#[test]
fn test_multiply() {
    assert_eq!(
        Expression::parse(Span::new("2*3")).unwrap().1,
        Marked {
            line: 1,
            column: 2,
            data: Expression::Multiply((
                Box::new(Marked {
                    line: 1,
                    column: 1,
                    data: Expression::Term(Term::Integer(2))
                }),
                Box::new(Marked {
                    line: 1,
                    column: 3,
                    data: Expression::Term(Term::Integer(3))
                })
            ))
        }
    );
}

#[test]
fn test_operators_precendence() {
    assert_eq!(
        Expression::parse(Span::new("1 +2 * 3* 4 - 10")).unwrap().1,
        Marked {
            line: 1,
            column: 13,
            data: Expression::Minus((
                Box::new(Marked {
                    line: 1,
                    column: 3,
                    data: Expression::Plus((
                        Box::new(Marked {
                            line: 1,
                            column: 1,
                            data: Expression::Term(Term::Integer(1))
                        }),
                        Box::new(Marked {
                            line: 1,
                            column: 6,
                            data: Expression::Multiply((
                                Box::new(Marked {
                                    line: 1,
                                    column: 4,
                                    data: Expression::Term(Term::Integer(2))
                                }),
                                Box::new(Marked {
                                    line: 1,
                                    column: 9,
                                    data: Expression::Multiply((
                                        Box::new(Marked {
                                            line: 1,
                                            column: 8,
                                            data: Expression::Term(Term::Integer(3))
                                        }),
                                        Box::new(Marked {
                                            line: 1,
                                            column: 11,
                                            data: Expression::Term(Term::Integer(4))
                                        })
                                    ))
                                })
                            ))
                        })
                    ))
                }),
                Box::new(Marked {
                    line: 1,
                    column: 15,
                    data: Expression::Term(Term::Integer(10))
                })
            ))
        }
    );
}
