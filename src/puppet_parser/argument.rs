use crate::puppet_parser::{common::capture_comment, range::Range, IResult, ParseError, Span};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
};

pub fn parse(input: Span) -> IResult<crate::puppet_lang::argument::Argument<Range>> {
    let parser = tuple((
        super::common::space0_delimimited(opt(crate::puppet_parser::typing::parse_type_specification)),
        tag("$"),
        ParseError::protect(
            |_| "Invalid variable name".to_owned(),
            crate::puppet_parser::identifier::identifier,
        ),
        opt(preceded(
            crate::puppet_parser::common::space0_delimimited(tag("=")),
            ParseError::protect(
                |_| "Expected expression after '='".to_owned(),
                crate::puppet_parser::expression::parse_expression,
            ),
        )),
    ));

    map(
        pair(capture_comment, parser),
        move |(comment, (type_spec, dollar_sign, name, default))| {
            let start_range = match &type_spec {
                None => Range::from((dollar_sign, dollar_sign)),
                Some(v) => v.extra.clone(),
            };
            let end_range = match &default {
                None => Range::from((name, name)),
                Some(v) => v.extra.clone(),
            };
            crate::puppet_lang::argument::Argument {
                type_spec,
                extra: Range::from((&start_range, &end_range)),
                name: name.to_string(),
                default,
                comment,
            }
        },
    )(input)
}

// #[test]
// fn test_argument() {
//     assert_eq!(
//         Argument::parse(Span::new("Any $v   =  1")).unwrap().1,
//         Marked {
//             line: 1,
//             column: 1,
//             data: Argument {
//                 type_spec: Some(Marked {
//                     line: 1,
//                     column: 1,
//                     data: super::typing::TypeSpecification::Any
//                 }),
//                 name: "v".to_owned(),
//                 default: Some(Marked {
//                     line: 1,
//                     column: 13,
//                     data: super::expression::Expression::Term(super::expression::Term::Integer(1))
//                 })
//             }
//         }
//     );

//     assert!(tuple((
//         super::common::space0_delimimited(opt(super::typing::TypeSpecification::parse)),
//         tag("$")
//     ))(Span::new("Hash[String, String] $aaa"))
//     .is_ok());
//     assert!(Argument::parse(Span::new("Hash[String, String] $aaa")).is_ok());
// }
