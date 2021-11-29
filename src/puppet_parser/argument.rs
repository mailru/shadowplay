use super::parser::{IResult, Marked, Span};
use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    pub type_spec: Option<Marked<super::typing::TypeSpecification>>,
    pub name: String,
    pub default: Option<Marked<super::expression::Expression>>,
}

impl Argument {
    pub fn parse(input: Span) -> IResult<Self> {
        let parser = map(
            tuple((
                super::common::space0_delimimited(opt(super::typing::TypeSpecification::parse)),
                preceded(tag("$"), super::common::identifier),
                opt(preceded(
                    super::common::space0_delimimited(Marked::parse(tag("="))),
                    super::common::space0_delimimited(super::expression::Expression::parse),
                )),
            )),
            |(type_spec, name, default)| Self {
                type_spec,
                name: name.data.to_string(),
                default,
            },
        );

        Marked::parse(parser)(input)
    }
}

#[test]
fn test_argument() {
    assert_eq!(
        Argument::parse(Span::new("Any $v   =  1")).unwrap().1,
        Marked {
            line: 1,
            column: 1,
            data: Argument {
                type_spec: Some(Marked {
                    line: 1,
                    column: 1,
                    data: super::typing::TypeSpecification::Any
                }),
                name: "v".to_owned(),
                default: Some(Marked {
                    line: 1,
                    column: 13,
                    data: super::expression::Expression::Term(super::expression::Term::Float(1.0))
                })
            }
        }
    );

    assert!(tuple((
        super::common::space0_delimimited(opt(super::typing::TypeSpecification::parse)),
        tag("$")
    ))(Span::new("Hash[String, String] $aaa"))
    .is_ok());
    assert!(Argument::parse(Span::new("Hash[String, String] $aaa")).is_ok());
}
