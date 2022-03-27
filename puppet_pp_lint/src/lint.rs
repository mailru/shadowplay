use puppet_parser::range::Range;

pub trait LintPass {
    fn name(&self) -> &str;
}

// #[derive(Clone)]
pub struct LintError {
    pub linter: Box<dyn LintPass>,
    pub message: String,
    pub url: Option<String>,
    pub location: Range,
}

impl LintError {
    pub fn new(linter: Box<dyn LintPass>, message: &str, location: &Range) -> Self {
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
        location: &Range,
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
    fn check_toplevel(&self, _: &puppet_lang::toplevel::Toplevel<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_class(&self, _: &puppet_lang::toplevel::Class<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_definition(&self, _: &puppet_lang::toplevel::Definition<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_plan(&self, _: &puppet_lang::toplevel::Plan<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_typedef(&self, _: &puppet_lang::toplevel::TypeDef<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_functiondef(&self, _: &puppet_lang::toplevel::FunctionDef<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_argument(&self, _: &puppet_lang::argument::Argument<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_statement(&self, _: &puppet_lang::statement::Statement<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_statement_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        _: &[puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_unless(
        &self,
        _: &puppet_lang::statement::ConditionAndStatement<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_if_else(&self, _: &puppet_lang::statement::IfElse<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_expression(
        &self,
        _ctx: &crate::ctx::Ctx,
        _is_toplevel_expr: bool,
        _: &puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_term(
        &self,
        _ctx: &crate::ctx::Ctx,
        _is_assignment: bool,
        _: &puppet_lang::expression::Term<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_string_expression(
        &self,
        _: &puppet_lang::string::StringExpr<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation_list(
        &self,
        _: &puppet_lang::statement::RelationList<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation(
        &self,
        _: &puppet_lang::statement::RelationElt<Range>,
        _: &puppet_lang::statement::Relation<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation_elt(&self, _: &puppet_lang::statement::RelationElt<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        _: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_resource_collection(
        &self,
        _ctx: &crate::ctx::Ctx,
        _: &puppet_lang::resource_collection::ResourceCollection<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_case_statement(&self, _: &puppet_lang::statement::Case<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_deprecated_resource_defaults(
        &self,
        _: &puppet_lang::statement::ResourceDefaults<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_condition_expression(
        &self,
        _: &puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
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
        v.register_early_pass(Box::new(super::lint_expression::InvalidVariableAssignment));
        v.register_early_pass(Box::new(super::lint_expression::DoubleNegation));
        v.register_early_pass(Box::new(super::lint_expression::NegationOfEquation));
        v.register_early_pass(Box::new(
            super::lint_expression::ConstantExpressionInCondition,
        ));
        v.register_early_pass(Box::new(
            super::lint_builtin::ErbReferencesToUnknownVariable,
        ));
        v.register_early_pass(Box::new(super::lint_string_expr::UselessDoubleQuotes));
        v.register_early_pass(Box::new(super::lint_string_expr::ExpressionInSingleQuotes));
        v.register_early_pass(Box::new(super::lint_term::LowerCaseVariable));
        v.register_early_pass(Box::new(super::lint_term::ReferenceToUndefinedValue));
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
        v.register_early_pass(Box::new(
            super::lint_resource_set::PerExpressionResourceDefaults,
        ));
        v.register_early_pass(Box::new(super::lint_resource_set::ExecAttributes));
        v.register_early_pass(Box::new(super::lint_resource_set::SelectorInAttributeValue));
        v.register_early_pass(Box::new(super::lint_resource_set::UnconditionalExec));
        v.register_early_pass(Box::new(
            super::lint_resource_set::InvalidResourceSetInvocation,
        ));
        v.register_early_pass(Box::new(
            super::lint_resource_set::InvalidResourceCollectionInvocation,
        ));
        v.register_early_pass(Box::new(super::lint_case_statement::EmptyCasesList));
        v.register_early_pass(Box::new(super::lint_case_statement::DefaultCaseIsNotLast));
        v.register_early_pass(Box::new(super::lint_case_statement::MultipleDefaultCase));
        v.register_early_pass(Box::new(super::lint_case_statement::NoDefaultCase));
        v.register_early_pass(Box::new(super::lint_statement::StatementWithNoEffect));
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
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::string::StringExpr<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_string_expression(elt));
        }

        if let puppet_lang::string::StringVariant::DoubleQuoted(s) = &elt.data {
            for fragment in s {
                match fragment {
                    puppet_lang::string::DoubleQuotedFragment::StringFragment(_) => {}
                    puppet_lang::string::DoubleQuotedFragment::Expression(elt) => errors
                        .append(&mut self.check_expression(storage, ctx, true, false, &elt.data)),
                }
            }
        }

        errors
    }

    pub fn check_array(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        is_assignment: bool,
        array: &puppet_lang::expression::Array<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for expr in &array.value.value {
            errors.append(&mut self.check_expression(storage, ctx, true, is_assignment, expr))
        }

        errors
    }

    pub fn check_accessor(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        accessor: &Option<puppet_lang::expression::Accessor<Range>>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        let accessor = match accessor {
            Some(v) => v,
            None => return errors,
        };

        for l1 in &accessor.list {
            for l2 in l1 {
                errors.append(&mut self.check_expression(storage, ctx, true, false, l2));
            }
        }

        errors
    }

    pub fn check_map(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::expression::Map<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for kv in &elt.value.value {
            errors.append(&mut self.check_expression(storage, ctx, true, false, &kv.key));
            errors.append(&mut self.check_expression(storage, ctx, true, false, &kv.value));
        }

        errors
    }

    pub fn check_variable(
        &self,
        _storage: &Storage,
        _ctx: &crate::ctx::Ctx,
        _elt: &puppet_lang::expression::Variable<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }

    pub fn check_type_specification(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::typing::TypeSpecification<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        match &elt.data {
            puppet_lang::typing::TypeSpecificationVariant::ExternalType(elt) => {
                for arg in &elt.arguments {
                    errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Enum(list) => {
                for elt in &list.list {
                    errors.append(&mut self.check_term(storage, ctx, false, elt));
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Array(elt) => {
                if let Some(inner) = &elt.inner {
                    errors.append(&mut self.check_type_specification(storage, ctx, inner))
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Hash(elt) => {
                if let Some(key) = &elt.key {
                    errors.append(&mut self.check_type_specification(storage, ctx, key))
                }
                if let Some(value) = &elt.value {
                    errors.append(&mut self.check_type_specification(storage, ctx, value))
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Optional(elt) => match &elt.value {
                puppet_lang::typing::TypeOptionalVariant::TypeSpecification(elt) => {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt))
                }
                puppet_lang::typing::TypeOptionalVariant::Term(elt) => {
                    errors.append(&mut self.check_term(storage, ctx, false, elt));
                }
            },
            puppet_lang::typing::TypeSpecificationVariant::Variant(elt) => {
                for elt in &elt.list {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt));
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Struct(elt) => {
                for elt in &elt.keys.value {
                    match &elt.key {
                        puppet_lang::typing::TypeStructKey::String(elt) => {
                            errors.append(&mut self.check_string_expression(storage, ctx, elt));
                        }
                        puppet_lang::typing::TypeStructKey::Optional(elt) => {
                            errors.append(
                                &mut self.check_string_expression(storage, ctx, &elt.value),
                            );
                        }
                        puppet_lang::typing::TypeStructKey::NotUndef(elt) => {
                            errors.append(
                                &mut self.check_string_expression(storage, ctx, &elt.value),
                            );
                        }
                    }
                    errors.append(&mut self.check_type_specification(storage, ctx, &elt.value));
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Sensitive(elt) => match &elt.value {
                puppet_lang::typing::TypeSensitiveVariant::TypeSpecification(elt) => {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt));
                }
                puppet_lang::typing::TypeSensitiveVariant::Term(elt) => {
                    errors.append(&mut self.check_term(storage, ctx, false, elt));
                }
            },
            puppet_lang::typing::TypeSpecificationVariant::Tuple(elt) => {
                for elt in &elt.list {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt))
                }
            }
            puppet_lang::typing::TypeSpecificationVariant::Float(_)
            | puppet_lang::typing::TypeSpecificationVariant::Integer(_)
            | puppet_lang::typing::TypeSpecificationVariant::Numeric(_)
            | puppet_lang::typing::TypeSpecificationVariant::String(_)
            | puppet_lang::typing::TypeSpecificationVariant::Pattern(_)
            | puppet_lang::typing::TypeSpecificationVariant::Regex(_)
            | puppet_lang::typing::TypeSpecificationVariant::Boolean(_)
            | puppet_lang::typing::TypeSpecificationVariant::Undef(_)
            | puppet_lang::typing::TypeSpecificationVariant::Any(_) => { // TODO
            }
        }

        errors
    }

    pub fn check_term(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        is_assignment: bool,
        elt: &puppet_lang::expression::Term<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_term(ctx, is_assignment, elt));
        }

        match &elt.value {
            puppet_lang::expression::TermVariant::String(elt) => {
                errors.append(&mut self.check_string_expression(storage, ctx, elt));
            }
            puppet_lang::expression::TermVariant::Parens(expr) => errors.append(
                &mut self.check_expression(storage, ctx, false, is_assignment, &expr.value),
            ),
            puppet_lang::expression::TermVariant::Array(list) => {
                errors.append(&mut self.check_array(storage, ctx, is_assignment, list))
            }
            puppet_lang::expression::TermVariant::Map(elt) => {
                errors.append(&mut self.check_map(storage, ctx, elt))
            }
            puppet_lang::expression::TermVariant::Variable(elt) => {
                errors.append(&mut self.check_variable(storage, ctx, elt))
            }
            puppet_lang::expression::TermVariant::TypeSpecitifaction(elt) => {
                errors.append(&mut self.check_type_specification(storage, ctx, elt))
            }
            puppet_lang::expression::TermVariant::Float(_)
            | puppet_lang::expression::TermVariant::Integer(_)
            | puppet_lang::expression::TermVariant::Boolean(_)
            | puppet_lang::expression::TermVariant::RegexpGroupID(_)
            | puppet_lang::expression::TermVariant::Sensitive(_)
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
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::expression::FunctionCall<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        for arg in &elt.args {
            errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
        }
        if let Some(lambda) = &elt.lambda {
            errors.append(&mut self.check_lambda(storage, ctx, lambda))
        }

        errors
    }

    pub fn check_builtin(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::builtin::BuiltinVariant<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        match elt {
            puppet_lang::builtin::BuiltinVariant::Undef => {}
            puppet_lang::builtin::BuiltinVariant::Tag(v)
            | puppet_lang::builtin::BuiltinVariant::Require(v)
            | puppet_lang::builtin::BuiltinVariant::Realize(v)
            | puppet_lang::builtin::BuiltinVariant::CreateResources(v)
            | puppet_lang::builtin::BuiltinVariant::Include(v) => {
                for arg in &v.args {
                    errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
                }
                if let Some(lambda) = &v.lambda {
                    errors.append(&mut self.check_lambda(storage, ctx, lambda))
                }
            }
            puppet_lang::builtin::BuiltinVariant::Return(arg) => {
                if let Some(arg) = arg.as_ref() {
                    errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
                }
            }
            puppet_lang::builtin::BuiltinVariant::Template(arg) => {
                errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
            }
        }

        errors
    }

    fn register_assignments(
        &self,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::expression::Expression<Range>,
    ) {
        use puppet_lang::expression::ExpressionVariant;
        if let ExpressionVariant::Term(term) = &elt.value {
            match &term.value {
                puppet_lang::expression::TermVariant::Array(list) => {
                    for elt in &list.value.value {
                        self.register_assignments(ctx, elt)
                    }
                }
                puppet_lang::expression::TermVariant::Parens(elt) => {
                    self.register_assignments(ctx, &elt.value)
                }
                puppet_lang::expression::TermVariant::Variable(variable) => {
                    ctx.register_defined_variable(variable)
                }
                _ => (),
            }
        }
    }

    pub fn check_expression(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        is_toplevel_expr: bool,
        is_assignment: bool,
        elt: &puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_expression(ctx, is_toplevel_expr, elt));
        }

        use puppet_lang::expression::ExpressionVariant;
        match &elt.value {
            ExpressionVariant::Term(elt) => {
                errors.append(&mut self.check_term(storage, ctx, is_assignment, elt))
            }
            ExpressionVariant::Multiply((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Divide((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Modulo((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Plus((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Minus((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::ShiftLeft((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::ShiftRight((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Equal((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::NotEqual((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Gt((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::GtEq((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Lt((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::LtEq((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::And((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Or((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::Not(expr) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, expr));
            }
            ExpressionVariant::Assign((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, true, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
                self.register_assignments(ctx, left);
            }
            ExpressionVariant::Selector(elt) => {
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    false,
                    &elt.condition,
                ));
                errors.append(&mut self.check_condition_expression(storage, ctx, &elt.condition));
                for case in &elt.cases.value {
                    match &case.case {
                        puppet_lang::expression::CaseVariant::Term(term) => {
                            errors.append(&mut self.check_term(storage, ctx, false, term))
                        }
                        puppet_lang::expression::CaseVariant::Default(_) => {}
                    }
                    errors.append(&mut self.check_expression(
                        storage,
                        ctx,
                        false,
                        is_assignment,
                        &case.body,
                    ));
                }
            }
            ExpressionVariant::FunctionCall(elt) => {
                errors.append(&mut self.check_funcall(storage, ctx, elt))
            }
            ExpressionVariant::BuiltinFunction(elt) => {
                errors.append(&mut self.check_builtin(storage, ctx, elt));
            }
            ExpressionVariant::MatchRegex((left, _))
            | ExpressionVariant::NotMatchRegex((left, _)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
            }
            ExpressionVariant::In((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    right,
                ));
            }
            ExpressionVariant::ChainCall(elt) => {
                errors.append(&mut self.check_expression(
                    storage,
                    ctx,
                    false,
                    is_assignment,
                    &elt.left,
                ));
                errors.append(&mut self.check_funcall(storage, ctx, &elt.right));
            }
            ExpressionVariant::MatchType((left, right))
            | ExpressionVariant::NotMatchType((left, right)) => {
                errors.append(&mut self.check_expression(storage, ctx, false, is_assignment, left));
                errors.append(&mut self.check_type_specification(storage, ctx, right));
            }
        };
        errors.append(&mut self.check_accessor(storage, ctx, &elt.accessor));

        errors
    }

    pub fn check_unless(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ConditionAndStatement<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_unless(elt));
            errors.append(&mut lint.check_condition_expression(&elt.condition));
        }

        errors.append(&mut self.check_expression(storage, ctx, true, false, &elt.condition));

        errors.append(&mut self.check_statement_set(storage, ctx, elt.body.value.as_ref()));

        errors
    }

    pub fn check_condition_expression(
        &self,
        storage: &Storage,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_condition_expression(elt));
        }

        errors
    }

    pub fn check_if_else(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::IfElse<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_if_else(elt));
        }
        errors.append(&mut self.check_condition_expression(storage, ctx, &elt.condition.condition));

        errors.append(&mut self.check_expression(
            storage,
            ctx,
            true,
            false,
            &elt.condition.condition,
        ));

        errors.append(&mut self.check_statement_set(
            storage,
            ctx,
            elt.condition.body.value.as_ref(),
        ));

        for elsif_block in &elt.elsif_list {
            errors.append(&mut self.check_expression(
                storage,
                ctx,
                true,
                false,
                &elsif_block.condition,
            ));
            errors.append(&mut self.check_condition_expression(
                storage,
                ctx,
                &elsif_block.condition,
            ));
            errors.append(&mut self.check_statement_set(
                storage,
                ctx,
                elsif_block.body.value.as_ref(),
            ));
        }

        if let Some(else_block) = &elt.else_block {
            errors.append(&mut self.check_statement_set(storage, ctx, else_block.value.as_ref()));
        }

        errors
    }

    pub fn check_resource_set(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_resource_set(ctx, elt));
        }

        for resource in &elt.list.value {
            errors.append(&mut self.check_expression(storage, ctx, true, false, &resource.title));
            ctx.register_phantom_variable("title");
            for attribute in &resource.attributes.value {
                match &attribute.value {
                    puppet_lang::statement::ResourceAttributeVariant::Name((name, value)) => {
                        errors.append(&mut self.check_string_expression(storage, ctx, name));
                        errors.append(&mut self.check_expression(storage, ctx, true, false, value))
                    }
                    puppet_lang::statement::ResourceAttributeVariant::Group(term) => {
                        errors.append(&mut self.check_term(storage, ctx, false, term))
                    }
                }
            }
        }

        errors
    }

    pub fn check_resource_collection(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::resource_collection::ResourceCollection<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_resource_collection(ctx, elt));
        }

        errors.append(&mut self.check_type_specification(storage, ctx, &elt.type_specification));

        errors
    }

    pub fn check_relation_elt(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::RelationElt<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_relation_elt(elt));
        }

        for elt in &elt.data.value {
            match elt {
                puppet_lang::statement::RelationEltVariant::ResourceSet(elt) => {
                    errors.append(&mut self.check_resource_set(storage, ctx, elt))
                }
                puppet_lang::statement::RelationEltVariant::ResourceCollection(elt) => {
                    errors.append(&mut self.check_resource_collection(storage, ctx, elt))
                }
            }
        }

        errors
    }

    pub fn check_relation(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        prev: &puppet_lang::statement::RelationElt<Range>,
        elt: &puppet_lang::statement::Relation<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_relation(prev, elt));
        }

        errors.append(&mut self.check_relation_list(storage, ctx, elt.relation_to.as_ref()));

        errors
    }

    pub fn check_relation_list(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::RelationList<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_relation_list(elt));
        }

        errors.append(&mut self.check_relation_elt(storage, ctx, &elt.head));

        if let Some(tail) = &elt.tail {
            errors.append(&mut self.check_relation(storage, ctx, &elt.head, tail));
        }

        errors
    }

    pub fn check_case(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::Case<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_case_statement(elt));
        }
        errors.append(&mut self.check_condition_expression(storage, ctx, &elt.condition));
        errors.append(&mut self.check_expression(storage, ctx, true, false, &elt.condition));

        for case in &elt.elements.value {
            for matchcase in &case.matches {
                match matchcase {
                    puppet_lang::expression::CaseVariant::Term(term) => {
                        errors.append(&mut self.check_term(storage, ctx, false, term));
                    }
                    puppet_lang::expression::CaseVariant::Default(_) => (),
                }
            }

            errors.append(&mut self.check_statement_set(storage, ctx, &case.body.value));
        }

        errors
    }

    pub fn check_statement_set(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        list: &[puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for statement in list {
            errors.append(&mut self.check_statement(storage, ctx, statement));
        }
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_statement_set(ctx, list));
        }

        errors
    }

    pub fn check_deprecated_resource_defaults(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceDefaults<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_deprecated_resource_defaults(elt));
            for (k, v) in &elt.args.value {
                errors.append(&mut self.check_term(storage, ctx, false, k));
                errors.append(&mut self.check_expression(storage, ctx, true, false, v));
            }
        }

        errors
    }

    pub fn check_statement(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        statement: &puppet_lang::statement::Statement<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_statement(statement));
        }

        use puppet_lang::statement::StatementVariant;
        match &statement.value {
            StatementVariant::Unless(elt) => {
                errors.append(&mut self.check_unless(storage, ctx, elt))
            }
            StatementVariant::Toplevel(elt) => {
                errors.append(&mut self.check_toplevel(storage, ctx, elt))
            }
            StatementVariant::Expression(elt) => {
                errors.append(&mut self.check_expression(storage, ctx, true, false, elt))
            }
            StatementVariant::IfElse(elt) => {
                errors.append(&mut self.check_if_else(storage, ctx, elt))
            }
            StatementVariant::RelationList(elt) => {
                errors.append(&mut self.check_relation_list(storage, ctx, elt))
            }
            StatementVariant::Case(elt) => errors.append(&mut self.check_case(storage, ctx, elt)),
            StatementVariant::ResourceDefaults(elt) => {
                errors.append(&mut self.check_deprecated_resource_defaults(storage, ctx, elt))
            }
        };

        errors
    }

    pub fn check_toplevel_variant(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        arguments: &[puppet_lang::argument::Argument<Range>],
        body: &[puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        let ctx = ctx.new_scope();
        ctx.register_phantom_variable("name");
        ctx.register_phantom_variable("title");

        let mut errors = Vec::new();
        for arg in arguments {
            errors.append(&mut self.check_argument(storage, &ctx, arg));
        }

        errors.append(&mut self.check_statement_set(storage, &ctx, body));

        errors
    }

    pub fn check_class(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::toplevel::Class<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_class(elt));
        }
        errors.append(&mut self.check_toplevel_variant(
            storage,
            ctx,
            &elt.arguments.value,
            &elt.body.value,
        ));

        errors
    }

    pub fn check_argument(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::argument::Argument<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_argument(elt));
        }
        if let Some(type_spec) = &elt.type_spec {
            errors.append(&mut self.check_type_specification(storage, ctx, type_spec));
        }
        if let Some(default) = &elt.default {
            errors.append(&mut self.check_expression(storage, ctx, true, false, default));
        }

        ctx.register_argument_variable(elt);

        errors
    }

    pub fn check_lambda(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::expression::Lambda<Range>,
    ) -> Vec<LintError> {
        let ctx = ctx.clone();
        let mut errors = Vec::new();
        for arg in &elt.args.value {
            errors.append(&mut self.check_argument(storage, &ctx, arg))
        }
        errors.append(&mut self.check_statement_set(storage, &ctx, &elt.body.value));
        for statement in &elt.body.value {
            errors.append(&mut self.check_statement(storage, &ctx, statement));
        }

        errors
    }

    pub fn check_definition(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::toplevel::Definition<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_definition(elt));
        }

        errors.append(&mut self.check_toplevel_variant(
            storage,
            ctx,
            &elt.arguments.value,
            &elt.body.value,
        ));

        errors
    }

    pub fn check_plan(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::toplevel::Plan<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_plan(elt));
        }

        errors.append(&mut self.check_toplevel_variant(
            storage,
            ctx,
            &elt.arguments.value,
            &elt.body.value,
        ));

        errors
    }

    pub fn check_typedef(
        &self,
        storage: &Storage,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::toplevel::TypeDef<Range>,
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
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::toplevel::FunctionDef<Range>,
    ) -> Vec<LintError> {
        let ctx = ctx.new_scope();
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.check_functiondef(elt));
        }

        errors.append(&mut self.check_toplevel_variant(
            storage,
            &ctx,
            &elt.arguments.value,
            &elt.body.value,
        ));

        errors
    }

    pub fn check_toplevel(
        &self,
        storage: &Storage,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::toplevel::Toplevel<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        for lint in storage.early_pass() {
            errors.append(&mut lint.check_toplevel(elt))
        }
        let mut res = match &elt.data {
            puppet_lang::toplevel::ToplevelVariant::Class(elt) => {
                self.check_class(storage, ctx, elt)
            }
            puppet_lang::toplevel::ToplevelVariant::Definition(elt) => {
                self.check_definition(storage, ctx, elt)
            }
            puppet_lang::toplevel::ToplevelVariant::Plan(elt) => self.check_plan(storage, ctx, elt),
            puppet_lang::toplevel::ToplevelVariant::TypeDef(elt) => {
                self.check_typedef(storage, ctx, elt)
            }
            puppet_lang::toplevel::ToplevelVariant::FunctionDef(elt) => {
                self.check_functiondef(storage, ctx, elt)
            }
        };
        errors.append(&mut res);

        errors
    }
}
