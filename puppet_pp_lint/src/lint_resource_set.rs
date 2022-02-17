use puppet_parser::parser::Location;

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone)]
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
                Box::new(self.clone()),
                "Name of resource set contains upper case characters",
                &elt.extra,
            )];
        }
        vec![]
    }
}

#[derive(Clone)]
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
                            Box::new(self.clone()),
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

#[derive(Clone)]
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
                        errors.push(LintError::new_with_url(
                            Box::new(self.clone()),
                            "Attribute 'ensure' is not the first.",
                            "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-attribute-ordering",
                            &elt.extra,
                        ));
                    }
                }
            }
        }

        errors
    }
}

#[derive(Clone)]
pub struct FileModeAttributeIsString;

impl LintPass for FileModeAttributeIsString {
    fn name(&self) -> &str {
        "file_mode_attribute_is_string"
    }
}

impl EarlyLintPass for FileModeAttributeIsString {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        if elt.name.name.len() != 1 || elt.name.name[0] != "file" {
            return vec![];
        }

        for resource in &elt.list {
            for attribute in &resource.attributes {
                if let puppet_lang::statement::ResourceAttribute::Name(attribute) = attribute {
                    if attribute.0.data == "mode" {
                        if let puppet_lang::expression::ExpressionVariant::Term(term) =
                            &attribute.1.value
                        {
                            match &term.value {
                                puppet_lang::expression::TermVariant::String(v) => {
                                    if !v.data.chars().all(|v| v.is_digit(10)) {
                                        return vec![LintError::new_with_url(
                                            Box::new(self.clone()),
                                            "Mode attribute is a string which is not all of digits.",
                                            "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-file-modes",
                                            &attribute.1.extra,
                                        )];
                                    }
                                    if v.data.len() != 4 {
                                        return vec![LintError::new_with_url(
                                            Box::new(self.clone()),
                                            "Mode attribute is a string which length != 4.",
                                            "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-file-modes",
                                            &attribute.1.extra,
                                        )];
                                    }
                                }
                                puppet_lang::expression::TermVariant::Integer(_) => {
                                    return vec![LintError::new_with_url(
                                    Box::new(self.clone()),
                                        "Integer value of mode attribute. Use string.",
                                        "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-file-modes",
                                        &attribute.1.extra,
                                    )];
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        vec![]
    }
}

#[derive(Clone)]
pub struct MultipleResourcesWithoutDefault;

impl LintPass for MultipleResourcesWithoutDefault {
    fn name(&self) -> &str {
        "multiple_resources_without_default"
    }
}

impl EarlyLintPass for MultipleResourcesWithoutDefault {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        let mut has_default = false;
        for resource in &elt.list {
            if let puppet_lang::expression::ExpressionVariant::Term(term) = &resource.title.value {
                if let puppet_lang::expression::TermVariant::String(v) = &term.value {
                    if v.data == "default" {
                        has_default = true
                    }
                }
            }
        }

        if elt.list.len() > 1 {
            if !has_default {
                return vec![LintError::new_with_url(
                    Box::new(self.clone()),
                    "Multiples resources without default set.",
                    "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-multiple-resources",
                    &elt.extra,
                )];
            }
            if elt.list.len() == 2 {
                return vec![LintError::new(
                                        Box::new(self.clone()),
                    "Multiples resources with default set and only two sets in total. Defaults set can be merged with the only resource.",
                    &elt.extra,
                )];
            }
        }

        vec![]
    }
}

#[derive(Clone)]
pub struct SelectorInAttributeValue;

impl LintPass for SelectorInAttributeValue {
    fn name(&self) -> &str {
        "selector_in_attribute_value"
    }
}

impl EarlyLintPass for SelectorInAttributeValue {
    fn check_resource_set(
        &self,
        elt: &puppet_lang::statement::ResourceSet<Location>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for resource in &elt.list {
            for attribute in &resource.attributes {
                if let puppet_lang::statement::ResourceAttribute::Name(attribute) = attribute {
                    if matches!(
                        attribute.1.value,
                        puppet_lang::expression::ExpressionVariant::Selector(_)
                    ) {
                        errors.push(LintError::new_with_url(
                            Box::new(self.clone()),
                            "Selector is used in attribute value",
                            "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_conditionals-simple-resource-declarations",
                            &attribute.1.extra,
                        ));
                    }
                }
            }
        }

        errors
    }
}
