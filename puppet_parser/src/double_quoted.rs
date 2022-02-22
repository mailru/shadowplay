use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, verify};
use nom::multi::fold_many0;
use nom::sequence::{delimited, pair, preceded, terminated};
use puppet_lang::string::{DoubleQuotedFragment, StringExpr, StringFragment, StringVariant};

fn parse_literal(input: Span) -> IResult<StringFragment<Location>> {
    let not_quote_slash = is_not("\"\\$");
    map(
        verify(map(not_quote_slash, |s: Span| *s), |s: &str| !s.is_empty()),
        |data| StringFragment::Literal(data.to_string()),
    )(input)
}

fn parse_interpolation(input: Span) -> IResult<DoubleQuotedFragment<Location>> {
    let parser_variable = || {
        map(crate::identifier::identifier_with_toplevel, |identifier| {
            puppet_lang::expression::Expression {
                extra: identifier.extra.clone(),
                value: puppet_lang::expression::ExpressionVariant::Term(
                    puppet_lang::expression::Term {
                        extra: identifier.extra.clone(),
                        value: puppet_lang::expression::TermVariant::Variable(
                            puppet_lang::expression::Variable {
                                extra: identifier.extra.clone(),
                                accessor: Vec::new(),
                                identifier,
                            },
                        ),
                    },
                ),
            }
        })
    };

    let parser_delimited = alt((parser_variable(), crate::expression::parse_expression));

    let parser = alt((
        delimited(
            tag("{"),
            parser_delimited,
            ParseError::protect(|_| "Closing '}' expected".to_string(), tag("}")),
        ),
        parser_variable(),
    ));

    preceded(
        char('$'),
        alt((
            map(parser, DoubleQuotedFragment::Expression),
            map(anychar, |c| {
                DoubleQuotedFragment::StringFragment(StringFragment::Literal(format!("${}", c)))
            }),
        )),
    )(input)
}

fn parse_fragment(input: Span) -> IResult<DoubleQuotedFragment<Location>> {
    alt((
        map(parse_literal, DoubleQuotedFragment::StringFragment),
        parse_interpolation,
        map(
            crate::single_quoted::parse_unicode,
            DoubleQuotedFragment::StringFragment,
        ),
        map(
            crate::single_quoted::parse_escaped,
            DoubleQuotedFragment::StringFragment,
        ),
    ))(input)
}

pub fn parse(input: Span) -> IResult<StringExpr<Location>> {
    let build_string = fold_many0(parse_fragment, Vec::new, |mut list, fragment| {
        list.push(fragment);
        list
    });

    let double_quoted_parser = preceded(
        char('"'),
        ParseError::protect(
            |_| "Unterminated double quoted string".to_string(),
            terminated(build_string, char('"')),
        ),
    );

    map(
        pair(double_quoted_parser, crate::term::parse_accessor),
        |(data, accessor)| StringExpr {
            data: StringVariant::DoubleQuoted(data),
            accessor,
            extra: Location::from(input),
        },
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("\"\"")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::DoubleQuoted(vec![]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("\"a\"")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(puppet_lang::string::StringFragment::Literal(
                    "a".to_owned()
                ))
            ]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
    assert_eq!(
        parse(Span::new("\"\\\"\"")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(puppet_lang::string::StringFragment::Escaped(
                    puppet_lang::string::Escaped {
                        data: '"',
                        extra: Location::new(1, 1, 2)
                    }
                ))
            ]),
            accessor: Vec::new(),
            extra: Location::new(0, 1, 1)
        }
    );
}
