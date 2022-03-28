use puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct LowerCaseVariable;

impl LintPass for LowerCaseVariable {
    fn name(&self) -> &str {
        "LowerCaseVariable"
    }
}

impl EarlyLintPass for LowerCaseVariable {
    fn check_term(
        &self,
        _ctx: &crate::ctx::Ctx,
        _is_assignment: bool,
        elt: &puppet_lang::expression::Term<Range>,
    ) -> Vec<super::lint::LintError> {
        if let puppet_lang::expression::TermVariant::Variable(var) = &elt.value {
            if var
                .identifier
                .name
                .iter()
                .any(|elt| elt.chars().any(|c| c.is_uppercase()))
            {
                return vec![LintError::new_with_url(
                    Box::new(self.clone()),
                    "Variable name with upper case letters.",
                    "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_variables-variable-format",
                    &elt.extra,
                )];
            }
        }
        vec![]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReferenceToUndefinedValue;

impl LintPass for ReferenceToUndefinedValue {
    fn name(&self) -> &str {
        "ReferenceToUndefinedValue"
    }
}

impl EarlyLintPass for ReferenceToUndefinedValue {
    fn check_term(
        &self,
        ctx: &crate::ctx::Ctx,
        is_assignment: bool,
        elt: &puppet_lang::expression::Term<Range>,
    ) -> Vec<super::lint::LintError> {
        let variable = match &elt.value {
            puppet_lang::expression::TermVariant::Variable(v) => v,
            _ => return Vec::new(),
        };

        if !is_assignment && variable.identifier.name.len() == 1 {
            let varname = variable.identifier.name.first().unwrap();
            let variables = ctx.variables.borrow();

            match variables.get(varname) {
                None => {
                    return vec![LintError::new(
                        Box::new(self.clone()),
                        &format!(
                            "Reference to undefined value {:?}",
                            variable.identifier.name.join("::")
                        ),
                        &elt.extra,
                    )];
                }
                Some(var) => var.incr_use_count(),
            }
        }
        Vec::new()
    }
}
