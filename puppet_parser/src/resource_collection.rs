use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, pair, tuple},
};

use crate::{
    common::{fold_many0_with_const_init, space0_delimimited},
    parser::{IResult, Location, ParseError, Span},
};

pub fn parse_search_condition(
    input: Span,
) -> IResult<puppet_lang::resource_collection::SearchExpression<Location>> {
    let parser = tuple((
        space0_delimimited(crate::identifier::lowercase_identifier),
        alt((tag("=="), tag("!="))),
        space0_delimimited(crate::term::parse_term),
    ));

    map(parser, |(name, op, expr)| match *op {
        "==" => puppet_lang::resource_collection::SearchExpression {
            value: puppet_lang::resource_collection::ExpressionVariant::Equal((
                puppet_lang::resource_collection::Attribute {
                    extra: Location::from(name),
                    name: name.to_string(),
                },
                expr,
            )),
            extra: Location::from(op),
        },
        "!=" => puppet_lang::resource_collection::SearchExpression {
            value: puppet_lang::resource_collection::ExpressionVariant::NotEqual((
                puppet_lang::resource_collection::Attribute {
                    extra: Location::from(name),
                    name: name.to_string(),
                },
                expr,
            )),
            extra: Location::from(op),
        },
        _ => unreachable!(),
    })(input)
}

fn parse_parens(
    input: Span,
) -> IResult<puppet_lang::resource_collection::SearchExpression<Location>> {
    let parser = map(
        tuple((
            space0_delimimited(tag("(")),
            parse_search_expression,
            space0_delimimited(tag(")")),
        )),
        |(left_paren, expr, _right_paren)| puppet_lang::resource_collection::SearchExpression {
            extra: Location::from(left_paren),
            value: puppet_lang::resource_collection::ExpressionVariant::Parens(Box::new(expr)),
        },
    );

    alt((parser, parse_search_condition))(input)
}

fn parse_search_expression(
    input: Span,
) -> IResult<puppet_lang::resource_collection::SearchExpression<Location>> {
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
                value: puppet_lang::resource_collection::ExpressionVariant::And((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            "or" => puppet_lang::resource_collection::SearchExpression {
                value: puppet_lang::resource_collection::ExpressionVariant::Or((
                    Box::new(prev),
                    Box::new(cur),
                )),
                extra: Location::from(op),
            },
            _ => unreachable!(),
        },
    );
    parser(input)
}

pub fn parse_resource_collection(
    input: Span,
) -> IResult<puppet_lang::resource_collection::ResourceCollection<Location>> {
    let parser = pair(
        crate::typing::parse_type_specification,
        opt(delimited(
            space0_delimimited(tag("<|")),
            opt(parse_search_expression),
            space0_delimimited(tag("|>")),
        )),
    );

    map(parser, |(type_specification, search_expression)| {
        puppet_lang::resource_collection::ResourceCollection {
            extra: type_specification.extra.clone(),
            type_specification,
            search_expression: search_expression.flatten(),
        }
    })(input)
}
