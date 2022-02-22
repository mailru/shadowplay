use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::combinator::{map, verify};
use nom::multi::fold_many0;
use nom::sequence::{pair, preceded, terminated};
use puppet_lang::string::{DoubleQuotedFragment, StringExpr, StringFragment, StringVariant};

fn parse_literal(input: Span) -> IResult<StringFragment<Location>> {
    let not_quote_slash = is_not("\"\\");
    map(
        verify(map(not_quote_slash, |s: Span| *s), |s: &str| !s.is_empty()),
        |data| StringFragment::Literal(data.to_string()),
    )(input)
}

fn parse_fragment(input: Span) -> IResult<DoubleQuotedFragment<Location>> {
    // TODO parse interpolations
    alt((
        map(parse_literal, DoubleQuotedFragment::StringFragment),
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
