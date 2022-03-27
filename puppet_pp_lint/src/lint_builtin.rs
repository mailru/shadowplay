use puppet_lang::{
    builtin::BuiltinVariant,
    expression::{ExpressionVariant, TermVariant},
};
use puppet_parser::range::Range;

use crate::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

#[derive(Clone)]
pub struct ErbReferencesToUnknownVariable;

impl LintPass for ErbReferencesToUnknownVariable {
    fn name(&self) -> &str {
        "erb_references_to_unknown_variable"
    }
}

impl EarlyLintPass for ErbReferencesToUnknownVariable {
    fn check_expression(
        &self,
        ctx: &crate::ctx::Ctx,
        _is_toplevel_expr: bool,
        elt: &puppet_lang::expression::Expression<Range>,
    ) -> Vec<super::lint::LintError> {
        let builtin = if let ExpressionVariant::BuiltinFunction(v) = &elt.value {
            v
        } else {
            return Vec::new();
        };

        let arg = if let BuiltinVariant::Template(v) = builtin {
            v
        } else {
            return Vec::new();
        };

        let arg = if let ExpressionVariant::Term(v) = &arg.value {
            v
        } else {
            return Vec::new();
        };

        let arg = if let TermVariant::String(v) = &arg.value {
            v
        } else {
            return Vec::new();
        };

        let arg = if let Some(v) = puppet_tool::string::constant_value(arg) {
            v
        } else {
            return Vec::new();
        };

        let _template = ctx.erb_of_path(&arg);
        let template: &crate::ctx::erb_template::Template = if let Some(v) = _template.as_ref() {
            v
        } else {
            return Vec::new();
        };

        let mut errors = Vec::new();

        let variables = ctx.variables.borrow();
        for var in &template.referenced_variables {
            match variables.get(var) {
                None => {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        &format!(
                            "ERB template references to undefined in this context variable {:?}",
                            var
                        ),
                        &elt.extra,
                    ));
                }
                Some(var) => var.incr_use_count(),
            }
        }

        errors
    }
}
