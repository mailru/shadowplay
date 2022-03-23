pub mod builtin_resources;

use std::collections::HashMap;

use puppet_parser::range::Range;

use crate::ctx::builtin_resources::Attribute;

#[derive(Clone)]
pub struct NamedBlock {
    pub value: puppet_lang::toplevel::Toplevel<Range>,
}

pub struct Ctx {
    pub repository_path: std::path::PathBuf,
    pub resources: std::cell::RefCell<HashMap<Vec<String>, Option<NamedBlock>>>,
    pub builtin_resources: HashMap<&'static str, crate::ctx::builtin_resources::Resource>,
    pub resource_metaparameters: HashMap<&'static str, Attribute>,
}

impl Ctx {
    pub fn new(repository_path: &std::path::Path) -> Self {
        let mut resource_metaparameters = HashMap::new();
        let _ = resource_metaparameters.insert("alias", Attribute::default());
        let _ = resource_metaparameters.insert("audit", Attribute::default());
        let _ = resource_metaparameters.insert("before", Attribute::default());
        let _ = resource_metaparameters.insert("loglevel", Attribute::default());
        let _ = resource_metaparameters.insert("noop", Attribute::default());
        let _ = resource_metaparameters.insert("notify", Attribute::default());
        let _ = resource_metaparameters.insert("require", Attribute::default());
        let _ = resource_metaparameters.insert("schedule", Attribute::default());
        let _ = resource_metaparameters.insert("stage", Attribute::default());
        let _ = resource_metaparameters.insert("subscribe", Attribute::default());
        let _ = resource_metaparameters.insert("tag", Attribute::default());

        Self {
            repository_path: repository_path.to_path_buf(),
            resources: std::cell::RefCell::new(HashMap::new()),
            builtin_resources: crate::ctx::builtin_resources::generate(),
            resource_metaparameters,
        }
    }

    fn fill_named_blocks(&self, name: &[String]) {
        // Lookup recursively
        if name.len() > 1 {
            if let &[list @ .., _suffix] = &name {
                self.fill_named_blocks(list)
            }
        }

        let module = match puppet_tool::module::Module::of_identifier(name) {
            None => {
                return;
            }
            Some(v) => v,
        };

        let file_content =
            match std::fs::read_to_string(module.full_file_path(&self.repository_path)) {
                Err(_err) => {
                    return;
                }
                Ok(v) => v,
            };

        let (_, statement_list) = match puppet_parser::toplevel::parse_file(
            puppet_parser::Span::new(file_content.as_str()),
        ) {
            Err(_err) => {
                return;
            }
            Ok(v) => v,
        };

        for statement in statement_list.value {
            match statement.value {
                puppet_lang::statement::StatementVariant::Toplevel(toplevel) => {
                    let name = match &toplevel.data {
                        puppet_lang::toplevel::ToplevelVariant::Class(v) => {
                            Some(&v.identifier.name)
                        }
                        puppet_lang::toplevel::ToplevelVariant::Definition(v) => {
                            Some(&v.identifier.name)
                        }
                        puppet_lang::toplevel::ToplevelVariant::Plan(v) => Some(&v.identifier.name),
                        // TODO
                        _ => None,
                    };

                    if let Some(name) = name {
                        let mut resources = self.resources.borrow_mut();
                        let _ =
                            resources.insert(name.clone(), Some(NamedBlock { value: toplevel }));
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
    }

    pub fn block_of_name(&self, name: &[String]) -> Option<NamedBlock> {
        {
            if let Some(v) = self.resources.borrow().get(&name.to_vec()) {
                return v.as_ref().cloned();
            }
        }

        self.fill_named_blocks(name);

        let mut resources = self.resources.borrow_mut();

        resources
            .entry(name.to_vec())
            .or_insert(None)
            .as_ref()
            .cloned()
    }
}
