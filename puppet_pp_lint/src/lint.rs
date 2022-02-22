use puppet_parser::parser::Location;

pub trait LintPass {
    fn name(&self) -> &str;
}

// #[derive(Clone)]
pub struct LintError {
    pub linter: Box<dyn LintPass>,
    pub message: String,
    pub url: Option<String>,
    pub location: Location,
}

impl LintError {
    pub fn new(linter: Box<dyn LintPass>, message: &str, location: &Location) -> Self {
        Self {
            linter,
            message: message.to_owned(),
            url: None,
            location: location.clone(),
        }
    }
    pub fn new_with_url(
        linter: Box<dyn LintPass>,
        message: &str,
        url: &str,
        location: &Location,
    ) -> Self {
        Self {
            linter,
            message: message.to_owned(),
            url: Some(url.to_owned()),
            location: location.clone(),
        }
    }
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
    fn check_typedef(&self, _: &puppet_lang::toplevel::TypeDef<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_functiondef(
        &self,
        _: &puppet_lang::toplevel::FunctionDef<Location>,
    ) -> Vec<LintError> {
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
    fn check_if_else(&self, _: &puppet_lang::statement::IfElse<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_expression(
        &self,
        _is_toplevel_expr: bool,
        _: &puppet_lang::expression::Expression<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_term(&self, _: &puppet_lang::expression::Term<Location>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_string_expression(
        &self,
        _: &puppet_lang::string::StringExpr<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation_list(
        &self,
        _: &puppet_lang::statement::RelationList<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation(
        &self,
        _: &puppet_lang::statement::RelationElt<Location>,
        _: &puppet_lang::statement::Relation<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation_elt(
        &self,
        _: &puppet_lang::statement::RelationElt<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_resource_set(
        &self,
        _: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_resource_collection(
        &self,
        _: &puppet_lang::resource_collection::ResourceCollection<Location>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_case_statement(&self, _: &puppet_lang::statement::Case<Location>) -> Vec<LintError> {
        Vec::new()
    }
}

#[derive(Default)]
pub struct Storage {
    early_pass: Vec<Box<dyn EarlyLintPass>>,
}

impl Storage {
    pub fn register_early_pass(&mut self, lint: Box<dyn EarlyLintPass>) {
        if self
            .early_pass
            .iter()
            .any(|registered| registered.name() == lint.name())
        {
            panic!("Lint {:?} already registered", lint.name())
        }
        self.early_pass.push(lint)
    }

    pub fn new() -> Self {
        let mut v = Self::default();

        v.register_early_pass(Box::new(super::lint_toplevel::OptionalArgumentsGoesFirst));
        v.register_early_pass(Box::new(super::lint_toplevel::UniqueArgumentsNames));
        v.register_early_pass(Box::new(super::lint_argument::ArgumentLooksSensitive));
        v.register_early_pass(Box::new(super::lint_argument::SensitiveArgumentWithDefault));
        v.register_early_pass(Box::new(super::lint_argument::ArgumentTyped));
        v.register_early_pass(Box::new(super::lint_argument::ReadableArgumentsName));
        v.register_early_pass(Box::new(super::lint_argument::LowerCaseArgumentName));
        v.register_early_pass(Box::new(super::lint_unless::DoNotUseUnless));
        v.register_early_pass(Box::new(super::lint_expression::UselessParens));
        v.register_early_pass(Box::new(
            super::lint_expression::AssignmentToInvalidExpression,
        ));
        v.register_early_pass(Box::new(super::lint_string_expr::UselessDoubleQuotes));
        v.register_early_pass(Box::new(super::lint_term::LowerCaseVariable));
        v.register_early_pass(Box::new(super::lint_resource_set::UpperCaseName));
        v.register_early_pass(Box::new(super::lint_resource_set::UniqueAttributeName));
        v.register_early_pass(Box::new(
            super::lint_resource_set::FileModeAttributeIsString,
        ));
        v.register_early_pass(Box::new(
            super::lint_resource_set::EnsureAttributeIsNotTheFirst,
        ));
        v.register_early_pass(Box::new(
            super::lint_resource_set::MultipleResourcesWithoutDefault,
        ));
        v.register_early_pass(Box::new(super::lint_resource_set::SelectorInAttributeValue));
        v.register_early_pass(Box::new(super::lint_case_statement::EmptyCasesList));
        v.register_early_pass(Box::new(super::lint_case_statement::DefaultCaseIsNotLast));
        v.register_early_pass(Box::new(super::lint_case_statement::MultipleDefaultCase));
        v.register_early_pass(Box::new(super::lint_case_statement::NoDefaultCase));
        v.register_early_pass(Box::new(super::lint_statement::StatementWithNoSideEffects));
        v.register_early_pass(Box::new(super::lint_statement::RelationToTheLeft));
        v.register_early_pass(Box::new(super::lint_string_expr::InvalidStringEscape));
        v
    }

    pub fn early_pass(&self) -> &[Box<dyn EarlyLintPass>] {
        &self.early_pass
    }
}

pub struct AstLinter;

impl AstLinter {
    pub fn check_string_expression(
        &self,
        storage: &Storage,
        elt: &puppet_lang::string::StringExpr<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_string_expression(elt));
        }

        if let puppet_lang::string::StringVariant::DoubleQuoted(s) = &elt.data {
            for fragment in s {
                match fragment {
                    puppet_lang::string::DoubleQuotedFragment::StringFragment(_) => {}
                    puppet_lang::string::DoubleQuotedFragment::Expression(elt) => {
                        errors.append(&mut self.check_expression(storage, true, elt))
                    }
                }
            }
        }

        errors
    }

    pub fn check_array(
        &self,
        storage: &Storage,
        elts: &[puppet_lang::expression::Expression<Location>],
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for expr in elts {
            errors.append(&mut self.check_expression(storage, true, expr))
        }

        errors
    }

    pub fn check_accessor(
        &self,
        storage: &Storage,
        list: &[Vec<Box<puppet_lang::expression::Expression<Location>>>],
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for l1 in list {
            for l2 in l1 {
                errors.append(&mut self.check_expression(storage, true, l2));
            }
        }

        errors
    }

    pub fn check_map(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::Map<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for (left, right) in &elt.value {
            errors.append(&mut self.check_expression(storage, true, left));
            errors.append(&mut self.check_expression(storage, true, right));
        }
        errors.append(&mut self.check_accessor(storage, &elt.accessor));

        errors
    }

    pub fn check_variable(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::Variable<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        errors.append(&mut self.check_accessor(storage, &elt.accessor));

        errors
    }

    pub fn check_term(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::Term<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_term(elt));
        }

        match &elt.value {
            puppet_lang::expression::TermVariant::String(elt) => {
                errors.append(&mut self.check_string_expression(storage, elt));
                errors.append(&mut self.check_accessor(storage, &elt.accessor));
            }
            puppet_lang::expression::TermVariant::Parens(expr) => {
                errors.append(&mut self.check_expression(storage, false, &expr.value))
            }
            puppet_lang::expression::TermVariant::Array(list) => {
                errors.append(&mut self.check_array(storage, list))
            }
            puppet_lang::expression::TermVariant::Map(elt) => {
                errors.append(&mut self.check_map(storage, elt))
            }
            puppet_lang::expression::TermVariant::Variable(elt) => {
                errors.append(&mut self.check_variable(storage, elt))
            }
            puppet_lang::expression::TermVariant::Float(_)
            | puppet_lang::expression::TermVariant::Integer(_)
            | puppet_lang::expression::TermVariant::Boolean(_)
            | puppet_lang::expression::TermVariant::Undef(_)
            | puppet_lang::expression::TermVariant::RegexpGroupID(_)
            | puppet_lang::expression::TermVariant::Sensitive(_)
            | puppet_lang::expression::TermVariant::TypeSpecitifaction(_)
            | puppet_lang::expression::TermVariant::Identifier(_)
            | puppet_lang::expression::TermVariant::Regexp(_) => {
                // TODO
            }
        }

        errors
    }

    pub fn check_funcall(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::FunctionCall<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        for arg in &elt.args {
            errors.append(&mut self.check_expression(storage, true, arg));
        }
        if let Some(lambda) = &elt.lambda {
            errors.append(&mut self.check_lambda(storage, lambda))
        }
        errors.append(&mut self.check_accessor(storage, &elt.accessor));

        errors
    }

    pub fn check_expression(
        &self,
        storage: &Storage,
        is_toplevel_expr: bool,
        elt: &puppet_lang::expression::Expression<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_expression(is_toplevel_expr, elt));
        }

        use puppet_lang::expression::ExpressionVariant;
        match &elt.value {
            ExpressionVariant::Term(elt) => errors.append(&mut self.check_term(storage, elt)),
            ExpressionVariant::Multiply((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Divide((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Modulo((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Plus((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Minus((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::ShiftLeft((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::ShiftRight((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Equal((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::NotEqual((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Gt((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::GtEq((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Lt((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::LtEq((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::And((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Or((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Not(expr) => {
                errors.append(&mut self.check_expression(storage, false, expr));
            }
            ExpressionVariant::Assign((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::Selector(elt) => {
                errors.append(&mut self.check_expression(storage, false, &elt.condition));
                for case in &elt.cases {
                    match &case.case {
                        puppet_lang::expression::CaseVariant::Term(term) => {
                            errors.append(&mut self.check_term(storage, term))
                        }
                        puppet_lang::expression::CaseVariant::Default(_) => {}
                    }
                    errors.append(&mut self.check_expression(storage, false, &case.body));
                }
            }
            ExpressionVariant::FunctionCall(elt) => {
                errors.append(&mut self.check_funcall(storage, elt))
            }
            ExpressionVariant::MatchRegex((left, _))
            | ExpressionVariant::NotMatchRegex((left, _)) => {
                errors.append(&mut self.check_expression(storage, false, left));
            }
            ExpressionVariant::In((left, right)) => {
                errors.append(&mut self.check_expression(storage, false, left));
                errors.append(&mut self.check_expression(storage, false, right));
            }
            ExpressionVariant::ChainCall(elt) => {
                errors.append(&mut self.check_expression(storage, false, &elt.left));
                errors.append(&mut self.check_funcall(storage, &elt.right));
            }
            ExpressionVariant::MatchType(_) | ExpressionVariant::NotMatchType(_) => {
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

        errors.append(&mut self.check_expression(storage, true, &elt.condition));
        for statement in elt.body.as_ref() {
            errors.append(&mut self.check_statement(storage, statement));
        }

        errors
    }

    pub fn check_if_else(
        &self,
        storage: &Storage,
        elt: &puppet_lang::statement::IfElse<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_if_else(elt));
        }

        errors.append(&mut self.check_expression(storage, true, &elt.condition.condition));
        for statement in elt.condition.body.as_ref() {
            errors.append(&mut self.check_statement(storage, statement));
        }

        for elsif_block in &elt.elsif_list {
            errors.append(&mut self.check_expression(storage, true, &elsif_block.condition));
            for statement in elsif_block.body.as_ref() {
                errors.append(&mut self.check_statement(storage, statement));
            }
        }

        if let Some(else_block) = &elt.else_block {
            for statement in else_block.as_ref() {
                errors.append(&mut self.check_statement(storage, statement));
            }
        }

        errors
    }

    pub fn check_resource_set(
        &self,
        storage: &Storage,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_resource_set(elt));
        }

        for resource in &elt.list {
            errors.append(&mut self.check_expression(storage, true, &resource.title));
            for attribute in &resource.attributes {
                match attribute {
                    puppet_lang::statement::ResourceAttribute::Name((name, value)) => {
                        errors.append(&mut self.check_string_expression(storage, name));
                        errors.append(&mut self.check_expression(storage, true, value))
                    }
                    puppet_lang::statement::ResourceAttribute::Group(term) => {
                        errors.append(&mut self.check_term(storage, term))
                    }
                }
            }
        }

        errors
    }

    pub fn check_resource_collection(
        &self,
        storage: &Storage,
        elt: &puppet_lang::resource_collection::ResourceCollection<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_resource_collection(elt));
        }

        errors
    }

    pub fn check_relation_elt(
        &self,
        storage: &Storage,
        elt: &puppet_lang::statement::RelationElt<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_relation_elt(elt));
        }

        match elt {
            puppet_lang::statement::RelationElt::ResourceSet(elt) => {
                errors.append(&mut self.check_resource_set(storage, elt))
            }
            puppet_lang::statement::RelationElt::ResourceCollection(list) => {
                for elt in list {
                    errors.append(&mut self.check_resource_collection(storage, elt))
                }
            }
        }

        errors
    }

    pub fn check_relation(
        &self,
        storage: &Storage,
        prev: &puppet_lang::statement::RelationElt<Location>,
        elt: &puppet_lang::statement::Relation<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_relation(prev, elt));
        }

        errors.append(&mut self.check_relation_list(storage, elt.relation_to.as_ref()));

        errors
    }

    pub fn check_relation_list(
        &self,
        storage: &Storage,
        elt: &puppet_lang::statement::RelationList<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_relation_list(elt));
        }

        errors.append(&mut self.check_relation_elt(storage, &elt.head));

        if let Some(tail) = &elt.tail {
            errors.append(&mut self.check_relation(storage, &elt.head, tail));
        }

        errors
    }

    pub fn check_case(
        &self,
        storage: &Storage,
        elt: &puppet_lang::statement::Case<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_case_statement(elt));
        }

        errors.append(&mut self.check_expression(storage, true, &elt.condition));

        for case in &elt.elements {
            for statement in case.body.as_ref() {
                errors.append(&mut self.check_statement(storage, statement));
            }
        }

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
            StatementVariant::Expression(elt) => self.check_expression(storage, true, elt),
            StatementVariant::IfElse(elt) => self.check_if_else(storage, elt),
            StatementVariant::RelationList(elt) => self.check_relation_list(storage, elt),
            StatementVariant::Case(elt) => self.check_case(storage, elt),
            StatementVariant::Include(_)
            | StatementVariant::Require(_)
            | StatementVariant::Fail(_)
            | StatementVariant::Contain(_)
            | StatementVariant::Realize(_)
            | StatementVariant::CreateResources(_)
            | StatementVariant::Tag(_) => {
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
        for arg in arguments {
            errors.append(&mut self.check_argument(storage, arg))
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

    pub fn check_argument(
        &self,
        storage: &Storage,
        elt: &puppet_lang::argument::Argument<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_argument(elt));
        }
        if let Some(default) = &elt.default {
            errors.append(&mut self.check_expression(storage, true, default));
        }
        errors
    }

    pub fn check_lambda(
        &self,
        storage: &Storage,
        elt: &puppet_lang::expression::Lambda<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for arg in &elt.args {
            errors.append(&mut self.check_argument(storage, arg))
        }
        for statement in &elt.body {
            errors.append(&mut self.check_statement(storage, statement));
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

    pub fn check_typedef(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::TypeDef<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_typedef(elt));
        }

        errors
    }

    pub fn check_functiondef(
        &self,
        storage: &Storage,
        elt: &puppet_lang::toplevel::FunctionDef<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_functiondef(elt));
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
            puppet_lang::toplevel::Toplevel::TypeDef(elt) => self.check_typedef(storage, elt),
            puppet_lang::toplevel::Toplevel::FunctionDef(elt) => {
                self.check_functiondef(storage, elt)
            }
        };
        errors.append(&mut res);

        errors
    }
}
