use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{many0, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};
use puppet_lang::statement::{Statement, StatementVariant};
use puppet_lang::ExtraGetter;

use crate::{
    common::{
        comma_separator, curly_brackets_delimimited, space0_delimimited, spaced0_separator,
        spaced_word, square_brackets_comma_separated1,
    },
    term::parse_string_variant,
    {range::Range, IResult, ParseError, Span},
};

fn parse_expression(input: Span) -> IResult<StatementVariant<Range>> {
    map(
        crate::expression::parse_expression,
        StatementVariant::Expression,
    )(input)
}

fn parse_resource(input: Span) -> IResult<puppet_lang::statement::Resource<Range>> {
    let parse_attribute = map(
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
        puppet_lang::statement::ResourceAttribute::Name,
    );

    let parse_attribute_group = map(
        pair(
            space0_delimimited(tag("*")),
            preceded(
                ParseError::protect(|_| "'=>' is expected".to_string(), tag("=>")),
                space0_delimimited(ParseError::protect(
                    |_| "Argument group value is expected".to_string(),
                    crate::term::parse_term,
                )),
            ),
        ),
        |(_, term)| puppet_lang::statement::ResourceAttribute::Group(term),
    );

    let parse_arguments = separated_list0(
        comma_separator,
        alt((parse_attribute, parse_attribute_group)),
    );

    let mut parser = map(
        tuple((
            space0_delimimited(crate::expression::parse_expression),
            preceded(tag(":"), space0_delimimited(parse_arguments)),
            opt(tag(",")),
        )),
        |(title, arguments, opt_comma)| {
            let last_range = match opt_comma {
                Some(v) => Range::from((v, v)),
                None => match arguments.last() {
                    Some(puppet_lang::statement::ResourceAttribute::Name((_, v))) => {
                        v.extra.clone()
                    }
                    Some(puppet_lang::statement::ResourceAttribute::Group(v)) => v.extra.clone(),
                    None => title.extra.clone(),
                },
            };
            puppet_lang::statement::Resource {
                attributes: arguments,
                extra: (&title.extra, &last_range).into(),
                title,
            }
        },
    );

    parser(input)
}

fn parse_resource_set(input: Span) -> IResult<puppet_lang::statement::ResourceSet<Range>> {
    let parser = tuple((
        space0_delimimited(pair(
            opt(tag("@")),
            crate::identifier::anycase_identifier_with_ns,
        )),
        space0_delimimited(crate::common::curly_brackets_delimimited(terminated(
            separated_list0(spaced0_separator(";"), parse_resource),
            opt(spaced0_separator(";")),
        ))),
    ));

    map(
        parser,
        |((virtual_tag, name), (_left_curly, list, right_curly))| {
            let start_range = match virtual_tag {
                Some(v) => Range::from((v, v)),
                None => name.extra.clone(),
            };
            puppet_lang::statement::ResourceSet {
                is_virtual: virtual_tag.is_some(),
                extra: Range::from((&start_range, right_curly)),
                name,
                list,
            }
        },
    )(input)
}

fn parse_relation_type(input: Span) -> IResult<puppet_lang::statement::RelationType<Range>> {
    alt((
        map(tag("->"), |tag: Span| {
            puppet_lang::statement::RelationType {
                variant: puppet_lang::statement::RelationVariant::ExecOrderRight,
                extra: (tag, tag).into(),
            }
        }),
        map(tag("~>"), |tag: Span| {
            puppet_lang::statement::RelationType {
                variant: puppet_lang::statement::RelationVariant::NotifyRight,
                extra: (tag, tag).into(),
            }
        }),
        map(tag("<-"), |tag: Span| {
            puppet_lang::statement::RelationType {
                variant: puppet_lang::statement::RelationVariant::ExecOrderLeft,
                extra: (tag, tag).into(),
            }
        }),
        map(tag("<~"), |tag: Span| {
            puppet_lang::statement::RelationType {
                variant: puppet_lang::statement::RelationVariant::NotifyLeft,
                extra: (tag, tag).into(),
            }
        }),
    ))(input)
}

fn parse_relation(input: Span) -> IResult<puppet_lang::statement::RelationList<Range>> {
    let head_parser = alt((
        map(
            square_brackets_comma_separated1(alt((
                map(
                    parse_resource_set,
                    puppet_lang::statement::RelationEltVariant::ResourceSet,
                ),
                map(
                    crate::resource_collection::parse_resource_collection,
                    puppet_lang::statement::RelationEltVariant::ResourceCollection,
                ),
            ))),
            |(left_curly, data, right_curly)| puppet_lang::statement::RelationElt {
                data,
                extra: Range::from((left_curly, right_curly)),
            },
        ),
        map(
            alt((
                map(
                    parse_resource_set,
                    puppet_lang::statement::RelationEltVariant::ResourceSet,
                ),
                map(
                    crate::resource_collection::parse_resource_collection,
                    puppet_lang::statement::RelationEltVariant::ResourceCollection,
                ),
            )),
            |elt| puppet_lang::statement::RelationElt {
                extra: Range::from((elt.extra(), elt.extra())),
                data: vec![elt],
            },
        ),
    ));

    let tail_parser = opt(map(
        pair(
            space0_delimimited(parse_relation_type),
            space0_delimimited(ParseError::protect(
                |_| "Second resource or type is expected after relation tag".to_string(),
                parse_relation,
            )),
        ),
        |(relation_type, relation_to)| puppet_lang::statement::Relation {
            relation_type,
            relation_to: Box::new(relation_to),
        },
    ));

    map(pair(head_parser, tail_parser), |(head, tail)| {
        let end_range = match &tail {
            Some(v) => v.relation_to.extra.clone(),
            None => head.extra.clone(),
        };
        puppet_lang::statement::RelationList {
            extra: Range::from((&head.extra, &end_range)),
            head,
            tail,
        }
    })(input)
}

fn parse_if_else(input: Span) -> IResult<StatementVariant<Range>> {
    let parser_if = tuple((
        spaced_word("if"),
        space0_delimimited(ParseError::protect(
            |_| "Condition is expected after 'if'".to_string(),
            crate::expression::parse_expression,
        )),
        ParseError::protect(
            |_| "Statement block expected 'if' condition".to_string(),
            parse_statement_block,
        ),
    ));

    let parser_elsif = many0(tuple((
        spaced_word("elsif"),
        space0_delimimited(ParseError::protect(
            |_| "Condition is expected after 'elsif'".to_string(),
            crate::expression::parse_expression,
        )),
        ParseError::protect(
            |_| "Statement block expected 'elsif' condition".to_string(),
            parse_statement_block,
        ),
    )));

    let parser_else = preceded(
        spaced_word("else"),
        ParseError::protect(
            |_| "Statement block expected 'else'".to_string(),
            parse_statement_block,
        ),
    );

    let parser = tuple((parser_if, opt(parser_elsif), opt(parser_else)));

    let parser = map(parser, |(first, middle, else_block)| {
        let (tag, condition, (_left_curly, body, right_curly)) = first;
        let if_block = puppet_lang::statement::ConditionAndStatement {
            condition,
            body: Box::new(body),
            extra: (tag, right_curly).into(),
        };

        let elsif_list: Vec<_> = middle
            .unwrap_or_default()
            .into_iter()
            .map(|(tag, condition, (_left_curly, body, right_curly))| {
                puppet_lang::statement::ConditionAndStatement {
                    condition,
                    body: Box::new(body),
                    extra: (tag, right_curly).into(),
                }
            })
            .collect();

        let end_range = match else_block {
            Some((_, _, right_curly)) => Range::from((right_curly, right_curly)),
            None => match &elsif_list.last() {
                Some(v) => v.extra.clone(),
                None => if_block.extra.clone(),
            },
        };

        puppet_lang::statement::IfElse {
            extra: (&if_block.extra, &end_range).into(),
            condition: if_block,
            elsif_list,
            else_block: else_block.map(|body| Box::new(body.1)),
        }
    });

    map(parser, StatementVariant::IfElse)(input)
}

fn parse_unless(input: Span) -> IResult<StatementVariant<Range>> {
    let parser = tuple((
        space0_delimimited(tag("unless")),
        space0_delimimited(ParseError::protect(
            |_| "Condition is expected after 'unless'".to_string(),
            crate::expression::parse_expression,
        )),
        parse_statement_block,
    ));

    map(
        parser,
        |(op, condition, (_left_curly, body, right_curly))| {
            StatementVariant::Unless(puppet_lang::statement::ConditionAndStatement {
                condition,
                body: Box::new(body),
                extra: (op, right_curly).into(),
            })
        },
    )(input)
}

fn parse_case(input: Span) -> IResult<StatementVariant<Range>> {
    let parser_header = pair(
        spaced_word("case"),
        space0_delimimited(ParseError::protect(
            |_| "Condition is expected after 'case'".to_string(),
            crate::expression::parse_expression,
        )),
    );

    let parser_element = map(
        tuple((
            separated_list1(
                comma_separator,
                space0_delimimited(crate::expression::parse_case_variant),
            ),
            tag(":"),
            space0_delimimited(parse_statement_block),
        )),
        |(matches, _tag, (_left_curly, body, right_curly))| puppet_lang::statement::CaseElement {
            extra: (matches.first().unwrap().extra(), right_curly).into(),
            matches,
            body: Box::new(body),
        },
    );

    let parser = pair(
        parser_header,
        curly_brackets_delimimited(many0(parser_element)),
    );

    let parser = map(
        parser,
        |((case_tag, condition), (_left_curly, elements, right_curly))| {
            puppet_lang::statement::Case {
                condition,
                elements,
                extra: (case_tag, right_curly).into(),
            }
        },
    );

    map(parser, StatementVariant::Case)(input)
}

fn parse_resource_defaults(
    input: Span,
) -> IResult<puppet_lang::statement::ResourceDefaults<Range>> {
    let kv_parser = pair(
        space0_delimimited(crate::term::parse_term),
        preceded(
            tag("=>"),
            space0_delimimited(ParseError::protect(
                |_| "Expression expected after '=>'".to_string(),
                crate::expression::parse_expression,
            )),
        ),
    );

    map(
        pair(
            crate::identifier::camel_case_identifier,
            space0_delimimited(curly_brackets_delimimited(terminated(
                separated_list0(comma_separator, kv_parser),
                opt(space0_delimimited(tag(","))),
            ))),
        ),
        |(name, (_left_curly, args, right_curly))| puppet_lang::statement::ResourceDefaults {
            name: name.to_string(),
            args,
            extra: Range::from((name, right_curly)),
        },
    )(input)
}

fn parse_statement_variant(input: Span) -> IResult<StatementVariant<Range>> {
    alt((
        parse_if_else,
        parse_unless,
        parse_case,
        map(parse_resource_defaults, StatementVariant::ResourceDefaults),
        map(parse_relation, StatementVariant::RelationList),
        map(crate::toplevel::parse, StatementVariant::Toplevel),
        parse_expression,
    ))(input)
}

fn parse_statement(input: Span) -> IResult<Statement<Range>> {
    map(parse_statement_variant, |value| Statement { value })(input)
}

pub fn parse_statement_list(input: Span) -> IResult<Vec<Statement<Range>>> {
    many0(terminated(
        space0_delimimited(parse_statement),
        opt(space0_delimimited(tag(";"))),
    ))(input)
}

pub fn parse_statement_block(input: Span) -> IResult<(Span, Vec<Statement<Range>>, Span)> {
    tuple((
        tag("{"),
        parse_statement_list,
        ParseError::protect(
            |_| "Closing '}' or statement is expected".to_string(),
            space0_delimimited(tag("}")),
        ),
    ))(input)
}

#[test]
fn test_selector() {
    assert!(parse_statement_block(Span::new("{ if $z { $a ? { default => 0, } } }")).is_ok())
}
