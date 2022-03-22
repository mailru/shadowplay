use std::collections::HashMap;

use puppet_parser::range::Range;

pub struct NamedBlock {
    pub value: puppet_lang::toplevel::Toplevel<Range>,
}

pub struct Ctx {
    pub repository_path: std::path::PathBuf,
    pub resources: HashMap<Vec<String>, Option<NamedBlock>>,
}

impl Ctx {
    pub fn new(repository_path: &std::path::Path) -> Self {
        Self {
            repository_path: repository_path.to_path_buf(),
            resources: HashMap::new(),
        }
    }

    fn calculate_named_block(
        repository_path: &std::path::Path,
        name: &[String],
    ) -> Option<NamedBlock> {
        let module = match puppet_tool::module::Module::of_identifier(name) {
            None => {
                return None;
            }
            Some(v) => v,
        };

        let file_content = match std::fs::read_to_string(module.full_file_path(repository_path)) {
            Err(_err) => {
                return None;
            }
            Ok(v) => v,
        };

        let (_, statement_list) = match puppet_parser::toplevel::parse_file(
            puppet_parser::Span::new(file_content.as_str()),
        ) {
            Err(_err) => {
                return None;
            }
            Ok(v) => v,
        };

        for statement in statement_list.value {
            match statement.value {
                puppet_lang::statement::StatementVariant::Toplevel(toplevel) => {
                    match &toplevel.data {
                        puppet_lang::toplevel::ToplevelVariant::Class(v)
                            if v.identifier.name.as_slice() == name =>
                        {
                            return Some(NamedBlock { value: toplevel })
                        }
                        puppet_lang::toplevel::ToplevelVariant::Definition(v)
                            if v.identifier.name.as_slice() == name =>
                        {
                            return Some(NamedBlock { value: toplevel })
                        }
                        puppet_lang::toplevel::ToplevelVariant::Plan(v)
                            if v.identifier.name.as_slice() == name =>
                        {
                            return Some(NamedBlock { value: toplevel })
                        }
                        // TODO
                        _ => (),
                    }
                }
                puppet_lang::statement::StatementVariant::Expression(_)
                | puppet_lang::statement::StatementVariant::RelationList(_)
                | puppet_lang::statement::StatementVariant::IfElse(_)
                | puppet_lang::statement::StatementVariant::Unless(_)
                | puppet_lang::statement::StatementVariant::Case(_)
                | puppet_lang::statement::StatementVariant::ResourceDefaults(_) => (),
            }
        }

        None
    }

    pub fn block_of_name<'a>(&'a mut self, name: &[String]) -> Option<&'a NamedBlock> {
        self.resources
            .entry(name.to_vec())
            .or_insert_with(|| Self::calculate_named_block(&self.repository_path, name))
            .as_ref()
    }
}
