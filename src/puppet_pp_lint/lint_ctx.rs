use serde::{Deserialize, Serialize};

use crate::{
    puppet_parser::range::Range,
    puppet_pp_lint::lint::{EarlyLintPass, LintError, LintPass},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct UnusedVariables;

impl LintPass for UnusedVariables {
    fn name(&self) -> &str {
        "UnusedVariables"
    }
    fn description(&self) -> &str {
        "Checks for unused variables"
    }
}

impl EarlyLintPass for UnusedVariables {
    fn check_ctx(&self, ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>) -> Vec<LintError> {
        let mut errors = Vec::new();
        let variables = ctx.variables.borrow();

        for (varname, variable) in &*variables {
            let use_count = variable.use_count.borrow();
            if *use_count > 0 {
                continue;
            }

            match &variable.variant {
                crate::puppet_pp_lint::ctx::VariableVariant::Builtin => (),
                crate::puppet_pp_lint::ctx::VariableVariant::Defined(variable) => {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        &format!("Variable '{}' is never used [EXPERIMENTAL]", varname),
                        &variable.extra,
                    ))
                }
                crate::puppet_pp_lint::ctx::VariableVariant::Argument(arg) => {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        &format!("Argument '{}' is never used [EXPERIMENTAL]", varname),
                        &arg.extra,
                    ))
                }
                crate::puppet_pp_lint::ctx::VariableVariant::Phantom => (),
            }
        }

        errors
    }
}
