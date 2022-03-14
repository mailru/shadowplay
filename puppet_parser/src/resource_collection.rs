use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, tuple},
};

use crate::{
    common::{capture_comment, fold_many0_with_const_init, space0_delimimited},
    {range::Range, IResult, ParseError, Span},
};

pub fn parse_search_condition(
    input: Span,
) -> IResult<puppet_lang::resource_collection::SearchExpression<Range>> {
    let parser = tuple((
        space0_delimimited(crate::identifier::lowercase_identifier),
        alt((tag("=="), tag("!="))),
        space0_delimimited(crate::term::parse_term),
    ));

    map(parser, |(name, op, expr)| match *op {
        "==" => puppet_lang::resource_collection::SearchExpression {
            extra: Range::from((name, &expr.extra)),
            value: puppet_lang::resource_collection::ExpressionVariant::Equal((
                puppet_lang::resource_collection::Attribute {
                    extra: Range::from((name, name)),
                    name: name.to_string(),
                },
                expr,
            )),
        },
        "!=" => puppet_lang::resource_collection::SearchExpression {
            extra: Range::from((name, &expr.extra)),
            value: puppet_lang::resource_collection::ExpressionVariant::NotEqual((
                puppet_lang::resource_collection::Attribute {
                    extra: Range::from((name, name)),
                    name: name.to_string(),
                },
                expr,
            )),
        },
        _ => unreachable!(),
    })(input)
}

fn parse_parens(input: Span) -> IResult<puppet_lang::resource_collection::SearchExpression<Range>> {
    let parser = map(
        tuple((
            space0_delimimited(tag("(")),
            parse_search_expression,
            space0_delimimited(tag(")")),
        )),
        |(left_paren, expr, right_paren)| puppet_lang::resource_collection::SearchExpression {
            extra: Range::from((left_paren, right_paren)),
            value: puppet_lang::resource_collection::ExpressionVariant::Parens(Box::new(expr)),
        },
    );

    alt((parser, parse_search_condition))(input)
}

fn parse_search_expression(
    input: Span,
) -> IResult<puppet_lang::resource_collection::SearchExpression<Range>> {
    let (input, left_expr) = space0_delimimited(parse_parens)(input)?;
    let mut parser = fold_many0_with_const_init(
        pair(
            alt((tag("and"), tag("or"))),
            space0_delimimited(ParseError::protect(
                |_| "Second argument of operator is expected".to_string(),
                parse_parens,
            )),
        ),
        left_expr,
        |prev, (op, cur)| match *op {
            "and" => puppet_lang::resource_collection::SearchExpression {
                extra: Range::from((&prev.extra, &cur.extra)),
                value: puppet_lang::resource_collection::ExpressionVariant::And((
                    Box::new(prev),
                    Box::new(cur),
                )),
            },
            "or" => puppet_lang::resource_collection::SearchExpression {
                extra: Range::from((&prev.extra, &cur.extra)),
                value: puppet_lang::resource_collection::ExpressionVariant::Or((
                    Box::new(prev),
                    Box::new(cur),
                )),
            },
            _ => unreachable!(),
        },
    );
    parser(input)
}

pub fn parse_resource_collection(
    input: Span,
) -> IResult<puppet_lang::resource_collection::ResourceCollection<Range>> {
    let parser = tuple((
        capture_comment,
        crate::typing::parse_type_specification,
        opt(tuple((
            space0_delimimited(tag("<|")),
            opt(parse_search_expression),
            space0_delimimited(tag("|>")),
        ))),
    ));

    map(
        parser,
        |(comment, type_specification, search_expression)| {
            let (search_expression, end_range) = match search_expression {
                Some((_left_tag, search_expression, right_tag)) => {
                    (Some(search_expression), Range::from((right_tag, right_tag)))
                }
                None => (None, type_specification.extra.clone()),
            };

            puppet_lang::resource_collection::ResourceCollection {
                extra: Range::from((&type_specification.extra, &end_range)),
                type_specification,
                search_expression: search_expression.flatten(),
                comment,
            }
        },
    )(input)
}
