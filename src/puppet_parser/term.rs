use crate::puppet_lang::expression::MapKV;
use crate::puppet_lang::string::StringExpr;
use crate::puppet_parser::common::{
    capture_comment, curly_brackets_delimimited, round_parens_delimimited, space0_delimimited,
    square_brackets_delimimited,
};
use crate::puppet_parser::expression::{parse_accessor, parse_expression};
use crate::puppet_parser::range::Range;
use crate::puppet_parser::{IResult, ParseError, Span};
use nom::character::complete::{anychar, multispace0};
use nom::combinator::{eof, map_res, peek, verify};
use nom::sequence::{terminated, tuple};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt, recognize},
    sequence::{pair, preceded},
};

pub fn parse_variable(input: Span) -> IResult<crate::puppet_lang::expression::Variable<Range>> {
    map(
        pair(
            tag("$"),
            crate::puppet_parser::identifier::identifier_with_toplevel,
        ),
        |(dollar_sign, identifier)| crate::puppet_lang::expression::Variable {
            extra: (dollar_sign, &identifier.extra).into(),
            is_local_scope: identifier.name.last().unwrap().starts_with('_'),
            identifier,
        },
    )(input)
}

pub fn parse_regexp_group_id(
    input: Span,
) -> IResult<crate::puppet_lang::expression::RegexpGroupID<Range>> {
    map(
        pair(
            tag("$"),
            map_res(digit1, |s: Span| s.parse::<u64>().map(|r| (r, s))),
        ),
        |(dollar_sign, (identifier, identifier_span))| {
            crate::puppet_lang::expression::RegexpGroupID {
                extra: Range::from((dollar_sign, identifier_span)),
                identifier,
            }
        },
    )(input)
}

pub fn parse_float(input: Span) -> IResult<(f32, Span)> {
    let number = pair(digit1, pair(alt((tag("e"), tag("E"), tag("."))), digit1));
    let (tail, s) = recognize(pair(opt(tag("-")), number))(input)?;

    let f = match s.parse::<f32>() {
        Ok(v) => v,
        Err(err) => return ParseError::fatal(format!("{}", err), input),
    };

    Ok((tail, (f, s)))
}

pub fn parse_float_term(input: Span) -> IResult<crate::puppet_lang::expression::Float<Range>> {
    map(parse_float, |(value, span)| {
        crate::puppet_lang::expression::Float {
            value,
            extra: Range::from((span, span)),
        }
    })(input)
}

pub fn parse_integer(input: Span) -> IResult<(i64, Span)> {
    let (tail, s) = recognize(pair(
        opt(tag("-")),
        terminated(
            digit1,
            alt((
                map(
                    peek(verify(anychar, |c| *c != 'e' && *c != 'E' && *c != '.')),
                    |_| (),
                ),
                map(eof, |_| ()),
            )),
        ),
    ))(input)?;

    let v = match s.parse::<i64>() {
        Ok(v) => v,
        Err(err) => return ParseError::fatal(format!("{}", err), input),
    };

    Ok((tail, (v, s)))
}

pub fn parse_integer_term(input: Span) -> IResult<crate::puppet_lang::expression::Integer<Range>> {
    map(parse_integer, |(value, span)| {
        crate::puppet_lang::expression::Integer {
            value,
            extra: Range::from((span, span)),
        }
    })(input)
}

pub fn parse_usize(input: Span) -> IResult<(usize, Span)> {
    let (tail, s) = digit1(input)?;

    let v = match s.parse::<usize>() {
        Ok(v) => v,
        Err(err) => return ParseError::fatal(format!("{}", err), input),
    };

    Ok((tail, (v, s)))
}

pub fn parse_usize_term(input: Span) -> IResult<crate::puppet_lang::expression::Usize<Range>> {
    map(parse_usize, |(value, span)| {
        crate::puppet_lang::expression::Usize {
            value,
            extra: Range::from((span, span)),
        }
    })(input)
}

pub fn parse_sensitive(input: Span) -> IResult<crate::puppet_lang::expression::TermVariant<Range>> {
    map(
        pair(
            tag("Sensitive"),
            preceded(
                multispace0,
                round_parens_delimimited(ParseError::protect(
                    |_| "Expected term".to_string(),
                    parse_term,
                )),
            ),
        ),
        |(tag, arg)| {
            crate::puppet_lang::expression::TermVariant::Sensitive(
                crate::puppet_lang::expression::Sensitive {
                    value: Box::new(arg.1),
                    extra: Range::from((tag, arg.2)),
                },
            )
        },
    )(input)
}

fn parse_map(input: Span) -> IResult<crate::puppet_lang::expression::TermVariant<Range>> {
    let key_parser = alt((
        map(pair(parse_term, parse_accessor), |(value, accessor)| {
            crate::puppet_lang::expression::Expression {
                extra: value.extra.clone(),
                value: crate::puppet_lang::expression::ExpressionVariant::Term(value),
                accessor,
                comment: vec![],
            }
        }),
        parse_expression,
    ));

    let kv_parser = map(
        tuple((
            capture_comment,
            space0_delimimited(key_parser),
            preceded(
                tag("=>"),
                space0_delimimited(ParseError::protect(
                    |_| "Expression expected after '=>'".to_string(),
                    crate::puppet_parser::expression::parse_expression,
                )),
            ),
        )),
        |(comment, key, value)| MapKV {
            key,
            value,
            comment,
        },
    );

    map(
        curly_brackets_delimimited(
            false,
            crate::puppet_parser::common::comma_separated_list0_with_last_comment(kv_parser),
        ),
        |(tag_left, value, tag_right)| {
            crate::puppet_lang::expression::TermVariant::Map(crate::puppet_lang::expression::Map {
                extra: (&tag_left, &tag_right).into(),
                value,
            })
        },
    )(input)
}

fn parse_array(input: Span) -> IResult<crate::puppet_lang::expression::TermVariant<Range>> {
    map(
        square_brackets_delimimited(
            false,
            crate::puppet_parser::common::comma_separated_list0_with_last_comment(
                crate::puppet_parser::expression::parse_expression,
            ),
        ),
        |(tag_left, value, tag_right)| {
            crate::puppet_lang::expression::TermVariant::Array(
                crate::puppet_lang::expression::Array {
                    extra: (&tag_left, &tag_right).into(),
                    value,
                },
            )
        },
    )(input)
}

pub fn parse_string_variant(input: Span) -> IResult<StringExpr<Range>> {
    alt((
        crate::puppet_parser::double_quoted::parse,
        crate::puppet_parser::single_quoted::parse,
    ))(input)
}

pub fn parse_parens(input: Span) -> IResult<crate::puppet_lang::expression::Parens<Range>> {
    map(
        tuple((
            space0_delimimited(tag("(")),
            crate::puppet_parser::expression::parse_expression,
            space0_delimimited(tag(")")),
        )),
        |(left_tag, value, right_tag)| crate::puppet_lang::expression::Parens {
            extra: (&left_tag, &right_tag).into(),
            value: Box::new(value),
        },
    )(input)
}

pub fn parse_resource_identifier(
    input: Span,
) -> IResult<crate::puppet_lang::expression::TermVariant<Range>> {
    let multi_parser = map(
        tuple((
            opt(tag("::")),
            crate::puppet_parser::identifier::lowercase_identifier,
            tag("::"),
            crate::puppet_parser::identifier::lower_identifier_with_ns,
        )),
        |(toplevel_tag, head, _, mut name)| {
            let first = toplevel_tag.as_ref().unwrap_or(&head);
            name.insert(0, head);
            crate::puppet_lang::expression::TermVariant::Identifier(
                crate::puppet_lang::identifier::LowerIdentifier {
                    extra: Range::from((first, name.last().unwrap())),
                    name: name.iter().map(|v| v.to_string()).collect(),
                    is_toplevel: toplevel_tag.is_some(),
                },
            )
        },
    );

    let single_with_toplevel_parser = map(
        tuple((
            tag("::"),
            crate::puppet_parser::identifier::lower_identifier_with_ns,
        )),
        |(toplevel_tag, name)| {
            crate::puppet_lang::expression::TermVariant::Identifier(
                crate::puppet_lang::identifier::LowerIdentifier {
                    extra: Range::from((&toplevel_tag, name.last().unwrap())),
                    name: name.iter().map(|v| v.to_string()).collect(),
                    is_toplevel: true,
                },
            )
        },
    );

    alt((multi_parser, single_with_toplevel_parser))(input)
}

pub fn parse_term(input: Span) -> IResult<crate::puppet_lang::expression::Term<Range>> {
    let parse_true = map(tag("true"), |kw| {
        crate::puppet_lang::expression::TermVariant::Boolean(
            crate::puppet_lang::expression::Boolean {
                value: true,
                extra: (kw, kw).into(),
            },
        )
    });

    let parse_false = map(tag("false"), |kw| {
        crate::puppet_lang::expression::TermVariant::Boolean(
            crate::puppet_lang::expression::Boolean {
                value: false,
                extra: (kw, kw).into(),
            },
        )
    });

    let parse_type_specification = map(
        crate::puppet_parser::typing::parse_type_specification,
        crate::puppet_lang::expression::TermVariant::TypeSpecitifaction,
    );

    let parser = alt((
        parse_true,
        parse_false,
        parse_sensitive,
        map(
            parse_float_term,
            crate::puppet_lang::expression::TermVariant::Float,
        ),
        map(
            parse_integer_term,
            crate::puppet_lang::expression::TermVariant::Integer,
        ),
        parse_type_specification,
        parse_resource_identifier,
        map(
            parse_string_variant,
            crate::puppet_lang::expression::TermVariant::String,
        ),
        parse_array,
        map(
            parse_parens,
            crate::puppet_lang::expression::TermVariant::Parens,
        ),
        parse_map,
        map(
            parse_variable,
            crate::puppet_lang::expression::TermVariant::Variable,
        ),
        map(
            parse_regexp_group_id,
            crate::puppet_lang::expression::TermVariant::RegexpGroupID,
        ),
        map(
            crate::puppet_parser::regex::parse,
            crate::puppet_lang::expression::TermVariant::Regexp,
        ),
    ));

    map(parser, |value| crate::puppet_lang::expression::Term {
        extra: Range::from(&value),
        value,
    })(input)
}

#[test]
fn test_array_of_types() {
    assert_eq!(
        parse_term(Span::new("[ Class['some_class'] ]")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Array(crate::puppet_lang::expression::Array {
                value: crate::puppet_lang::List {
                    last_comment: vec![],
                    value: vec![
                        crate::puppet_lang::expression::Expression {
                            accessor: None,
                            comment: vec![],
                            value: crate::puppet_lang::expression::ExpressionVariant::Term(
                                crate::puppet_lang::expression::Term {
                                    value: crate::puppet_lang::expression::TermVariant::TypeSpecitifaction(
                                        crate::puppet_lang::typing::TypeSpecification {
                                            comment: vec![],
                                            data:
                                            crate::puppet_lang::typing::TypeSpecificationVariant::ExternalType(
                                                crate::puppet_lang::typing::ExternalType {
                                                    name: vec!["Class".to_owned()],
                                                    arguments: vec![
                                                        crate::puppet_lang::expression::Expression {
                                                            accessor: None,
                                                            comment: vec![],
                                                            value: crate::puppet_lang::expression::ExpressionVariant::Term(
                                                                crate::puppet_lang::expression::Term {
                                                                    value: crate::puppet_lang::expression::TermVariant::String(crate::puppet_lang::string::StringExpr {
                                                                        data: crate::puppet_lang::string::StringVariant::SingleQuoted(vec![
                                                                            crate::puppet_lang::string::StringFragment::Literal(crate::puppet_lang::string::Literal {
                                                                                data: "some_class".to_owned(),
                                                                                extra: Range::new(9, 1, 10, 18, 1, 19)
                                                                            })
                                                                        ]),
                                                                        extra: Range::new(8, 1, 9, 19, 1, 20)
                                                                    }),
                                                                    extra: Range::new(8, 1, 9, 19, 1, 20)
                                                                }),
                                                            extra: Range::new(8, 1, 9, 19, 1, 20)
                                                        }
                                                    ],
                                                    extra: Range::new(2, 1, 3, 20, 1, 21)
                                                }
                                            ),
                                            extra: Range::new(2, 1, 3, 20, 1, 21)
                                        }
                                    ),
                                    extra: Range::new(2, 1, 3, 20, 1, 21)
                                }
                            ),
                            extra: Range::new(2, 1, 3, 20, 1, 21)
                        }
                    ]},
                extra: Range::new(0, 1, 1, 22, 1, 23)
            }),
            extra: Range::new(0, 1, 1, 22, 1, 23)
        }
    )
}

#[test]
fn test_numbers() {
    assert_eq!(
        parse_term(Span::new("12345")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Integer(
                crate::puppet_lang::expression::Integer {
                    extra: Range::new(0, 1, 1, 4, 1, 5),
                    value: 12345
                }
            ),
            extra: Range::new(0, 1, 1, 4, 1, 5)
        }
    );
    assert_eq!(
        parse_term(Span::new("12345.1")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Float(
                crate::puppet_lang::expression::Float {
                    value: 12345.1,
                    extra: Range::new(0, 1, 1, 6, 1, 7)
                }
            ),
            extra: Range::new(0, 1, 1, 6, 1, 7)
        }
    );
    assert_eq!(
        parse_term(Span::new("-12345.3")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Float(
                crate::puppet_lang::expression::Float {
                    value: -12345.3,
                    extra: Range::new(0, 1, 1, 7, 1, 8)
                }
            ),
            extra: Range::new(0, 1, 1, 7, 1, 8)
        }
    );
}

#[test]
fn test_bool() {
    assert_eq!(
        parse_term(Span::new("true")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Boolean(
                crate::puppet_lang::expression::Boolean {
                    value: true,
                    extra: Range::new(0, 1, 1, 3, 1, 4)
                }
            ),
            extra: Range::new(0, 1, 1, 3, 1, 4)
        }
    );
    assert_eq!(
        parse_term(Span::new("false")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Boolean(
                crate::puppet_lang::expression::Boolean {
                    value: false,
                    extra: Range::new(0, 1, 1, 4, 1, 5)
                }
            ),
            extra: Range::new(0, 1, 1, 4, 1, 5)
        }
    );
}

#[test]
fn test_array() {
    assert_eq!(
        parse_term(Span::new("[]")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Array(
                crate::puppet_lang::expression::Array {
                    value: crate::puppet_lang::List::default(),
                    extra: Range::new(0, 1, 1, 1, 1, 2)
                }
            ),
            extra: Range::new(0, 1, 1, 1, 1, 2)
        }
    );

    assert_eq!(
        parse_term(Span::new("[false]")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Array(
                crate::puppet_lang::expression::Array {
                    value: crate::puppet_lang::List {
                        last_comment: vec![],
                        value: vec![crate::puppet_lang::expression::Expression {
                            accessor: None,
                            comment: vec![],
                            value: crate::puppet_lang::expression::ExpressionVariant::Term(
                                crate::puppet_lang::expression::Term {
                                    value: crate::puppet_lang::expression::TermVariant::Boolean(
                                        crate::puppet_lang::expression::Boolean {
                                            value: false,
                                            extra: Range::new(1, 1, 2, 5, 1, 6)
                                        }
                                    ),
                                    extra: Range::new(1, 1, 2, 5, 1, 6)
                                }
                            ),
                            extra: Range::new(1, 1, 2, 5, 1, 6)
                        }]
                    },
                    extra: Range::new(0, 1, 1, 6, 1, 7)
                }
            ),
            extra: Range::new(0, 1, 1, 6, 1, 7)
        }
    );
}

#[test]
fn test_map() {
    assert_eq!(
        parse_term(Span::new("{}")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Map(
                crate::puppet_lang::expression::Map {
                    value: crate::puppet_lang::List::default(),
                    extra: Range::new(0, 1, 1, 1, 1, 2)
                }
            ),
            extra: Range::new(0, 1, 1, 1, 1, 2)
        }
    );

    assert_eq!(
        parse_term(Span::new("{false => 1}")).unwrap().1,
        crate::puppet_lang::expression::Term {
            value: crate::puppet_lang::expression::TermVariant::Map(
                crate::puppet_lang::expression::Map {
                    value: crate::puppet_lang::List {
                        last_comment: vec![],
                        value: vec![crate::puppet_lang::expression::MapKV {
                            key: crate::puppet_lang::expression::Expression {
                                extra: Range::new(1, 1, 2, 5, 1, 6),
                                accessor: None,
                                comment: vec![],
                                value: crate::puppet_lang::expression::ExpressionVariant::Term(
                                    crate::puppet_lang::expression::Term {
                                        value: crate::puppet_lang::expression::TermVariant::Boolean(
                                            crate::puppet_lang::expression::Boolean {
                                                value: false,
                                                extra: Range::new(1, 1, 2, 5, 1, 6)
                                            }
                                        ),
                                        extra: Range::new(1, 1, 2, 5, 1, 6)
                                    }
                                )
                            },
                            value: crate::puppet_lang::expression::Expression {
                                accessor: None,
                                comment: vec![],
                                value: crate::puppet_lang::expression::ExpressionVariant::Term(
                                    crate::puppet_lang::expression::Term {
                                        value: crate::puppet_lang::expression::TermVariant::Integer(
                                            crate::puppet_lang::expression::Integer {
                                                value: 1,
                                                extra: Range::new(10, 1, 11, 10, 1, 11)
                                            }
                                        ),
                                        extra: Range::new(10, 1, 11, 10, 1, 11)
                                    }
                                ),
                                extra: Range::new(10, 1, 11, 10, 1, 11)
                            },
                            comment: vec![],
                        }],
                    },
                    extra: Range::new(0, 1, 1, 11, 1, 12)
                }
            ),
            extra: Range::new(0, 1, 1, 11, 1, 12)
        }
    );

    assert!(parse_term(Span::new("{'asdasd' => {}, 'a' => 'b', }")).is_ok());
}

#[test]
fn test_variable() {
    assert_eq!(
        parse_variable(Span::new("$a")).unwrap().1,
        crate::puppet_lang::expression::Variable {
            identifier: crate::puppet_lang::identifier::LowerIdentifier {
                name: vec!["a".to_owned()],
                is_toplevel: false,
                extra: Range::new(1, 1, 2, 1, 1, 2)
            },
            is_local_scope: false,
            extra: Range::new(0, 1, 1, 1, 1, 2)
        }
    );
    assert_eq!(
        parse_variable(Span::new("$_a")).unwrap().1,
        crate::puppet_lang::expression::Variable {
            identifier: crate::puppet_lang::identifier::LowerIdentifier {
                name: vec!["_a".to_owned()],
                is_toplevel: false,
                extra: Range::new(1, 1, 2, 2, 1, 3)
            },
            is_local_scope: true,
            extra: Range::new(0, 1, 1, 2, 1, 3)
        }
    );
    assert_eq!(
        parse_variable(Span::new("$::a::b")).unwrap().1,
        crate::puppet_lang::expression::Variable {
            identifier: crate::puppet_lang::identifier::LowerIdentifier {
                name: vec!["a".to_owned(), "b".to_owned()],
                is_toplevel: true,
                extra: Range::new(1, 1, 2, 6, 1, 7)
            },
            is_local_scope: false,
            extra: Range::new(0, 1, 1, 6, 1, 7)
        }
    );
}
