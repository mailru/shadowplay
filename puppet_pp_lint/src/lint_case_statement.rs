use puppet_parser::Location;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
pub struct EmptyCasesList;

impl LintPass for EmptyCasesList {
    fn name(&self) -> &str {
        "empty_cases_last"
    }
}

impl EarlyLintPass for EmptyCasesList {
    fn check_case_statement(&self, elt: &puppet_lang::statement::Case<Location>) -> Vec<LintError> {
        if elt.elements.is_empty() {
            return vec![LintError::new(
                Box::new(self.clone()),
                "Cases list is empty",
                &elt.extra,
            )];
        }

        vec![]
    }
}

#[derive(Clone)]
pub struct DefaultCaseIsNotLast;

impl LintPass for DefaultCaseIsNotLast {
    fn name(&self) -> &str {
        "default_case_is_not_last"
    }
}

impl EarlyLintPass for DefaultCaseIsNotLast {
    fn check_case_statement(&self, elt: &puppet_lang::statement::Case<Location>) -> Vec<LintError> {
        let mut default = None;
        let mut errors = Vec::new();
        for case in &elt.elements {
            if case
                .matches
                .iter()
                .any(|elt| matches!(elt, puppet_lang::expression::CaseVariant::Default(_)))
            {
                default = Some(case)
            } else if let Some(default) = default {
                errors.push(LintError::new(
                    Box::new(self.clone()),
                    &format!(
                        "Match case after default match which is defined earlier at line {}",
                        default.extra.line()
                    ),
                    &elt.extra,
                ))
            }
        }

        errors
    }
}

#[derive(Clone)]
pub struct MultipleDefaultCase;

impl LintPass for MultipleDefaultCase {
    fn name(&self) -> &str {
        "multiple_default_cases"
    }
}

impl EarlyLintPass for MultipleDefaultCase {
    fn check_case_statement(&self, elt: &puppet_lang::statement::Case<Location>) -> Vec<LintError> {
        let mut default: Option<&puppet_lang::statement::CaseElement<Location>> = None;
        let mut errors = Vec::new();
        for case in &elt.elements {
            if case
                .matches
                .iter()
                .any(|elt| matches!(elt, puppet_lang::expression::CaseVariant::Default(_)))
            {
                if let Some(default) = default {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        &format!(
                            "Default match case is already defined at line {}",
                            default.extra.line()
                        ),
                        &elt.extra,
                    ))
                }
                default = Some(case)
            }
        }

        errors
    }
}

#[derive(Clone)]
pub struct NoDefaultCase;

impl LintPass for NoDefaultCase {
    fn name(&self) -> &str {
        "no_default_case"
    }
}

impl EarlyLintPass for NoDefaultCase {
    fn check_case_statement(&self, elt: &puppet_lang::statement::Case<Location>) -> Vec<LintError> {
        let mut has_default = false;
        for case in &elt.elements {
            for case_elt in &case.matches {
                if matches!(case_elt, puppet_lang::expression::CaseVariant::Default(_)) {
                    has_default = true
                }
            }
        }

        if !has_default {
            return vec![LintError::new_with_url(
                Box::new(self.clone()),
                "Case with no default",
                "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_conditionals-case-selector-defaults",
                &elt.extra,
            )];
        }

        vec![]
    }
}
