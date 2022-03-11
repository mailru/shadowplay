use puppet_parser::range::Range;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct LowerCaseVariable;

impl LintPass for LowerCaseVariable {
    fn name(&self) -> &str {
        "lower_case_variable"
    }
}

impl EarlyLintPass for LowerCaseVariable {
    fn check_term(
        &self,
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
