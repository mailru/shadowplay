use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::preceded,
};
use puppet_lang::statement::{Statement, StatementVariant};

use crate::{
    common::{comma_separator, fold_many0_with_const_init, round_brackets_delimimited, separator1},
    identifier::identifier_with_toplevel,
    parser::{IResult, Location, ParseError, Span},
    term::parse_string_variant,
};

pub fn parse_require(input: Span) -> IResult<StatementVariant<Location>> {
    let parser = preceded(
        tag("require"),
        preceded(
            separator1,
            ParseError::protect(
                |_| "Argument for 'require' is expected".to_string(),
                alt((
                    round_brackets_delimimited(identifier_with_toplevel),
                    identifier_with_toplevel,
                )),
            ),
        ),
    );

    map(parser, StatementVariant::Require)(input)
}

pub fn parse_include(input: Span) -> IResult<StatementVariant<Location>> {
    let parser = preceded(
        tag("include"),
        preceded(
            separator1,
            ParseError::protect(
                |_| "Argument for 'include' is expected".to_string(),
                alt((
                    round_brackets_delimimited(identifier_with_toplevel),
                    identifier_with_toplevel,
                )),
            ),
        ),
    );

    map(parser, StatementVariant::Include)(input)
}

pub fn parse_contain(input: Span) -> IResult<StatementVariant<Location>> {
    let parser = preceded(
        tag("contain"),
        preceded(
            separator1,
            ParseError::protect(
                |_| "Argument for 'contain' is expected".to_string(),
                alt((
                    round_brackets_delimimited(identifier_with_toplevel),
                    identifier_with_toplevel,
                )),
            ),
        ),
    );

    map(parser, StatementVariant::Contain)(input)
}

pub fn parse_tag(input: Span) -> IResult<StatementVariant<Location>> {
    let parser = preceded(
        tag("tag"),
        preceded(
            separator1,
            ParseError::protect(
                |_| "Arguments for 'tag' are expected".to_string(),
                alt((
                    round_brackets_delimimited(separated_list1(
                        comma_separator,
                        parse_string_variant,
                    )),
                    separated_list1(comma_separator, parse_string_variant),
                )),
            ),
        ),
    );

    map(parser, StatementVariant::Tag)(input)
}

pub fn parse_expression(input: Span) -> IResult<StatementVariant<Location>> {
    map(
        crate::expression::parse_expression,
        StatementVariant::Expression,
    )(input)
}

pub fn parse_resource_relation(input: Span) -> IResult<StatementVariant<Location>> {
    let (input, left_type) =
        crate::common::space0_delimimited(crate::typing::parse_type_specification)(input)?;
    let parser = fold_many0_with_const_init(
        preceded(
            tag("->"),
            crate::common::space0_delimimited(ParseError::protect(
                |_| "Second argument of relation is expected".to_string(),
                crate::typing::parse_type_specification,
            )),
        ),
        vec![left_type],
        |mut acc: Vec<_>, cur| {
            acc.push(cur);
            acc
        },
    );

    map(parser, StatementVariant::ResourceRelation)(input)
}

pub fn parse_statement_variant(input: Span) -> IResult<StatementVariant<Location>> {
    alt((
        parse_require,
        parse_include,
        parse_contain,
        parse_tag,
        parse_resource_relation,
        parse_expression,
    ))(input)
}

pub fn parse_statement(input: Span) -> IResult<Statement<Location>> {
    map(parse_statement_variant, |value| Statement {
        value,
        extra: Location::from(input),
    })(input)
}
