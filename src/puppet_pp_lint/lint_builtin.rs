use crate::puppet_lang::{
    builtin::BuiltinVariant,
    expression::{ExpressionVariant, TermVariant},
};
use crate::puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use crate::puppet_pp_lint::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct ErbReferencesToUnknownVariable;

impl LintPass for ErbReferencesToUnknownVariable {
    fn name(&self) -> &str {
        "ErbReferencesToUnknownVariable"
    }
    fn description(&self) -> &str {
        "Checks ERB templates specified in template() for undefined variables"
    }
}

impl EarlyLintPass for ErbReferencesToUnknownVariable {
    fn check_expression(
        &self,
        ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>,
        _is_toplevel_expr: bool,
        elt: &crate::puppet_lang::expression::Expression<Range>,
    ) -> Vec<super::lint::LintError> {
        let builtin = if let ExpressionVariant::BuiltinFunction(v) = &elt.value {
            v
        } else {
            return Vec::new();
        };

        let builtin = if let BuiltinVariant::Template(v) = builtin {
            v
        } else {
            return Vec::new();
        };

        let mut errors = Vec::new();

        for arg in &builtin.args {
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

            let arg = if let Some(v) = crate::puppet_tool::string::constant_value(arg) {
                v
            } else {
                return Vec::new();
            };

            let _template = ctx.erb_of_path(&arg);
            let template: &crate::puppet_pp_lint::ctx::erb_template::Template =
                if let Some(v) = _template.as_ref() {
                    v
                } else {
                    return vec![LintError::new(
                        Box::new(self.clone()),
                        &format!("ERB template {:?} does not exists for failed to parse", arg),
                        &elt.extra,
                    )];
                };

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
        }

        errors
    }
}
