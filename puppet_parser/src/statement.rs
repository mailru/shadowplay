use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{many0, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};
use puppet_lang::statement::{Statement, StatementVariant};

use crate::{
    common::{
        comma_separator, fold_many0_with_const_init, round_brackets_delimimited, separator1,
        space0_delimimited,
    },
    identifier::identifier_with_toplevel,
    parser::{IResult, Location, ParseError, Span},
    term::parse_string_variant,
};

fn parse_require(input: Span) -> IResult<StatementVariant<Location>> {
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

fn parse_include(input: Span) -> IResult<StatementVariant<Location>> {
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

fn parse_contain(input: Span) -> IResult<StatementVariant<Location>> {
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

fn parse_tag(input: Span) -> IResult<StatementVariant<Location>> {
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

fn parse_expression(input: Span) -> IResult<StatementVariant<Location>> {
    map(
        crate::expression::parse_expression,
        StatementVariant::Expression,
    )(input)
}

fn parse_resource_type_relation(input: Span) -> IResult<StatementVariant<Location>> {
    let (input, left_type) = space0_delimimited(crate::typing::parse_type_specification)(input)?;
    let parser = fold_many0_with_const_init(
        preceded(
            tag("->"),
            space0_delimimited(ParseError::protect(
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

    map(parser, StatementVariant::ResourceTypeRelation)(input)
}

fn parse_resource(input: Span) -> IResult<puppet_lang::statement::Resource<Location>> {
    let parse_arguments = separated_list0(
        comma_separator,
        pair(
            space0_delimimited(parse_string_variant),
            preceded(
                ParseError::protect(|_| "'=>' is expected".to_string(), tag("=>")),
                space0_delimimited(ParseError::protect(
                    |_| "Argument value is expected".to_string(),
                    crate::expression::parse_expression,
                )),
            ),
        ),
    );

    let parser = tuple((
        space0_delimimited(crate::expression::parse_expression),
        preceded(
            ParseError::protect(|_| "':' is expected".to_string(), tag(":")),
            terminated(space0_delimimited(parse_arguments), opt(tag(";"))),
        ),
    ));

    map(parser, |(title, arguments)| {
        puppet_lang::statement::Resource {
            arguments,
            extra: title.extra.clone(),
            title,
        }
    })(input)
}

fn parse_resource_set(input: Span) -> IResult<puppet_lang::statement::ResourceSet<Location>> {
    let parser = pair(
        space0_delimimited(crate::identifier::identifier_with_toplevel),
        ParseError::protect(
            |_| "Resource set arguments list in '{ ... }' is expected".to_string(),
            space0_delimimited(crate::common::curly_brackets_comma_separated0(
                parse_resource,
            )),
        ),
    );

    map(parser, |(name, list)| puppet_lang::statement::ResourceSet {
        extra: name.extra.clone(),
        name,
        list,
    })(input)
}

fn parse_resource_set_relation(input: Span) -> IResult<StatementVariant<Location>> {
    let (input, left_set) = parse_resource_set(input)?;
    let parser = fold_many0_with_const_init(
        preceded(
            tag("->"),
            space0_delimimited(ParseError::protect(
                |_| "Second resource in relation is expected".to_string(),
                parse_resource_set,
            )),
        ),
        vec![left_set],
        |mut acc: Vec<_>, cur| {
            acc.push(cur);
            acc
        },
    );

    map(parser, StatementVariant::ResourceSetRelation)(input)
}

fn parse_if_else(input: Span) -> IResult<StatementVariant<Location>> {
    let parser_if = tuple((
        space0_delimimited(tag("if")),
        space0_delimimited(ParseError::protect(
            |_| "Condition is expected after 'if'".to_string(),
            crate::expression::parse_expression,
        )),
        parse_statement_set,
    ));

    let parser_elsif = many0(tuple((
        space0_delimimited(tag("elsif")),
        space0_delimimited(ParseError::protect(
            |_| "Condition is expected after 'elsif'".to_string(),
            crate::expression::parse_expression,
        )),
        parse_statement_set,
    )));

    let parser_else = preceded(space0_delimimited(tag("else")), parse_statement_set);

    let parser = tuple((parser_if, opt(parser_elsif), opt(parser_else)));

    let parser = map(parser, |(first, middle, else_block)| {
        let (tag, condition, body) = first;
        let if_block = puppet_lang::statement::ConditionAndStatement {
            condition,
            body: Box::new(body),
            extra: Location::from(tag),
        };

        let elsif_list = middle
            .unwrap_or_default()
            .into_iter()
            .map(
                |(tag, condition, body)| puppet_lang::statement::ConditionAndStatement {
                    condition,
                    body: Box::new(body),
                    extra: Location::from(tag),
                },
            )
            .collect();

        puppet_lang::statement::IfElse {
            if_block,
            elsif_list,
            else_block: else_block.map(Box::new),
            extra: Location::from(tag),
        }
    });

    map(parser, StatementVariant::IfElse)(input)
}

fn parse_statement_variant(input: Span) -> IResult<StatementVariant<Location>> {
    alt((
        parse_if_else,
        parse_require,
        parse_include,
        parse_contain,
        parse_tag,
        parse_resource_type_relation,
        parse_resource_set_relation,
        map(parse_resource_set, StatementVariant::ResourceSet),
        parse_expression,
    ))(input)
}

fn parse_statement(input: Span) -> IResult<Statement<Location>> {
    map(parse_statement_variant, |value| Statement {
        value,
        extra: Location::from(input),
    })(input)
}

pub fn parse_statement_set(input: Span) -> IResult<Vec<Statement<Location>>> {
    preceded(
        tag("{"),
        terminated(
            many0(space0_delimimited(parse_statement)),
            ParseError::protect(
                |_| "Closing '}' of body is expected".to_string(),
                space0_delimimited(tag("}")),
            ),
        ),
    )(input)
}
