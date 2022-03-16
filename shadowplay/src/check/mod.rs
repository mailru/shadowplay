use puppet_parser::range::Range;

pub mod error;
pub mod hiera_yaml;
pub mod pp;
pub mod yaml;

#[derive(Debug, Clone)]
pub struct PuppetAst {
    pub input: String,
    pub data: puppet_lang::List<Range, puppet_lang::statement::Statement<Range>>,
}

impl PuppetAst {
    pub fn parse(i: &str) -> Result<Self, nom::Err<puppet_parser::ParseError>> {
        let input = i.to_string();
        let (_remaining, data) =
            puppet_parser::statement::parse_statement_list(puppet_parser::Span::new(i))?;
        Ok(Self { data, input })
    }
}

pub struct Summary {
    pub errors_count: usize,
    pub files_checked: usize,
}