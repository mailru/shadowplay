pub mod error;
pub mod hiera_yaml;
pub mod pp;
pub mod yaml;

#[derive(Debug, Clone)]
pub struct PuppetAst {
    pub input: String,
    pub data: Vec<puppet_lang::statement::Statement<puppet_parser::parser::Location>>,
}

impl PuppetAst {
    pub fn parse(i: &str) -> Result<Self, nom::Err<puppet_parser::parser::ParseError>> {
        let input = i.to_string();
        let (_remaining, data) =
            puppet_parser::statement::parse_statement_list(puppet_parser::parser::Span::new(i))?;
        Ok(Self { data, input })
    }
}
