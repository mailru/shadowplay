use puppet_parser::parser::Location;

use crate::check::pp_static::lint::LintError;

use super::lint::{EarlyLintPass, LintPass};

pub struct UpperCaseName;

impl LintPass for UpperCaseName {
    fn name(&self) -> &str {
        "upper_case_name_of_resource_set"
    }
}

impl EarlyLintPass for UpperCaseName {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        if elt
            .name
            .name
            .iter()
            .any(|v| v.chars().any(|v| v.is_uppercase()))
        {
            return vec![LintError::new(
                self.name(),
                "Name of resource set contains upper case characters",
                &elt.extra,
            )];
        }
        vec![]
    }
}

pub struct UniqueAttributeName;

impl LintPass for UniqueAttributeName {
    fn name(&self) -> &str {
        "unique_attribute_name"
    }
}

impl EarlyLintPass for UniqueAttributeName {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for resource in &elt.list {
            let mut names = std::collections::HashSet::new();
            for attribute in &resource.attributes {
                if let puppet_lang::statement::ResourceAttribute::Name(name) = attribute {
                    if names.contains(&name.0.data) {
                        errors.push(LintError::new(
                            self.name(),
                            &format!("Attribute {:?} is not unique", name.0.data),
                            &elt.extra,
                        ));
                    }
                    let _ = names.insert(&name.0.data);
                }
            }
        }

        errors
    }
}

pub struct EnsureAttributeIsNotTheFirst;

impl LintPass for EnsureAttributeIsNotTheFirst {
    fn name(&self) -> &str {
        "ensure_attribute_is_not_the_first"
    }
}

impl EarlyLintPass for EnsureAttributeIsNotTheFirst {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for resource in &elt.list {
            for (pos, attribute) in resource.attributes.iter().enumerate() {
                if let puppet_lang::statement::ResourceAttribute::Name(name) = attribute {
                    if name.0.data == "ensure" && pos > 0 {
                        errors.push(LintError::new(
                self.name(),
                "Attribute 'ensure' is not the first. See https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-attribute-ordering",
                &elt.extra,
            ));
                    }
                }
            }
        }

        errors
    }
}
