use crate::puppet_lang::builtin;
use crate::puppet_lang::builtin::BuiltinVariant;
use crate::puppet_lang::expression::{Expression, ExpressionVariant};
use nom::combinator::{eof, opt, success};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::sequence::{pair, tuple};

use crate::puppet_parser::common::{
    capture_comment, comma_separator, round_parens_delimimited, space0_delimimited, spaced_word,
};

use crate::puppet_parser::range::Range;
use crate::puppet_parser::{IResult, ParseError, Span};

use nom::{branch::alt, combinator::map};

fn builtin_variant_parser<'a, O, PARSER, MAPPER, PARSERESULT>(
    keyword: &'static str,
    args_parser: PARSER,
    mapper: MAPPER,
) -> impl Fn(Span<'a>) -> IResult<O>
where
    PARSER: Fn(Span<'a>) -> IResult<(PARSERESULT, Option<Range>)>,
    MAPPER: Fn(
        (
            Vec<crate::puppet_lang::comment::Comment<Range>>,
            Span<'a>,
            (
                (PARSERESULT, crate::puppet_parser::range::Range),
                Option<crate::puppet_lang::expression::Lambda<Range>>,
            ),
            Option<crate::puppet_lang::expression::Accessor<Range>>,
        ),
    ) -> O,
    O: Clone,
    PARSERESULT: Clone,
{
    move |i| {
        let parse_with_parens = pair(
            map(
                round_parens_delimimited(terminated(
                    &args_parser,
                    // Optional trailing comma
                    opt(comma_separator),
                )),
                |(_left_paren, (body, _end_range), right_paren)| {
                    (body, Some(Range::from((right_paren, right_paren))))
                },
            ),
            opt(space0_delimimited(
                crate::puppet_parser::expression::parse_lambda,
            )),
        );

        let parse_no_parens = map(&args_parser, |(list, end_range)| ((list, end_range), None));

        let body_parser = ParseError::protect(
            |_| "Arguments list or () expected".to_string(),
            alt((parse_with_parens, parse_no_parens)),
        );

        map(
            map(
                tuple((
                    capture_comment,
                    spaced_word(keyword),
                    body_parser,
                    crate::puppet_parser::expression::parse_accessor,
                )),
                |(comment, kw, ((args, end_range), lambda), accessor)| {
                    let range = match &lambda {
                        None => match end_range {
                            None => Range::from((kw, kw)),
                            Some(end_range) => Range::from((kw, &end_range)),
                        },
                        Some(v) => Range::from((&kw, &accessor, &v.extra)),
                    };
                    (comment, kw, ((args, range), lambda), accessor)
                },
            ),
            &mapper,
        )(i)
    }
}

fn builtin_unit<'a, O, MAPPER>(
    keyword: &'static str,
    mapper: MAPPER,
) -> impl Fn(Span<'a>) -> IResult<O>
where
    MAPPER: Fn(
        (
            Vec<crate::puppet_lang::comment::Comment<Range>>,
            Span<'a>,
            (
                ((), crate::puppet_parser::range::Range),
                Option<crate::puppet_lang::expression::Lambda<Range>>,
            ),
            Option<crate::puppet_lang::expression::Accessor<Range>>,
        ),
    ) -> O,
    O: Clone,
{
    builtin_variant_parser(
        keyword,
        |i| {
            map(alt((success(()), nom::combinator::value((), eof))), |()| {
                ((), None)
            })(i)
        },
        mapper,
    )
}

// fn builtin_one<'a, O, MAPPER>(
//     keyword: &'static str,
//     mapper: MAPPER,
// ) -> impl Fn(Span<'a>) -> IResult<O>
// where
//     MAPPER: Fn(
//         (
//             Vec<crate::puppet_lang::comment::Comment<Range>>,
//             Span<'a>,
//             (
//                 (Expression<Range>, crate::puppet_parser::range::Range),
//                 Option<crate::puppet_lang::expression::Lambda<Range>>,
//             ),
//             Option<crate::puppet_lang::expression::Accessor<Range>>,
//         ),
//     ) -> O,

//     MAPPER: Fn(
//         (
//             Span<'a>,
//             (
//                 (Expression<Range>, crate::puppet_parser::range::Range),
//                 Option<crate::puppet_lang::expression::Lambda<Range>>,
//             ),
//         ),
//     ) -> O,
//     O: Clone,
// {
//     builtin_variant_parser(
//         keyword,
//         |i| {
//             map(crate::puppet_parser::expression::parse_expression, |expr| {
//                 let range = expr.extra.clone();
//                 (expr, Some(range))
//             })(i)
//         },
//         mapper,
//     )
// }

fn builtin_many1<'a, O, MAPPER>(
    keyword: &'static str,
    mapper: MAPPER,
) -> impl Fn(Span<'a>) -> IResult<O>
where
    MAPPER: Fn(
        (
            Vec<crate::puppet_lang::comment::Comment<Range>>,
            Span<'a>,
            (
                (Vec<Expression<Range>>, crate::puppet_parser::range::Range),
                Option<crate::puppet_lang::expression::Lambda<Range>>,
            ),
            Option<crate::puppet_lang::expression::Accessor<Range>>,
        ),
    ) -> O,
    O: Clone,
{
    builtin_variant_parser(
        keyword,
        |i| {
            map(
                separated_list1(
                    comma_separator,
                    crate::puppet_parser::expression::parse_expression,
                ),
                |list| {
                    let range = list.last().unwrap().extra.clone();
                    (list, Some(range))
                },
            )(i)
        },
        mapper,
    )
}

fn parse_undef(input: Span) -> IResult<Expression<Range>> {
    builtin_unit(
        "undef",
        |(comment, _kw, ((_, range), _lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Undef),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

fn parse_return(input: Span) -> IResult<Expression<Range>> {
    builtin_variant_parser(
        "return",
        |i| {
            map(
                opt(crate::puppet_parser::expression::parse_expression),
                |expr| {
                    let range = expr.as_ref().map(|v| v.extra.clone());
                    (expr, range)
                },
            )(i)
        },
        |(comment, _kw, ((arg, range), _lambda), _accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Return(Box::new(arg))),
            extra: range,
            comment,
            accessor: None,
        },
    )(input)
}

fn parse_template(input: Span) -> IResult<Expression<Range>> {
    builtin_many1(
        "template",
        |(comment, _kw, ((args, range), lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Template(builtin::Many1 {
                lambda,
                args,
            })),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

fn parse_tag(input: Span) -> IResult<Expression<Range>> {
    builtin_many1(
        "tag",
        |(comment, _kw, ((args, range), lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Tag(builtin::Many1 {
                lambda,
                args,
            })),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

fn parse_require(input: Span) -> IResult<Expression<Range>> {
    builtin_many1(
        "require",
        |(comment, _kw, ((args, range), lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Require(builtin::Many1 {
                lambda,
                args,
            })),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

fn parse_include(input: Span) -> IResult<Expression<Range>> {
    builtin_many1(
        "include",
        |(comment, _kw, ((args, range), lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Include(builtin::Many1 {
                lambda,
                args,
            })),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

fn parse_realize(input: Span) -> IResult<Expression<Range>> {
    builtin_many1(
        "realize",
        |(comment, _kw, ((args, range), lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Realize(builtin::Many1 {
                lambda,
                args,
            })),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

fn parse_create_resources(input: Span) -> IResult<Expression<Range>> {
    builtin_many1(
        "create_resources",
        |(comment, _kw, ((args, range), lambda), accessor)| Expression {
            value: ExpressionVariant::BuiltinFunction(BuiltinVariant::CreateResources(
                builtin::Many1 { lambda, args },
            )),
            extra: range,
            comment,
            accessor,
        },
    )(input)
}

pub fn parse_builtin(input: Span) -> IResult<Expression<Range>> {
    alt((
        parse_undef,
        parse_return,
        parse_template,
        parse_tag,
        parse_require,
        parse_include,
        parse_realize,
        parse_create_resources,
    ))(input)
}

#[test]
fn test_undef() {
    assert_eq!(
        parse_builtin(Span::new("undef")).unwrap().1,
        crate::puppet_lang::expression::Expression {
            accessor: None,
            comment: vec![],
            value: crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                crate::puppet_lang::builtin::BuiltinVariant::Undef
            ),
            extra: Range::new(0, 1, 1, 4, 1, 5)
        }
    );

    assert_eq!(
        parse_builtin(Span::new("undef()")).unwrap().1,
        crate::puppet_lang::expression::Expression {
            accessor: None,
            comment: vec![],
            value: crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                crate::puppet_lang::builtin::BuiltinVariant::Undef
            ),
            extra: Range::new(0, 1, 1, 6, 1, 7)
        }
    );
}

#[test]
fn test_return() {
    assert_eq!(
        parse_builtin(Span::new("return")).unwrap().1,
        crate::puppet_lang::expression::Expression {
            accessor: None,
            comment: vec![],
            value: crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                crate::puppet_lang::builtin::BuiltinVariant::Return(Box::new(None))
            ),
            extra: Range::new(0, 1, 1, 5, 1, 6)
        }
    );

    assert_eq!(
        parse_builtin(Span::new("return(100)")).unwrap().1,
        crate::puppet_lang::expression::Expression {
            accessor: None,
            comment: vec![],
            value: crate::puppet_lang::expression::ExpressionVariant::BuiltinFunction(
                crate::puppet_lang::builtin::BuiltinVariant::Return(Box::new(Some(
                    crate::puppet_lang::expression::Expression {
                        value: crate::puppet_lang::expression::ExpressionVariant::Term(
                            crate::puppet_lang::expression::Term {
                                value: crate::puppet_lang::expression::TermVariant::Integer(
                                    crate::puppet_lang::expression::Integer {
                                        value: 100,
                                        extra: Range::new(7, 1, 8, 9, 1, 10),
                                    }
                                ),
                                extra: Range::new(7, 1, 8, 9, 1, 10),
                            }
                        ),
                        extra: Range::new(7, 1, 8, 9, 1, 10),
                        accessor: None,
                        comment: vec![]
                    }
                )))
            ),
            extra: Range::new(0, 1, 1, 10, 1, 11)
        }
    );
}
