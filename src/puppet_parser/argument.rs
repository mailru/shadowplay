use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    pub type_spec: Option<super::typing::TypeSpecification>,
    pub name: String,
    pub default: Option<super::expression::Expression>,
}

impl Argument {
    pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: nom::error::ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map(
            tuple((
                super::common::space0_delimimited(opt(super::typing::TypeSpecification::parse)),
                preceded(tag("$"), super::common::identifier),
                opt(preceded(
                    super::common::space0_delimimited(tag("=")),
                    super::common::space0_delimimited(super::expression::Expression::parse),
                )),
            )),
            |(type_spec, name, default)| Self {
                type_spec,
                name: name.to_string(),
                default,
            },
        )(input)
    }
}

#[test]
fn test_argument() {
    assert_eq!(
        Argument::parse::<nom::error::Error<_>>("Any $v   =  1").unwrap(),
        (
            "",
            Argument {
                type_spec: Some(super::typing::TypeSpecification::Any),
                name: "v".to_owned(),
                default: Some(super::expression::Expression::Term(
                    super::expression::Term::Float(1.0)
                ))
            }
        )
    );

    assert!(tuple::<_, _, nom::error::Error<_>, _>((
        super::common::space0_delimimited(opt(super::typing::TypeSpecification::parse)),
        tag("$")
    ))("Hash[String, String] $aaa")
    .is_ok());
    assert!(Argument::parse::<nom::error::Error<_>>("Hash[String, String] $aaa").is_ok());
}
