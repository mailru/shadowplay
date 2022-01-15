use std::collections::HashMap;

use puppet_parser::parser::Location;

use super::lint::{EarlyLintPass, LintError, LintPass};

pub struct OptionalArgumentsGoesFirst;

impl LintPass for OptionalArgumentsGoesFirst {
    fn name(&self) -> &str {
        "optional_arguments_goes_first"
    }
}

impl OptionalArgumentsGoesFirst {
    fn check_order(&self, args: &[puppet_lang::argument::Argument<Location>]) -> Vec<LintError> {
        let mut errors = Vec::new();
        let mut found_optional = false;
        for arg in args {
            if arg.default.is_none() {
                found_optional = true;
            } else {
                if found_optional {
                    errors.push(LintError::new(
                        self.name(),
                        "Required argument goes after optional",
                        &arg.extra,
                    ))
                }
            }
        }

        errors
    }
}

impl EarlyLintPass for OptionalArgumentsGoesFirst {
    fn check_class(&self, elt: &puppet_lang::toplevel::Class<Location>) -> Vec<LintError> {
        self.check_order(&elt.arguments)
    }

    fn check_definition(
        &self,
        elt: &puppet_lang::toplevel::Definition<Location>,
    ) -> Vec<LintError> {
        self.check_order(&elt.arguments)
    }

    fn check_plan(&self, elt: &puppet_lang::toplevel::Plan<Location>) -> Vec<LintError> {
        self.check_order(&elt.arguments)
    }
}

pub struct UniqueArgumentsNames;

impl LintPass for UniqueArgumentsNames {
    fn name(&self) -> &str {
        "unique_arguments_names"
    }
}

impl UniqueArgumentsNames {
    fn check(&self, args: &[puppet_lang::argument::Argument<Location>]) -> Vec<LintError> {
        let mut errors = Vec::new();
        let mut names: HashMap<String, &puppet_lang::argument::Argument<Location>> = HashMap::new();
        for arg in args {
            match names.get(&arg.name) {
                Some(prev) => errors.push(LintError::new(
                    self.name(),
                    &format!(
                        "Argument '{}' was already defined earlier at line {}",
                        arg.name,
                        prev.extra.line()
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
    fn check_class(&self, elt: &puppet_lang::toplevel::Class<Location>) -> Vec<LintError> {
        self.check(&elt.arguments)
    }

    fn check_definition(
        &self,
        elt: &puppet_lang::toplevel::Definition<Location>,
    ) -> Vec<LintError> {
        self.check(&elt.arguments)
    }

    fn check_plan(&self, elt: &puppet_lang::toplevel::Plan<Location>) -> Vec<LintError> {
        self.check(&elt.arguments)
    }
}
