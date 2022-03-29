use std::collections::HashMap;

use crate::puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use super::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct OptionalArgumentsGoesFirst;

impl LintPass for OptionalArgumentsGoesFirst {
    fn name(&self) -> &str {
        "OptionalArgumentsGoesFirst"
    }
    fn description(&self) -> &str {
        "Warns if optional argument specified before required"
    }
}

impl OptionalArgumentsGoesFirst {
    fn check_order(&self, args: &[crate::puppet_lang::argument::Argument<Range>]) -> Vec<LintError> {
        let mut errors = Vec::new();
        let mut found_optional = false;
        for arg in args {
            if arg.default.is_some() {
                found_optional = true;
            } else if found_optional {
                errors.push(LintError::new(
                    Box::new(self.clone()),
                    "Required argument goes after optional",
                    &arg.extra,
                ))
            }
        }

        errors
    }
}

impl EarlyLintPass for OptionalArgumentsGoesFirst {
    fn check_class(&self, elt: &crate::puppet_lang::toplevel::Class<Range>) -> Vec<LintError> {
        self.check_order(&elt.arguments.value)
    }

    fn check_definition(&self, elt: &crate::puppet_lang::toplevel::Definition<Range>) -> Vec<LintError> {
        self.check_order(&elt.arguments.value)
    }

    fn check_plan(&self, elt: &crate::puppet_lang::toplevel::Plan<Range>) -> Vec<LintError> {
        self.check_order(&elt.arguments.value)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UniqueArgumentsNames;

impl LintPass for UniqueArgumentsNames {
    fn name(&self) -> &str {
        "UniqueArgumentsNames"
    }
    fn description(&self) -> &str {
        "Checks for class/definition/plan arguments uniqueness"
    }
}

impl UniqueArgumentsNames {
    fn check(&self, args: &[crate::puppet_lang::argument::Argument<Range>]) -> Vec<LintError> {
        let mut errors = Vec::new();
        let mut names: HashMap<String, &crate::puppet_lang::argument::Argument<Range>> = HashMap::new();
        for arg in args {
            match names.get(&arg.name) {
                Some(prev) => errors.push(LintError::new(
                    Box::new(self.clone()),
                    &format!(
                        "Argument '{}' was already defined earlier at line {}",
                        arg.name,
                        prev.extra.start().line()
                    ),
                    &arg.extra,
                )),
                None => {
                    let _ = names.insert(arg.name.clone(), arg);
                }
            }
        }

        errors
    }
}

impl EarlyLintPass for UniqueArgumentsNames {
    fn check_class(&self, elt: &crate::puppet_lang::toplevel::Class<Range>) -> Vec<LintError> {
        self.check(&elt.arguments.value)
    }

    fn check_definition(&self, elt: &crate::puppet_lang::toplevel::Definition<Range>) -> Vec<LintError> {
        self.check(&elt.arguments.value)
    }

    fn check_plan(&self, elt: &crate::puppet_lang::toplevel::Plan<Range>) -> Vec<LintError> {
        self.check(&elt.arguments.value)
    }
}
