use puppet_parser::parser::Location;

pub trait LintPass {
    fn name(&self) -> &str;
}

pub trait EarlyLintPass: LintPass {
    fn check_toplevel(&self, _: &puppet_lang::toplevel::Toplevel<Location>) {}
    fn check_class(&self, _: &puppet_lang::toplevel::Class<Location>) {}
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

        // TODO register linters
        v.register_early_pass(Box::new(Simple));

        v
    }

    pub fn early_pass(&self) -> &[Box<dyn EarlyLintPass>] {
        &self.early_pass
    }
}

pub struct Simple;

impl LintPass for Simple {
    fn name(&self) -> &str {
        "test_lint"
    }
}

impl EarlyLintPass for Simple {
    fn check_class(&self, _: &puppet_lang::toplevel::Class<Location>) {
        todo!()
    }
}

pub struct AstLinter;

impl AstLinter {
    pub fn check_class(&self, storage: &Storage, elt: &puppet_lang::toplevel::Class<Location>) {
        for lint in storage.early_pass() {
            lint.check_class(elt)
        }
    }

    pub fn check_toplevel(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::Toplevel<Location>,
    ) {
        for lint in storage.early_pass() {
            lint.check_toplevel(elt)
        }
        match elt {
            puppet_lang::toplevel::Toplevel::Class(elt) => self.check_class(storage, elt),
            puppet_lang::toplevel::Toplevel::Definition(_) => todo!(),
            puppet_lang::toplevel::Toplevel::Plan(_) => todo!(),
        }
    }
}
