use puppet_parser::parser::Location;

use crate::check::pp_static::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

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
                self.name(),
                "Cases list is empty",
                &elt.extra,
            )];
        }

        vec![]
    }
}

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
            if case.matches.iter().any(|elt| {
                if let puppet_lang::expression::TermVariant::String(v) = &elt.value {
                    v.data == "default"
                } else {
                    false
                }
            }) {
                default = Some(case)
            } else {
                if let Some(default) = default {
                    errors.push(LintError::new(
                        self.name(),
                        &format!(
                            "Match case after default match which is defined earlier at line {}",
                            default.extra.line()
                        ),
                        &elt.extra,
                    ))
                }
            }
        }

        errors
    }
}

pub struct MultipleDefaultCase;

impl LintPass for MultipleDefaultCase {
    fn name(&self) -> &str {
        "multiple_default_cases"
    }
}

impl EarlyLintPass for MultipleDefaultCase {
    fn check_case_statement(&self, elt: &puppet_lang::statement::Case<Location>) -> Vec<LintError> {
        let mut default: Option<
            &puppet_lang::statement::CaseElement<puppet_parser::parser::Location>,
        > = None;
        let mut errors = Vec::new();
        for case in &elt.elements {
            if case.matches.iter().any(|elt| {
                if let puppet_lang::expression::TermVariant::String(v) = &elt.value {
                    v.data == "default"
                } else {
                    false
                }
            }) {
                if let Some(default) = default {
                    errors.push(LintError::new(
                        self.name(),
                        &format!(
                            "Default match case is aready defined at line {}",
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
