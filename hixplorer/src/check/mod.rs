pub mod hiera_yaml;
pub mod pp;
pub mod pp_static;
pub mod yaml;

#[derive(Debug, Clone)]
pub struct PuppetAst {
    pub input: String,
    pub data: puppet_lang::toplevel::Toplevel<puppet_parser::parser::Location>,
}

impl PuppetAst {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let input = input.to_string();
        match puppet_parser::toplevel::parse(puppet_parser::parser::Span::new(&input)) {
            Ok((_remaining, data)) => Ok(Self { data, input }),
            Err(nom::Err::Failure(err)) => return Err(anyhow::format_err!("{}", err.to_string())),
            Err(err) => return Err(anyhow::format_err!("{}", err)),
        }
    }
}
