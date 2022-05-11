use crate::puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

pub trait LintPass {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
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
    fn check_toplevel(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _: &crate::puppet_lang::toplevel::Toplevel<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_class(&self, _: &crate::puppet_lang::toplevel::Class<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_ctx(&self, _ctx: &crate::puppet_pp_lint::ctx::Ctx) -> Vec<LintError> {
        Vec::new()
    }
    fn check_definition(
        &self,
        _: &crate::puppet_lang::toplevel::Definition<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_plan(&self, _: &crate::puppet_lang::toplevel::Plan<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_typedef(&self, _: &crate::puppet_lang::toplevel::TypeDef<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_functiondef(
        &self,
        _: &crate::puppet_lang::toplevel::FunctionDef<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_argument(&self, _: &crate::puppet_lang::argument::Argument<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_statement(
        &self,
        _: &crate::puppet_lang::statement::Statement<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_statement_set(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _: &[crate::puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_unless(
        &self,
        _: &crate::puppet_lang::statement::ConditionAndStatement<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_if_else(&self, _: &crate::puppet_lang::statement::IfElse<Range>) -> Vec<LintError> {
        Vec::new()
    }
    fn check_expression(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _is_toplevel_expr: bool,
        _: &crate::puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_term(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _is_assignment: bool,
        _: &crate::puppet_lang::expression::Term<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_string_expression(
        &self,
        _: &crate::puppet_lang::string::StringExpr<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation_list(
        &self,
        _: &crate::puppet_lang::statement::RelationList<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation(
        &self,
        _: &crate::puppet_lang::statement::RelationElt<Range>,
        _: &crate::puppet_lang::statement::Relation<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_relation_elt(
        &self,
        _: &crate::puppet_lang::statement::RelationElt<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_resource_set(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _: &crate::puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_resource_collection(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _: &crate::puppet_lang::resource_collection::ResourceCollection<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_case_statement(
        &self,
        _: &crate::puppet_lang::statement::Case<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_deprecated_resource_defaults(
        &self,
        _: &crate::puppet_lang::statement::ResourceDefaults<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
    fn check_condition_expression(
        &self,
        _: &crate::puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub enum EarlyLintPassVariant {
    OptionalArgumentsGoesFirst(crate::puppet_pp_lint::lint_toplevel::OptionalArgumentsGoesFirst),
    UniqueArgumentsNames(crate::puppet_pp_lint::lint_toplevel::UniqueArgumentsNames),
    ArgumentLooksSensitive(crate::puppet_pp_lint::lint_argument::ArgumentLooksSensitive),
    SensitiveArgumentWithDefault(
        crate::puppet_pp_lint::lint_argument::SensitiveArgumentWithDefault,
    ),
    ArgumentTyped(crate::puppet_pp_lint::lint_argument::ArgumentTyped),
    ReadableArgumentsName(crate::puppet_pp_lint::lint_argument::ReadableArgumentsName),
    LowerCaseArgumentName(crate::puppet_pp_lint::lint_argument::LowerCaseArgumentName),
    DoNotUseUnless(crate::puppet_pp_lint::lint_unless::DoNotUseUnless),
    UselessParens(crate::puppet_pp_lint::lint_expression::UselessParens),
    InvalidVariableAssignment(crate::puppet_pp_lint::lint_expression::InvalidVariableAssignment),
    DoubleNegation(crate::puppet_pp_lint::lint_expression::DoubleNegation),
    NegationOfEquation(crate::puppet_pp_lint::lint_expression::NegationOfEquation),
    ConstantExpressionInCondition(
        crate::puppet_pp_lint::lint_expression::ConstantExpressionInCondition,
    ),
    ErbReferencesToUnknownVariable(
        crate::puppet_pp_lint::lint_builtin::ErbReferencesToUnknownVariable,
    ),
    UselessDoubleQuotes(crate::puppet_pp_lint::lint_string_expr::UselessDoubleQuotes),
    ExpressionInSingleQuotes(crate::puppet_pp_lint::lint_string_expr::ExpressionInSingleQuotes),
    LowerCaseVariable(crate::puppet_pp_lint::lint_term::LowerCaseVariable),
    ReferenceToUndefinedValue(crate::puppet_pp_lint::lint_term::ReferenceToUndefinedValue),
    UpperCaseName(crate::puppet_pp_lint::lint_resource_set::UpperCaseName),
    UniqueAttributeName(crate::puppet_pp_lint::lint_resource_set::UniqueAttributeName),
    FileModeAttributeIsString(crate::puppet_pp_lint::lint_resource_set::FileModeAttributeIsString),
    EnsureAttributeIsNotTheFirst(
        crate::puppet_pp_lint::lint_resource_set::EnsureAttributeIsNotTheFirst,
    ),
    MultipleResourcesWithoutDefault(
        crate::puppet_pp_lint::lint_resource_set::MultipleResourcesWithoutDefault,
    ),
    PerExpressionResourceDefaults(
        crate::puppet_pp_lint::lint_resource_set::PerExpressionResourceDefaults,
    ),
    ExecAttributes(crate::puppet_pp_lint::lint_resource_set::ExecAttributes),
    SelectorInAttributeValue(crate::puppet_pp_lint::lint_resource_set::SelectorInAttributeValue),
    UnconditionalExec(crate::puppet_pp_lint::lint_resource_set::UnconditionalExec),
    InvalidResourceSetInvocation(
        crate::puppet_pp_lint::lint_resource_set::InvalidResourceSetInvocation,
    ),
    InvalidResourceCollectionInvocation(
        crate::puppet_pp_lint::lint_resource_set::InvalidResourceCollectionInvocation,
    ),
    EmptyCasesList(crate::puppet_pp_lint::lint_case_statement::EmptyCasesList),
    DefaultCaseIsNotLast(crate::puppet_pp_lint::lint_case_statement::DefaultCaseIsNotLast),
    MultipleDefaultCase(crate::puppet_pp_lint::lint_case_statement::MultipleDefaultCase),
    NoDefaultCase(crate::puppet_pp_lint::lint_case_statement::NoDefaultCase),
    StatementWithNoEffect(crate::puppet_pp_lint::lint_statement::StatementWithNoEffect),
    RelationToTheLeft(crate::puppet_pp_lint::lint_statement::RelationToTheLeft),
    InvalidStringEscape(crate::puppet_pp_lint::lint_string_expr::InvalidStringEscape),
    UnusedVariables(crate::puppet_pp_lint::lint_ctx::UnusedVariables),
}

impl EarlyLintPassVariant {
    pub fn inner(&self) -> Box<&dyn EarlyLintPass> {
        match self {
            EarlyLintPassVariant::OptionalArgumentsGoesFirst(v) => Box::new(v),
            EarlyLintPassVariant::UniqueArgumentsNames(v) => Box::new(v),
            EarlyLintPassVariant::ArgumentLooksSensitive(v) => Box::new(v),
            EarlyLintPassVariant::SensitiveArgumentWithDefault(v) => Box::new(v),
            EarlyLintPassVariant::ArgumentTyped(v) => Box::new(v),
            EarlyLintPassVariant::ReadableArgumentsName(v) => Box::new(v),
            EarlyLintPassVariant::LowerCaseArgumentName(v) => Box::new(v),
            EarlyLintPassVariant::DoNotUseUnless(v) => Box::new(v),
            EarlyLintPassVariant::UselessParens(v) => Box::new(v),
            EarlyLintPassVariant::InvalidVariableAssignment(v) => Box::new(v),
            EarlyLintPassVariant::DoubleNegation(v) => Box::new(v),
            EarlyLintPassVariant::NegationOfEquation(v) => Box::new(v),
            EarlyLintPassVariant::ConstantExpressionInCondition(v) => Box::new(v),
            EarlyLintPassVariant::ErbReferencesToUnknownVariable(v) => Box::new(v),
            EarlyLintPassVariant::UselessDoubleQuotes(v) => Box::new(v),
            EarlyLintPassVariant::ExpressionInSingleQuotes(v) => Box::new(v),
            EarlyLintPassVariant::LowerCaseVariable(v) => Box::new(v),
            EarlyLintPassVariant::ReferenceToUndefinedValue(v) => Box::new(v),
            EarlyLintPassVariant::UpperCaseName(v) => Box::new(v),
            EarlyLintPassVariant::UniqueAttributeName(v) => Box::new(v),
            EarlyLintPassVariant::FileModeAttributeIsString(v) => Box::new(v),
            EarlyLintPassVariant::EnsureAttributeIsNotTheFirst(v) => Box::new(v),
            EarlyLintPassVariant::MultipleResourcesWithoutDefault(v) => Box::new(v),
            EarlyLintPassVariant::PerExpressionResourceDefaults(v) => Box::new(v),
            EarlyLintPassVariant::ExecAttributes(v) => Box::new(v),
            EarlyLintPassVariant::SelectorInAttributeValue(v) => Box::new(v),
            EarlyLintPassVariant::UnconditionalExec(v) => Box::new(v),
            EarlyLintPassVariant::InvalidResourceSetInvocation(v) => Box::new(v),
            EarlyLintPassVariant::InvalidResourceCollectionInvocation(v) => Box::new(v),
            EarlyLintPassVariant::EmptyCasesList(v) => Box::new(v),
            EarlyLintPassVariant::DefaultCaseIsNotLast(v) => Box::new(v),
            EarlyLintPassVariant::MultipleDefaultCase(v) => Box::new(v),
            EarlyLintPassVariant::NoDefaultCase(v) => Box::new(v),
            EarlyLintPassVariant::StatementWithNoEffect(v) => Box::new(v),
            EarlyLintPassVariant::RelationToTheLeft(v) => Box::new(v),
            EarlyLintPassVariant::InvalidStringEscape(v) => Box::new(v),
            EarlyLintPassVariant::UnusedVariables(v) => Box::new(v),
        }
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Storage {
    early_pass: Vec<EarlyLintPassVariant>,
}

impl Storage {
    pub fn register_early_pass(&mut self, lint: EarlyLintPassVariant) {
        self.early_pass.push(lint);
    }

    pub fn early_pass(&self) -> &[EarlyLintPassVariant] {
        &self.early_pass
    }
}

impl Default for Storage {
    fn default() -> Self {
        let mut v = Self {
            early_pass: Vec::new(),
        };

        v.register_early_pass(EarlyLintPassVariant::OptionalArgumentsGoesFirst(
            super::lint_toplevel::OptionalArgumentsGoesFirst,
        ));
        v.register_early_pass(EarlyLintPassVariant::UniqueArgumentsNames(
            super::lint_toplevel::UniqueArgumentsNames,
        ));
        v.register_early_pass(EarlyLintPassVariant::ArgumentLooksSensitive(
            super::lint_argument::ArgumentLooksSensitive::default(),
        ));
        v.register_early_pass(EarlyLintPassVariant::SensitiveArgumentWithDefault(
            super::lint_argument::SensitiveArgumentWithDefault,
        ));
        v.register_early_pass(EarlyLintPassVariant::ArgumentTyped(
            super::lint_argument::ArgumentTyped,
        ));
        v.register_early_pass(EarlyLintPassVariant::ReadableArgumentsName(
            super::lint_argument::ReadableArgumentsName::default(),
        ));
        v.register_early_pass(EarlyLintPassVariant::LowerCaseArgumentName(
            super::lint_argument::LowerCaseArgumentName,
        ));
        v.register_early_pass(EarlyLintPassVariant::DoNotUseUnless(
            super::lint_unless::DoNotUseUnless,
        ));
        v.register_early_pass(EarlyLintPassVariant::UselessParens(
            super::lint_expression::UselessParens,
        ));
        v.register_early_pass(EarlyLintPassVariant::InvalidVariableAssignment(
            super::lint_expression::InvalidVariableAssignment,
        ));
        v.register_early_pass(EarlyLintPassVariant::DoubleNegation(
            super::lint_expression::DoubleNegation,
        ));
        v.register_early_pass(EarlyLintPassVariant::NegationOfEquation(
            super::lint_expression::NegationOfEquation,
        ));
        v.register_early_pass(EarlyLintPassVariant::ConstantExpressionInCondition(
            super::lint_expression::ConstantExpressionInCondition,
        ));
        v.register_early_pass(EarlyLintPassVariant::ErbReferencesToUnknownVariable(
            super::lint_builtin::ErbReferencesToUnknownVariable,
        ));
        v.register_early_pass(EarlyLintPassVariant::UselessDoubleQuotes(
            super::lint_string_expr::UselessDoubleQuotes,
        ));
        v.register_early_pass(EarlyLintPassVariant::ExpressionInSingleQuotes(
            super::lint_string_expr::ExpressionInSingleQuotes,
        ));
        v.register_early_pass(EarlyLintPassVariant::LowerCaseVariable(
            super::lint_term::LowerCaseVariable,
        ));
        v.register_early_pass(EarlyLintPassVariant::ReferenceToUndefinedValue(
            super::lint_term::ReferenceToUndefinedValue,
        ));
        v.register_early_pass(EarlyLintPassVariant::UpperCaseName(
            super::lint_resource_set::UpperCaseName,
        ));
        v.register_early_pass(EarlyLintPassVariant::UniqueAttributeName(
            super::lint_resource_set::UniqueAttributeName,
        ));
        v.register_early_pass(EarlyLintPassVariant::FileModeAttributeIsString(
            super::lint_resource_set::FileModeAttributeIsString,
        ));
        v.register_early_pass(EarlyLintPassVariant::EnsureAttributeIsNotTheFirst(
            super::lint_resource_set::EnsureAttributeIsNotTheFirst,
        ));
        v.register_early_pass(EarlyLintPassVariant::MultipleResourcesWithoutDefault(
            super::lint_resource_set::MultipleResourcesWithoutDefault,
        ));
        v.register_early_pass(EarlyLintPassVariant::PerExpressionResourceDefaults(
            super::lint_resource_set::PerExpressionResourceDefaults,
        ));
        v.register_early_pass(EarlyLintPassVariant::ExecAttributes(
            super::lint_resource_set::ExecAttributes,
        ));
        v.register_early_pass(EarlyLintPassVariant::SelectorInAttributeValue(
            super::lint_resource_set::SelectorInAttributeValue,
        ));
        v.register_early_pass(EarlyLintPassVariant::UnconditionalExec(
            super::lint_resource_set::UnconditionalExec,
        ));
        v.register_early_pass(EarlyLintPassVariant::InvalidResourceSetInvocation(
            super::lint_resource_set::InvalidResourceSetInvocation,
        ));
        v.register_early_pass(EarlyLintPassVariant::InvalidResourceCollectionInvocation(
            super::lint_resource_set::InvalidResourceCollectionInvocation,
        ));
        v.register_early_pass(EarlyLintPassVariant::EmptyCasesList(
            super::lint_case_statement::EmptyCasesList,
        ));
        v.register_early_pass(EarlyLintPassVariant::DefaultCaseIsNotLast(
            super::lint_case_statement::DefaultCaseIsNotLast,
        ));
        v.register_early_pass(EarlyLintPassVariant::MultipleDefaultCase(
            super::lint_case_statement::MultipleDefaultCase,
        ));
        v.register_early_pass(EarlyLintPassVariant::NoDefaultCase(
            super::lint_case_statement::NoDefaultCase,
        ));
        v.register_early_pass(EarlyLintPassVariant::StatementWithNoEffect(
            super::lint_statement::StatementWithNoEffect,
        ));
        v.register_early_pass(EarlyLintPassVariant::RelationToTheLeft(
            super::lint_statement::RelationToTheLeft,
        ));
        v.register_early_pass(EarlyLintPassVariant::InvalidStringEscape(
            super::lint_string_expr::InvalidStringEscape,
        ));
        v.register_early_pass(EarlyLintPassVariant::UnusedVariables(
            super::lint_ctx::UnusedVariables,
        ));
        v
    }
}

pub struct AstLinter;

impl AstLinter {
    pub fn check_string_expression(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::string::StringExpr<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_string_expression(elt));
        }

        if let crate::puppet_lang::string::StringVariant::DoubleQuoted(s) = &elt.data {
            for fragment in s {
                match fragment {
                    crate::puppet_lang::string::DoubleQuotedFragment::StringFragment(_) => {}
                    crate::puppet_lang::string::DoubleQuotedFragment::Expression(elt) => errors
                        .append(&mut self.check_expression(storage, ctx, true, false, &elt.data)),
                }
            }
        }

        errors
    }

    pub fn check_array(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        is_assignment: bool,
        array: &crate::puppet_lang::expression::Array<Range>,
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        accessor: &Option<crate::puppet_lang::expression::Accessor<Range>>,
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::expression::Map<Range>,
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
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        _elt: &crate::puppet_lang::expression::Variable<Range>,
    ) -> Vec<LintError> {
        Vec::new()
    }

    pub fn check_type_specification(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::typing::TypeSpecification<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        match &elt.data {
            crate::puppet_lang::typing::TypeSpecificationVariant::ExternalType(elt) => {
                for arg in &elt.arguments {
                    errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Enum(list) => {
                for elt in &list.list {
                    errors.append(&mut self.check_term(storage, ctx, false, elt));
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Array(elt) => {
                if let Some(inner) = &elt.inner {
                    errors.append(&mut self.check_type_specification(storage, ctx, inner))
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Hash(elt) => {
                if let Some(key) = &elt.key {
                    errors.append(&mut self.check_type_specification(storage, ctx, key))
                }
                if let Some(value) = &elt.value {
                    errors.append(&mut self.check_type_specification(storage, ctx, value))
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Optional(elt) => match &elt.value
            {
                crate::puppet_lang::typing::TypeOptionalVariant::TypeSpecification(elt) => {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt))
                }
                crate::puppet_lang::typing::TypeOptionalVariant::Term(elt) => {
                    errors.append(&mut self.check_term(storage, ctx, false, elt));
                }
            },
            crate::puppet_lang::typing::TypeSpecificationVariant::Variant(elt) => {
                for elt in &elt.list {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt));
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Struct(elt) => {
                for elt in &elt.keys.value {
                    match &elt.key {
                        crate::puppet_lang::typing::TypeStructKey::String(elt) => {
                            errors.append(&mut self.check_string_expression(storage, ctx, elt));
                        }
                        crate::puppet_lang::typing::TypeStructKey::Optional(elt) => {
                            errors.append(
                                &mut self.check_string_expression(storage, ctx, &elt.value),
                            );
                        }
                        crate::puppet_lang::typing::TypeStructKey::NotUndef(elt) => {
                            errors.append(
                                &mut self.check_string_expression(storage, ctx, &elt.value),
                            );
                        }
                    }
                    errors.append(&mut self.check_type_specification(storage, ctx, &elt.value));
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Sensitive(elt) => {
                match &elt.value {
                    crate::puppet_lang::typing::TypeSensitiveVariant::TypeSpecification(elt) => {
                        errors.append(&mut self.check_type_specification(storage, ctx, elt));
                    }
                    crate::puppet_lang::typing::TypeSensitiveVariant::Term(elt) => {
                        errors.append(&mut self.check_term(storage, ctx, false, elt));
                    }
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Tuple(elt) => {
                for elt in &elt.list {
                    errors.append(&mut self.check_type_specification(storage, ctx, elt))
                }
            }
            crate::puppet_lang::typing::TypeSpecificationVariant::Float(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Integer(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Numeric(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::String(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Pattern(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Regex(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Boolean(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Undef(_)
            | crate::puppet_lang::typing::TypeSpecificationVariant::Any(_) => { // TODO
            }
        }

        errors
    }

    pub fn check_term(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        is_assignment: bool,
        elt: &crate::puppet_lang::expression::Term<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_term(ctx, is_assignment, elt));
        }

        match &elt.value {
            crate::puppet_lang::expression::TermVariant::String(elt) => {
                errors.append(&mut self.check_string_expression(storage, ctx, elt));
            }
            crate::puppet_lang::expression::TermVariant::Parens(expr) => errors.append(
                &mut self.check_expression(storage, ctx, false, is_assignment, &expr.value),
            ),
            crate::puppet_lang::expression::TermVariant::Array(list) => {
                errors.append(&mut self.check_array(storage, ctx, is_assignment, list))
            }
            crate::puppet_lang::expression::TermVariant::Map(elt) => {
                errors.append(&mut self.check_map(storage, ctx, elt))
            }
            crate::puppet_lang::expression::TermVariant::Variable(elt) => {
                errors.append(&mut self.check_variable(storage, ctx, elt))
            }
            crate::puppet_lang::expression::TermVariant::TypeSpecitifaction(elt) => {
                errors.append(&mut self.check_type_specification(storage, ctx, elt))
            }
            crate::puppet_lang::expression::TermVariant::Float(_)
            | crate::puppet_lang::expression::TermVariant::Integer(_)
            | crate::puppet_lang::expression::TermVariant::Boolean(_)
            | crate::puppet_lang::expression::TermVariant::RegexpGroupID(_)
            | crate::puppet_lang::expression::TermVariant::Sensitive(_)
            | crate::puppet_lang::expression::TermVariant::Identifier(_)
            | crate::puppet_lang::expression::TermVariant::Regexp(_) => {
                // TODO
            }
        }

        errors
    }

    pub fn check_funcall(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::expression::FunctionCall<Range>,
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::builtin::BuiltinVariant<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        match elt {
            crate::puppet_lang::builtin::BuiltinVariant::Undef => {}
            crate::puppet_lang::builtin::BuiltinVariant::Tag(v)
            | crate::puppet_lang::builtin::BuiltinVariant::Require(v)
            | crate::puppet_lang::builtin::BuiltinVariant::Realize(v)
            | crate::puppet_lang::builtin::BuiltinVariant::CreateResources(v)
            | crate::puppet_lang::builtin::BuiltinVariant::Include(v) => {
                for arg in &v.args {
                    errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
                }
                if let Some(lambda) = &v.lambda {
                    errors.append(&mut self.check_lambda(storage, ctx, lambda))
                }
            }
            crate::puppet_lang::builtin::BuiltinVariant::Return(arg) => {
                if let Some(arg) = arg.as_ref() {
                    errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
                }
            }
            crate::puppet_lang::builtin::BuiltinVariant::Template(arg) => {
                errors.append(&mut self.check_expression(storage, ctx, true, false, arg));
            }
        }

        errors
    }

    fn register_assignments(
        &self,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::expression::Expression<Range>,
    ) {
        use crate::puppet_lang::expression::ExpressionVariant;
        if let ExpressionVariant::Term(term) = &elt.value {
            match &term.value {
                crate::puppet_lang::expression::TermVariant::Array(list) => {
                    for elt in &list.value.value {
                        self.register_assignments(ctx, elt)
                    }
                }
                crate::puppet_lang::expression::TermVariant::Parens(elt) => {
                    self.register_assignments(ctx, &elt.value)
                }
                crate::puppet_lang::expression::TermVariant::Variable(variable) => {
                    ctx.register_defined_variable(variable)
                }
                _ => (),
            }
        }
    }

    pub fn check_expression(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        is_toplevel_expr: bool,
        is_assignment: bool,
        elt: &crate::puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_expression(ctx, is_toplevel_expr, elt));
        }

        use crate::puppet_lang::expression::ExpressionVariant;
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
                        crate::puppet_lang::expression::CaseVariant::Term(term) => {
                            errors.append(&mut self.check_term(storage, ctx, false, term))
                        }
                        crate::puppet_lang::expression::CaseVariant::Default(_) => {}
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::ConditionAndStatement<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_unless(elt));
            errors.append(&mut lint.inner().check_condition_expression(&elt.condition));
        }

        errors.append(&mut self.check_expression(storage, ctx, true, false, &elt.condition));

        errors.append(&mut self.check_statement_set(storage, ctx, elt.body.value.as_ref()));

        errors
    }

    pub fn check_condition_expression(
        &self,
        storage: &Storage,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::expression::Expression<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_condition_expression(elt));
        }

        errors
    }

    pub fn check_if_else(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::IfElse<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_if_else(elt));
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_resource_set(ctx, elt));
        }

        for resource in &elt.list.value {
            errors.append(&mut self.check_expression(storage, ctx, true, false, &resource.title));
            ctx.register_phantom_variable("title");
            for attribute in &resource.attributes.value {
                match &attribute.value {
                    crate::puppet_lang::statement::ResourceAttributeVariant::Name((
                        _name,
                        value,
                    )) => {
                        errors.append(&mut self.check_expression(storage, ctx, true, false, value))
                    }
                    crate::puppet_lang::statement::ResourceAttributeVariant::Group(term) => {
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::resource_collection::ResourceCollection<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_resource_collection(ctx, elt));
        }

        errors.append(&mut self.check_type_specification(storage, ctx, &elt.type_specification));

        errors
    }

    pub fn check_relation_elt(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::RelationElt<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_relation_elt(elt));
        }

        for elt in &elt.data.value {
            match elt {
                crate::puppet_lang::statement::RelationEltVariant::ResourceSet(elt) => {
                    errors.append(&mut self.check_resource_set(storage, ctx, elt))
                }
                crate::puppet_lang::statement::RelationEltVariant::ResourceCollection(elt) => {
                    errors.append(&mut self.check_resource_collection(storage, ctx, elt))
                }
            }
        }

        errors
    }

    pub fn check_relation(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        prev: &crate::puppet_lang::statement::RelationElt<Range>,
        elt: &crate::puppet_lang::statement::Relation<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_relation(prev, elt));
        }

        errors.append(&mut self.check_relation_list(storage, ctx, elt.relation_to.as_ref()));

        errors
    }

    pub fn check_relation_list(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::RelationList<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_relation_list(elt));
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::Case<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_case_statement(elt));
        }
        errors.append(&mut self.check_condition_expression(storage, ctx, &elt.condition));
        errors.append(&mut self.check_expression(storage, ctx, true, false, &elt.condition));

        for case in &elt.elements.value {
            for matchcase in &case.matches {
                match matchcase {
                    crate::puppet_lang::expression::CaseVariant::Term(term) => {
                        errors.append(&mut self.check_term(storage, ctx, false, term));
                    }
                    crate::puppet_lang::expression::CaseVariant::Default(_) => (),
                }
            }

            errors.append(&mut self.check_statement_set(storage, ctx, &case.body.value));
        }

        errors
    }

    pub fn check_statement_set(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        list: &[crate::puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for statement in list {
            errors.append(&mut self.check_statement(storage, ctx, statement));
        }
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_statement_set(ctx, list));
        }

        errors
    }

    pub fn check_deprecated_resource_defaults(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::statement::ResourceDefaults<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_deprecated_resource_defaults(elt));
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        statement: &crate::puppet_lang::statement::Statement<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_statement(statement));
        }

        use crate::puppet_lang::statement::StatementVariant;
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        arguments: &[crate::puppet_lang::argument::Argument<Range>],
        body: &[crate::puppet_lang::statement::Statement<Range>],
    ) -> Vec<LintError> {
        let ctx = ctx.new_scope();
        ctx.register_phantom_variable("name");
        ctx.register_phantom_variable("title");

        let mut errors = Vec::new();
        for arg in arguments {
            errors.append(&mut self.check_argument(storage, &ctx, arg));
        }

        errors.append(&mut self.check_statement_set(storage, &ctx, body));

        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_ctx(&ctx));
        }

        errors
    }

    pub fn check_class(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::toplevel::Class<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_class(elt));
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::argument::Argument<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_argument(elt));
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::expression::Lambda<Range>,
    ) -> Vec<LintError> {
        let ctx = ctx.clone();
        let mut errors = Vec::new();
        for arg in &elt.args.value {
            errors.append(&mut self.check_argument(storage, &ctx, arg))
        }
        errors.append(&mut self.check_statement_set(storage, &ctx, &elt.body.value));

        errors
    }

    pub fn check_definition(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::toplevel::Definition<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_definition(elt));
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
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::toplevel::Plan<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_plan(elt));
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
        _ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::toplevel::TypeDef<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_typedef(elt));
        }

        errors
    }

    pub fn check_functiondef(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::toplevel::FunctionDef<Range>,
    ) -> Vec<LintError> {
        let ctx = ctx.new_scope();
        let mut errors = Vec::new();
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_functiondef(elt));
        }

        errors.append(&mut self.check_toplevel_variant(
            storage,
            &ctx,
            &elt.arguments.value,
            &elt.body.value,
        ));

        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_ctx(&ctx));
        }

        errors
    }

    pub fn check_toplevel(
        &self,
        storage: &Storage,
        ctx: &crate::puppet_pp_lint::ctx::Ctx,
        elt: &crate::puppet_lang::toplevel::Toplevel<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        match &elt.data {
            crate::puppet_lang::toplevel::ToplevelVariant::Class(elt) => {
                errors.append(&mut self.check_class(storage, ctx, elt))
            }
            crate::puppet_lang::toplevel::ToplevelVariant::Definition(elt) => {
                errors.append(&mut self.check_definition(storage, ctx, elt))
            }
            crate::puppet_lang::toplevel::ToplevelVariant::Plan(elt) => {
                errors.append(&mut self.check_plan(storage, ctx, elt))
            }
            crate::puppet_lang::toplevel::ToplevelVariant::TypeDef(elt) => {
                errors.append(&mut self.check_typedef(storage, ctx, elt))
            }
            crate::puppet_lang::toplevel::ToplevelVariant::FunctionDef(elt) => {
                errors.append(&mut self.check_functiondef(storage, ctx, elt))
            }
        }
        for lint in storage.early_pass() {
            errors.append(&mut lint.inner().check_toplevel(ctx, elt))
        }

        errors
    }
}
