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
