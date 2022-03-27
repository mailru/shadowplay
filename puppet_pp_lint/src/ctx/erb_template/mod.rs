mod parser;

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Template {
    pub referenced_variables: HashSet<String>,
}

impl Template {
    pub fn read(path: &std::path::Path) -> Option<Self> {
        let file_content = match std::fs::read_to_string(path) {
            Err(_err) => {
                return None;
            }
            Ok(v) => v,
        };

        parser::parse_toplevel(&file_content)
            .ok()
            .map(|(_, template)| template)
    }
}
