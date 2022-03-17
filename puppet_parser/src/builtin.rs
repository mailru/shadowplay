use nom::combinator::{eof, opt, success};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::sequence::{pair, tuple};
use puppet_lang::builtin;
use puppet_lang::builtin::BuiltinVariant;
use puppet_lang::expression::{Expression, ExpressionVariant};

use crate::common::{
    capture_comment, comma_separator, round_parens_delimimited, space0_delimimited, spaced_word,
};

use crate::range::Range;
use crate::{IResult, ParseError, Span};

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
            Vec<puppet_lang::comment::Comment<Range>>,
            Span<'a>,
            (
                (PARSERESULT, crate::range::Range),
                Option<puppet_lang::expression::Lambda<Range>>,
            ),
            Option<puppet_lang::expression::Accessor<Range>>,
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
                    // В конце не обязательная запятая
                    opt(comma_separator),
                )),
                |(_left_paren, (body, _end_range), right_paren)| {
                    (body, Some(Range::from((right_paren, right_paren))))
                },
            ),
            opt(space0_delimimited(crate::expression::parse_lambda)),
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
                    crate::expression::parse_accessor,
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
            Vec<puppet_lang::comment::Comment<Range>>,
            Span<'a>,
            (
                ((), crate::range::Range),
                Option<puppet_lang::expression::Lambda<Range>>,
            ),
            Option<puppet_lang::expression::Accessor<Range>>,
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
//             Span<'a>,
//             (
//                 (Expression<Range>, crate::range::Range),
//                 Option<puppet_lang::expression::Lambda<Range>>,
//             ),
//         ),
//     ) -> O,
//     O: Clone,
// {
//     builtin_variant_parser(
//         keyword,
//         |i| {
//             map(crate::expression::parse_expression, |expr| {
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
            Vec<puppet_lang::comment::Comment<Range>>,
            Span<'a>,
            (
                (Vec<Expression<Range>>, crate::range::Range),
                Option<puppet_lang::expression::Lambda<Range>>,
            ),
            Option<puppet_lang::expression::Accessor<Range>>,
        ),
    ) -> O,
    O: Clone,
{
    builtin_variant_parser(
        keyword,
        |i| {
            map(
                separated_list1(comma_separator, crate::expression::parse_expression),
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
        parse_tag,
        parse_require,
        parse_include,
        parse_realize,
        parse_create_resources,
    ))(input)
}
