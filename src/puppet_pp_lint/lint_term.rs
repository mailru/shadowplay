use crate::{puppet_lang::expression::Term, puppet_parser::range::Range};
use serde::{Deserialize, Serialize};

use crate::puppet_pp_lint::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct LowerCaseVariable;

impl LintPass for LowerCaseVariable {
    fn name(&self) -> &str {
        "LowerCaseVariable"
    }
    fn description(&self) -> &str {
        "Warns if variable name is not lowercase"
    }
}

impl EarlyLintPass for LowerCaseVariable {
    fn check_term(
        &self,
        _ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>,
        _is_assignment: bool,
        elt: &crate::puppet_lang::expression::Term<Range>,
    ) -> Vec<super::lint::LintError> {
        if let crate::puppet_lang::expression::TermVariant::Variable(var) = &elt.value {
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
    fn description(&self) -> &str {
        "Warns if variable is not defined in current context"
    }
}

impl EarlyLintPass for ReferenceToUndefinedValue {
    fn check_term(
        &self,
        ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>,
        is_assignment: bool,
        elt: &crate::puppet_lang::expression::Term<Range>,
    ) -> Vec<super::lint::LintError> {
        let variable = match &elt.value {
            crate::puppet_lang::expression::TermVariant::Variable(v) => v,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct MagicNumber;

impl LintPass for MagicNumber {
    fn name(&self) -> &str {
        "MagicNumber"
    }
    fn description(&self) -> &str {
        "Warns if term contains magic number"
    }
}

impl EarlyLintPass for MagicNumber {
    fn check_term(
        &self,
        ctx: &crate::puppet_pp_lint::ctx::Ctx<Range>,
        _is_assignment: bool,
        elt: &Term<Range>,
    ) -> Vec<super::lint::LintError> {
        let magic_number = match &elt.value {
            crate::puppet_lang::expression::TermVariant::Integer(v) => {
                if v.value > 10 || v.value < -10 {
                    format!("{}", v.value)
                } else {
                    return Vec::new();
                }
            }
            crate::puppet_lang::expression::TermVariant::Float(v) => {
                if v.value > 10.0 || v.value < -10.0 {
                    format!("{}", v.value)
                } else {
                    let s = format!("{}", v.value);
                    if s.len() > 2 {
                        s
                    } else {
                        return Vec::new();
                    }
                }
            }
            crate::puppet_lang::expression::TermVariant::String(_)
            | crate::puppet_lang::expression::TermVariant::Boolean(_)
            | crate::puppet_lang::expression::TermVariant::Array(_)
            | crate::puppet_lang::expression::TermVariant::Identifier(_)
            | crate::puppet_lang::expression::TermVariant::Parens(_)
            | crate::puppet_lang::expression::TermVariant::Map(_)
            | crate::puppet_lang::expression::TermVariant::Variable(_)
            | crate::puppet_lang::expression::TermVariant::RegexpGroupID(_)
            | crate::puppet_lang::expression::TermVariant::Sensitive(_)
            | crate::puppet_lang::expression::TermVariant::TypeSpecitifaction(_)
            | crate::puppet_lang::expression::TermVariant::Regexp(_) => {
                return Vec::new();
            }
        };

        if ctx.path.is_empty() {
            return Vec::new();
        }

        if ctx.path.iter().any(|p_elt| {
            matches!(
                p_elt,
                crate::puppet_pp_lint::ctx::Path::ExpressionAssignRight(_)
            ) || matches!(p_elt, crate::puppet_pp_lint::ctx::Path::Argument(_))
        }) {
            return Vec::new();
        }

        return vec![LintError::new(
            Box::new(self.clone()),
            &format!(
                "Magic number {}. Assign it as named constant.",
                magic_number
            ),
            &elt.extra,
        )];
    }
}
