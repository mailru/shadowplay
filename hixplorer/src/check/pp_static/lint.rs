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
    fn check_statement(&self, _: &puppet_lang::statement::Statement<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_unless(
        &self,
        _: &puppet_lang::statement::ConditionAndStatement<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_expression(
        &self,
        _: &puppet_lang::expression::Expression<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_term(&self, _: &puppet_lang::expression::Term<Location>) -> Vec<LintError> {
        Vec::new()
    }
}

#[derive(Default)]
pub struct Storage {
    early_pass: Vec<Box<dyn EarlyLintPass>>,
}

impl Storage {
    pub fn register_early_pass(&mut self, lint: Box<dyn EarlyLintPass>) {
        self.early_pass.push(lint)
    }

    pub fn new() -> Self {
        let mut v = Self::default();

        v.register_early_pass(Box::new(super::lint_toplevel::OptionalArgumentsGoesFirst));
        v.register_early_pass(Box::new(super::lint_toplevel::UniqueArgumentsNames));
        v.register_early_pass(Box::new(super::lint_argument::ArgumentLooksSensitive));
        // v.register_early_pass(Box::new(super::lint_argument::ArgumentTyped));
        v.register_early_pass(Box::new(super::lint_argument::ReadableArgumentsName));
        v.register_early_pass(Box::new(super::lint_unless::DoNotUseUnless));
        v.register_early_pass(Box::new(super::lint_term::UselessParens));
        v
    }

    pub fn early_pass(&self) -> &[Box<dyn EarlyLintPass>] {
        &self.early_pass
    }
}

pub struct AstLinter;

impl AstLinter {
    pub fn check_term(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::Term<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_term(elt));
        }

        errors
    }

    pub fn check_expression(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::Expression<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_expression(elt));
        }

        use puppet_lang::expression::ExpressionVariant;
        match &elt.value {
            ExpressionVariant::Term(elt) => errors.append(&mut self.check_term(storage, elt)),
            ExpressionVariant::Multiply((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Divide((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Modulo((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Plus((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Minus((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::ShiftLeft((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::ShiftRight((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Equal((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::NotEqual((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Gt((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::GtEq((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Lt((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::LtEq((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::And((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Or((left, right)) => {
                errors.append(&mut self.check_expression(storage, left));
                errors.append(&mut self.check_expression(storage, right));
            }
            ExpressionVariant::Not(expr) => {
                // TODO linters for negation
                errors.append(&mut self.check_expression(storage, expr));
            }
            ExpressionVariant::Selector(_)
            | ExpressionVariant::Assign(_)
            | ExpressionVariant::MatchRegex(_)
            | ExpressionVariant::NotMatchRegex(_)
            | ExpressionVariant::MatchType(_)
            | ExpressionVariant::NotMatchType(_)
            | ExpressionVariant::In(_)
            | ExpressionVariant::ChainCall(_) => {
                // TODO
            }
        };

        errors
    }

    pub fn check_unless(
        &self,
        storage: &Storage,
        elt: &puppet_lang::statement::ConditionAndStatement<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_unless(elt));
        }

        errors.append(&mut self.check_expression(storage, &elt.condition));

        errors
    }

    pub fn check_statement(
        &self,
        storage: &Storage,
        statement: &puppet_lang::statement::Statement<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_statement(statement));
        }

        use puppet_lang::statement::StatementVariant;
        let mut variant_errors = match &statement.value {
            StatementVariant::Unless(elt) => self.check_unless(storage, elt),
            StatementVariant::Toplevel(elt) => self.check_toplevel(storage, elt),
            StatementVariant::Expression(elt) => self.check_expression(storage, elt),
            StatementVariant::Include(_)
            | StatementVariant::Require(_)
            | StatementVariant::Contain(_)
            | StatementVariant::Realize(_)
            | StatementVariant::CreateResources(_)
            | StatementVariant::Tag(_)
            | StatementVariant::RelationList(_)
            | StatementVariant::IfElse(_)
            | StatementVariant::Case(_) => {
                // TODO
                vec![]
            }
        };

        errors.append(&mut variant_errors);

        errors
    }

    pub fn check_toplevel_variant(
        &self,
        storage: &Storage,
        arguments: &[puppet_lang::argument::Argument<Location>],
        body: &[puppet_lang::statement::Statement<Location>],
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            for arg in arguments {
                errors.append(&mut lint.check_argument(arg));
            }
        }

        for statement in body {
            errors.append(&mut self.check_statement(storage, statement));
        }

        errors
    }

    pub fn check_class(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::Class<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_class(elt));
        }
        errors.append(&mut self.check_toplevel_variant(storage, &elt.arguments, &elt.body));

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
        }

        errors.append(&mut self.check_toplevel_variant(storage, &elt.arguments, &elt.body));

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
        }

        errors.append(&mut self.check_toplevel_variant(storage, &elt.arguments, &elt.body));

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
