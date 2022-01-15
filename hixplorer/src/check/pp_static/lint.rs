use puppet_parser::parser::Location;

#[derive(Clone)]
pub struct LintError {
    pub linter: String,
    pub message: String,
    pub location: Location,
}

impl LintError {
    pub fn new(linter: &str, message: &str, location: &Location) -> Self {
        Self {
            linter: linter.to_owned(),
            message: message.to_owned(),
            location: location.clone(),
        }
    }
}

pub trait LintPass {
    fn name(&self) -> &str;
}

pub trait EarlyLintPass: LintPass {
    fn check_toplevel(&self, _: &puppet_lang::toplevel::Toplevel<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_class(&self, _: &puppet_lang::toplevel::Class<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_definition(&self, _: &puppet_lang::toplevel::Definition<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_plan(&self, _: &puppet_lang::toplevel::Plan<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_argument(&self, _: &puppet_lang::argument::Argument<Location>) -> Vec<LintError> {
        Vec::new()
    }
}

pub struct Storage {
    early_pass: Vec<Box<dyn EarlyLintPass>>,
}

impl Storage {
    pub fn register_early_pass(&mut self, lint: Box<dyn EarlyLintPass>) {
        self.early_pass.push(lint)
    }

    pub fn new() -> Self {
        let mut v = Self {
            early_pass: Vec::new(),
        };

        v.register_early_pass(Box::new(super::lint_toplevel::OptionalArgumentsGoesFirst));
        v.register_early_pass(Box::new(super::lint_argument::ArgumentLooksSensitive));
        // v.register_early_pass(Box::new(super::lint_argument::ArgumentTyped));

        v
    }

    pub fn early_pass(&self) -> &[Box<dyn EarlyLintPass>] {
        &self.early_pass
    }
}

pub struct AstLinter;

impl AstLinter {
    pub fn check_class(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::Class<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_class(elt));
            for arg in &elt.arguments {
                errors.append(&mut lint.check_argument(arg));
            }
        }

        errors
    }

    pub fn check_definition(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::Definition<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_definition(elt));
            for arg in &elt.arguments {
                errors.append(&mut lint.check_argument(arg));
            }
        }

        errors
    }

    pub fn check_plan(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::Plan<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_plan(elt));
            for arg in &elt.arguments {
                errors.append(&mut lint.check_argument(arg));
            }
        }

        errors
    }

    pub fn check_toplevel(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::Toplevel<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        for lint in storage.early_pass() {
            errors.append(&mut lint.check_toplevel(elt))
        }
        let mut res = match elt {
            puppet_lang::toplevel::Toplevel::Class(elt) => self.check_class(storage, elt),
            puppet_lang::toplevel::Toplevel::Definition(elt) => self.check_definition(storage, elt),
            puppet_lang::toplevel::Toplevel::Plan(elt) => self.check_plan(storage, elt),
        };
        errors.append(&mut res);

        errors
    }
}
