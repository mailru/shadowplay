use crate::parser::Location;

use super::parser::{IResult, ParseError, Span};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

pub fn parse(input: Span) -> IResult<puppet_lang::argument::Argument<Location>> {
    let parser = tuple((
        super::common::space0_delimimited(opt(crate::typing::parse_type_specification)),
        preceded(
            tag("$"),
            ParseError::protect(
                |_| "Invalid variable name".to_owned(),
                crate::identifier::identifier,
            ),
        ),
        opt(preceded(
            crate::common::space0_delimimited(tag("=")),
            ParseError::protect(
                |_| {
                    // TODO
                    println!("================= Expected expression after '='");
                    "Expected expression after '='".to_owned()
                },
                crate::expression::parse_expression,
            ),
        )),
    ));

    map(parser, |(type_spec, name, default)| {
        puppet_lang::argument::Argument {
            type_spec,
            name: name.to_string(),
            default,
            extra: Location::from(name),
        }
    })(input)
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
