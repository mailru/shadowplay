use crate::{range::Range, IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, peek, recognize, verify};
use nom::multi::fold_many0;
use nom::sequence::{pair, tuple};
use puppet_lang::string::{
    DoubleQuotedFragment, Literal, StringExpr, StringFragment, StringVariant,
};

fn parse_literal(input: Span) -> IResult<StringFragment<Range>> {
    let not_quote_slash = is_not("\"\\$");
    map(
        verify(not_quote_slash, |s: &Span| !(*s).is_empty()),
        |data: Span| {
            StringFragment::Literal(Literal {
                extra: Range::from((data, data)),
                data: data.to_string(),
            })
        },
    )(input)
}

fn parse_interpolation(input: Span) -> IResult<DoubleQuotedFragment<Range>> {
    let parser_variable = || {
        map(
            pair(
                crate::identifier::identifier_with_toplevel,
                crate::term::parse_accessor,
            ),
            |(identifier, accessor)| puppet_lang::expression::Expression {
                extra: (&identifier.extra, &accessor, &identifier.extra).into(),
                value: puppet_lang::expression::ExpressionVariant::Term(
                    puppet_lang::expression::Term {
                        extra: (&identifier.extra, &accessor, &identifier.extra).into(),
                        value: puppet_lang::expression::TermVariant::Variable(
                            puppet_lang::expression::Variable {
                                extra: (&identifier.extra, &accessor, &identifier.extra).into(),
                                accessor,
                                identifier,
                            },
                        ),
                    },
                ),
                // Comments are not possible for interpolated expressions
                comment: vec![],
            },
        )
    };

    let parser_delimited = alt((parser_variable(), crate::expression::parse_expression));

    let (input, dollar_tag) = tag("$")(input)?;

    let parser = alt((
        map(
            tuple((tag("{"), parser_delimited, tag("}"))),
            |(_left_bracket, expr, _right_bracket)| {
                DoubleQuotedFragment::Expression(puppet_lang::string::Expression { data: expr })
            },
        ),
        map(parser_variable(), |expr| {
            DoubleQuotedFragment::Expression(puppet_lang::string::Expression { data: expr })
        }),
    ));

    let mut fragment_parser = alt((
        parser,
        map(peek(char('"')), |_| {
            DoubleQuotedFragment::StringFragment(StringFragment::Literal(
                puppet_lang::string::Literal {
                    extra: Range::from((dollar_tag, dollar_tag)),
                    data: (*dollar_tag).to_owned(),
                },
            ))
        }),
        map(recognize(anychar), |c: Span| {
            DoubleQuotedFragment::StringFragment(StringFragment::Literal(
                puppet_lang::string::Literal {
                    extra: Range::from((dollar_tag, c)),
                    data: c.to_string(),
                },
            ))
        }),
    ));

    fragment_parser(input)
}

fn parse_fragment(input: Span) -> IResult<DoubleQuotedFragment<Range>> {
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

pub fn parse(input: Span) -> IResult<StringExpr<Range>> {
    let build_string = fold_many0(parse_fragment, Vec::new, |mut list, fragment| {
        list.push(fragment);
        list
    });

    let double_quoted_parser = tuple((
        tag("\""),
        build_string,
        ParseError::protect(
            |_| "Unterminated double quoted string".to_string(),
            tag("\""),
        ),
    ));

    map(
        pair(double_quoted_parser, crate::term::parse_accessor),
        |((left_quote, data, right_quote), accessor)| StringExpr {
            data: StringVariant::DoubleQuoted(data),
            extra: Range::from((&left_quote, &accessor, &right_quote)),
            accessor,
        },
    )(input)
}

#[test]
fn test() {
    assert_eq!(
        parse(Span::new("\"\"")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::DoubleQuoted(vec![]),
            accessor: None,
            extra: Range::new(0, 1, 1, 1, 1, 2)
        }
    );
    assert_eq!(
        parse(Span::new("\"a\"")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(puppet_lang::string::StringFragment::Literal(
                    puppet_lang::string::Literal {
                        data: "a".to_owned(),
                        extra: Range::new(1, 1, 2, 1, 1, 2)
                    }
                ))
            ]),
            accessor: None,
            extra: Range::new(0, 1, 1, 2, 1, 3)
        }
    );
    assert_eq!(
        parse(Span::new("\"\\\"\"")).unwrap().1,
        puppet_lang::string::StringExpr {
            data: puppet_lang::string::StringVariant::DoubleQuoted(vec![
                DoubleQuotedFragment::StringFragment(puppet_lang::string::StringFragment::Escaped(
                    puppet_lang::string::Escaped {
                        data: '"',
                        extra: Range::new(1, 1, 2, 2, 1, 3)
                    }
                ))
            ]),
            accessor: None,
            extra: Range::new(0, 1, 1, 3, 1, 4)
        }
    );
}
