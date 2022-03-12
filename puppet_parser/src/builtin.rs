use nom::combinator::{opt, success};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::terminated;
use puppet_lang::builtin;
use puppet_lang::builtin::BuiltinVariant;
use puppet_lang::expression::{Expression, ExpressionVariant};

use crate::common::{comma_separator, round_brackets_delimimited, space0_delimimited, spaced_word};

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
            Span<'a>,
            (
                (PARSERESULT, crate::range::Range),
                Option<puppet_lang::expression::Lambda<Range>>,
            ),
        ),
    ) -> O,
    O: Clone,
    PARSERESULT: Clone,
{
    move |i| {
        let parse_with_parens = pair(
            map(
                round_brackets_delimimited(terminated(
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
                pair(spaced_word(keyword), body_parser),
                |(kw, ((args, end_range), lambda))| {
                    let range = match &lambda {
                        None => match end_range {
                            None => Range::from((kw, kw)),
                            Some(end_range) => Range::from((kw, &end_range)),
                        },
                        Some(v) => Range::from((kw, &v.extra)),
                    };
                    (kw, ((args, range), lambda))
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
            Span<'a>,
            (
                ((), crate::range::Range),
                Option<puppet_lang::expression::Lambda<Range>>,
            ),
        ),
    ) -> O,
    O: Clone,
{
    builtin_variant_parser(keyword, |i| map(success(()), |()| ((), None))(i), mapper)
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
            Span<'a>,
            (
                (Vec<Expression<Range>>, crate::range::Range),
                Option<puppet_lang::expression::Lambda<Range>>,
            ),
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
    builtin_unit("undef", |(_kw, ((_, range), _lambda))| Expression {
        value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Undef),
        extra: range,
    })(input)
}

fn parse_tag(input: Span) -> IResult<Expression<Range>> {
    builtin_many1("tag", |(_kw, ((args, range), lambda))| Expression {
        value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Tag(builtin::Many1 {
            lambda,
            args,
        })),
        extra: range,
    })(input)
}

fn parse_require(input: Span) -> IResult<Expression<Range>> {
    builtin_many1("require", |(_kw, ((args, range), lambda))| Expression {
        value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Require(builtin::Many1 {
            lambda,
            args,
        })),
        extra: range,
    })(input)
}

fn parse_include(input: Span) -> IResult<Expression<Range>> {
    builtin_many1("include", |(_kw, ((args, range), lambda))| Expression {
        value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Include(builtin::Many1 {
            lambda,
            args,
        })),
        extra: range,
    })(input)
}

fn parse_realize(input: Span) -> IResult<Expression<Range>> {
    builtin_many1("realize", |(_kw, ((args, range), lambda))| Expression {
        value: ExpressionVariant::BuiltinFunction(BuiltinVariant::Realize(builtin::Many1 {
            lambda,
            args,
        })),
        extra: range,
    })(input)
}

pub fn parse_builtin(input: Span) -> IResult<Expression<Range>> {
    alt((
        parse_undef,
        parse_tag,
        parse_require,
        parse_include,
        parse_realize,
    ))(input)
}
