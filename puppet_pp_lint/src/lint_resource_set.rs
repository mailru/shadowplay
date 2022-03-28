use puppet_parser::range::Range;
use serde::{Deserialize, Serialize};

use crate::lint::{EarlyLintPass, LintError, LintPass};

#[derive(Clone, Serialize, Deserialize)]
pub struct UpperCaseName;

impl LintPass for UpperCaseName {
    fn name(&self) -> &str {
        "UpperCaseName"
    }
}

impl EarlyLintPass for UpperCaseName {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct UniqueAttributeName;

impl LintPass for UniqueAttributeName {
    fn name(&self) -> &str {
        "UniqueAttributeName"
    }
}

impl EarlyLintPass for UniqueAttributeName {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for resource in &elt.list.value {
            let mut names = std::collections::HashSet::new();
            for attribute in &resource.attributes.value {
                if let puppet_lang::statement::ResourceAttributeVariant::Name(pair) =
                    &attribute.value
                {
                    let name = puppet_tool::string::raw_content(&pair.0);
                    if names.contains(&name) {
                        errors.push(LintError::new(
                            Box::new(self.clone()),
                            &format!("Attribute {:?} is not unique", name),
                            &elt.extra,
                        ));
                    }
                    let _ = names.insert(name.clone());
                }
            }
        }

        errors
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnsureAttributeIsNotTheFirst;

impl LintPass for EnsureAttributeIsNotTheFirst {
    fn name(&self) -> &str {
        "EnsureAttributeIsNotTheFirst"
    }
}

impl EarlyLintPass for EnsureAttributeIsNotTheFirst {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for resource in &elt.list.value {
            for (pos, attribute) in resource.attributes.value.iter().enumerate() {
                if let puppet_lang::statement::ResourceAttributeVariant::Name(pair) =
                    &attribute.value
                {
                    let name = puppet_tool::string::raw_content(&pair.0);
                    if name == "ensure" && pos > 0 {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct FileModeAttributeIsString;

impl LintPass for FileModeAttributeIsString {
    fn name(&self) -> &str {
        "FileModeAttributeIsString"
    }
}

impl FileModeAttributeIsString {
    fn check_expr(&self, expr: &puppet_lang::string::StringExpr<Range>) -> Vec<LintError> {
        let list = match &expr.data {
            puppet_lang::string::StringVariant::SingleQuoted(list) => list.clone(),
            puppet_lang::string::StringVariant::DoubleQuoted(list) => {
                let mut r = Vec::new();
                for elt in list {
                    match elt {
                        puppet_lang::string::DoubleQuotedFragment::StringFragment(elt) => {
                            r.push(elt.clone())
                        }
                        puppet_lang::string::DoubleQuotedFragment::Expression(_) => {
                            return Vec::new()
                        }
                    }
                }
                r
            }
        };

        let mut errors = Vec::new();
        for elt in list {
            match elt {
                puppet_lang::string::StringFragment::Literal(v) => {
                    if !v.data.chars().all(|v| v.is_digit(10)) {
                        errors.push(LintError::new_with_url(
                                            Box::new(self.clone()),
                                            "Mode attribute is a string which is not all of digits.",
                                            "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-file-modes",
                                            &expr.extra,
                                        ));
                    }
                    if v.data.len() != 4 {
                        errors.push(LintError::new_with_url(
                                            Box::new(self.clone()),
                                            "Mode attribute is a string which length != 4.",
                                            "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-file-modes",
                                            &expr.extra,
                                        ));
                    }
                }
                puppet_lang::string::StringFragment::Escaped(elt)
                | puppet_lang::string::StringFragment::EscapedUTF(elt) => {
                    errors.push(LintError::new(
                        Box::new(self.clone()),
                        "Mode attribute contains escaped char.",
                        &elt.extra,
                    ))
                }
            }
        }

        errors
    }
}

impl EarlyLintPass for FileModeAttributeIsString {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        if elt.name.name.len() != 1 || elt.name.name[0] != "file" {
            return vec![];
        }

        for resource in &elt.list.value {
            for attribute in &resource.attributes.value {
                if let puppet_lang::statement::ResourceAttributeVariant::Name(attribute) =
                    &attribute.value
                {
                    let name = puppet_tool::string::raw_content(&attribute.0);
                    if name == "mode" {
                        if let puppet_lang::expression::ExpressionVariant::Term(term) =
                            &attribute.1.value
                        {
                            match &term.value {
                                puppet_lang::expression::TermVariant::String(v) => {
                                    return self.check_expr(v)
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

#[derive(Clone, Serialize, Deserialize)]
pub struct MultipleResourcesWithoutDefault;

impl LintPass for MultipleResourcesWithoutDefault {
    fn name(&self) -> &str {
        "MultipleResourcesWithoutDefault"
    }
}

impl EarlyLintPass for MultipleResourcesWithoutDefault {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut has_default = false;
        for resource in &elt.list.value {
            if let puppet_lang::expression::ExpressionVariant::Term(term) = &resource.title.value {
                if let puppet_lang::expression::TermVariant::String(v) = &term.value {
                    if puppet_tool::string::raw_content(v) == "default" {
                        has_default = true
                    }
                }
            }
        }

        if elt.list.value.len() > 1 {
            if !has_default {
                return vec![LintError::new_with_url(
                    Box::new(self.clone()),
                    "Multiple esources without default set.",
                    "https://puppet.com/docs/puppet/7/style_guide.html#style_guide_resources-multiple-resources",
                    &elt.extra,
                )];
            }
            if elt.list.value.len() == 2 {
                return vec![LintError::new(
                                        Box::new(self.clone()),
                    "Multiple resources with default set and only two sets in total. Defaults set can be merged with the only resource.",
                    &elt.extra,
                )];
            }
        }

        vec![]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SelectorInAttributeValue;

impl LintPass for SelectorInAttributeValue {
    fn name(&self) -> &str {
        "SelectorInAttributeValue"
    }
}

impl EarlyLintPass for SelectorInAttributeValue {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();
        for resource in &elt.list.value {
            for attribute in &resource.attributes.value {
                if let puppet_lang::statement::ResourceAttributeVariant::Name(attribute) =
                    &attribute.value
                {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct UnconditionalExec;

impl LintPass for UnconditionalExec {
    fn name(&self) -> &str {
        "UnconditionalExec"
    }
}

impl EarlyLintPass for UnconditionalExec {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        if elt.name.name.len() != 1 || elt.name.name.first().unwrap() != "exec" {
            return vec![];
        }

        let mut errors = Vec::new();
        for resource in &elt.list.value {
            let mut found = false;
            for attribute in &resource.attributes.value {
                if let puppet_lang::statement::ResourceAttributeVariant::Name(attribute) =
                    &attribute.value
                {
                    let name = puppet_tool::string::raw_content(&attribute.0);
                    if name == "unless"
                        || name == "onlyif"
                        || name == "creates"
                        || name == "refreshonly"
                    {
                        found = true
                    }
                }
            }
            if !found {
                errors.push(LintError::new(
                    Box::new(self.clone()),
                    "exec {} resource without attribute 'unless', 'onlyif', 'creates' or 'refreshonly'",
                    &resource.extra,
                ));
            }
        }

        errors
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecAttributes;

impl LintPass for ExecAttributes {
    fn name(&self) -> &str {
        "ExecAttributes"
    }
}

impl EarlyLintPass for ExecAttributes {
    fn check_resource_set(
        &self,
        _ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        if elt.name.name.len() != 1 || elt.name.name.first().unwrap() != "exec" {
            return vec![];
        }

        let mut errors = Vec::new();

        for resource in &elt.list.value {
            let mut command = None;
            let mut provider = None;
            let mut path = None;
            for attribute in &resource.attributes.value {
                if let puppet_lang::statement::ResourceAttributeVariant::Name(attribute) =
                    &attribute.value
                {
                    let name = puppet_tool::string::raw_content(&attribute.0);
                    match name.as_str() {
                        "command" => command = Some(&attribute.1),
                        "provider" => provider = Some(&attribute.1),
                        "path" => path = Some(&attribute.1),
                        "creates" | "cwd" | "environment" | "group" | "logoutput" | "onlyif"
                        | "refresh" | "refreshonly" | "returns" | "timeout" | "tries"
                        | "try_sleep" | "umask" | "unless" | "user" => (),
                        name => {
                            match crate::tool::resource_set::is_valid_metaparameter_value(
                                name,
                                &attribute.1,
                            ) {
                                Some(false) => errors.push(LintError::new(
                                    Box::new(self.clone()),
                                    &format!("Invalid metaparameter {:?} value", name),
                                    &attribute.0.extra,
                                )),
                                Some(true) => (),
                                None => errors.push(LintError::new(
                                    Box::new(self.clone()),
                                    &format!("Parameter {:?} is not applicable to exec {{}}", name),
                                    &attribute.0.extra,
                                )),
                            }
                        }
                    }
                }
            }
            if command.is_none() {
                errors.push(LintError::new(
                    Box::new(self.clone()),
                    "exec {} with implicit 'command' attribute which value defaults to resource name",
                    &resource.extra,
                ));
                command = Some(&resource.title)
            }

            #[derive(PartialEq)]
            enum Provider {
                Posix,
                Shell,
                Windows,
                Unknown,
            }

            let provider = match &provider {
                Some(expr) => {
                    let provider_str = puppet_tool::expression::string_constant_value(expr);
                    match provider_str {
                        None => Provider::Unknown,
                        Some(provider_str) => match provider_str.as_str() {
                            "posix" => Provider::Posix,
                            "shell" => Provider::Shell,
                            "windows" => Provider::Windows,
                            other => {
                                errors.push(LintError::new(
                                    Box::new(self.clone()),
                                    &format!("Unexpected provider value {:?}", other),
                                    &expr.extra,
                                ));
                                Provider::Unknown
                            }
                        },
                    }
                }
                None => {
                    // TODO under Windows default is Windows
                    Provider::Posix
                }
            };

            let command_starts_with_path =
                match command.and_then(puppet_tool::expression::string_constant_value) {
                    None => {
                        // TODO detect possible set of values with static analyzer
                        true
                    }
                    Some(v) => v.starts_with('/'),
                };

            if !command_starts_with_path && provider == Provider::Posix && path.is_none() {
                errors.push(LintError::new(
                    Box::new(self.clone()),
                    "'path' is not set, 'provider' is not 'shell', thus 'command' attribute of exec {} must start with absolute path",
                    &resource.extra,
                ));
            }
        }

        errors
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerExpressionResourceDefaults;

impl LintPass for PerExpressionResourceDefaults {
    fn name(&self) -> &str {
        "PerExpressionResourceDefaults"
    }
}

impl EarlyLintPass for PerExpressionResourceDefaults {
    fn check_deprecated_resource_defaults(
        &self,
        elt: &puppet_lang::statement::ResourceDefaults<Range>,
    ) -> Vec<LintError> {
        vec![LintError::new_with_url(
            Box::new(self.clone()),
            "Whenever possible, use resource declaration defaults, also known as per-expression defaults",
            "https://puppet.com/docs/puppet/7/lang_resources.html#lang_resource_syntax-local-resource-defaults",
            &elt.extra,
        )]
    }
}

fn check_builtin_invocation<LINTER>(
    linter: &LINTER,
    ctx: &crate::ctx::Ctx,
    errors: &mut Vec<super::lint::LintError>,
    elt: &puppet_lang::statement::ResourceSet<Range>,
    builtin: &crate::ctx::builtin_resources::Resource,
) where
    LINTER: LintPass + Clone + 'static,
{
    for resource in &elt.list.value {
        for attribute in &resource.attributes.value {
            let name = match &attribute.value {
                puppet_lang::statement::ResourceAttributeVariant::Name((name, _)) => name,
                puppet_lang::statement::ResourceAttributeVariant::Group(_) => continue,
            };
            let const_name = match puppet_tool::string::constant_value(name) {
                None => continue,
                Some(v) => v,
            };

            if !builtin.attributes.contains_key(const_name.as_str())
                && !ctx
                    .resource_metaparameters
                    .contains_key(const_name.as_str())
            {
                errors.push(LintError::new(
                    Box::new(linter.clone()),
                    &format!(
                        "Builtin resource {:?} does not accept argument {:?}",
                        elt.name.name.join("::"),
                        const_name
                    ),
                    &name.extra,
                ))
            }
        }
    }
}

fn check_defined_resource_invocation<LINTER>(
    linter: &LINTER,
    ctx: &crate::ctx::Ctx,
    errors: &mut Vec<super::lint::LintError>,
    elt: &puppet_lang::statement::ResourceSet<Range>,
    named_block: &crate::ctx::NamedBlock,
) where
    LINTER: LintPass + Clone + 'static,
{
    let arguments = match &named_block.value.data {
        puppet_lang::toplevel::ToplevelVariant::Class(v) => &v.arguments.value,
        puppet_lang::toplevel::ToplevelVariant::Definition(v) => &v.arguments.value,
        puppet_lang::toplevel::ToplevelVariant::Plan(v) => &v.arguments.value,
        puppet_lang::toplevel::ToplevelVariant::TypeDef(_) => return,
        puppet_lang::toplevel::ToplevelVariant::FunctionDef(_) => return,
    };

    for resource in &elt.list.value {
        for attribute in &resource.attributes.value {
            let name = match &attribute.value {
                puppet_lang::statement::ResourceAttributeVariant::Name((name, _)) => name,
                puppet_lang::statement::ResourceAttributeVariant::Group(_) => continue,
            };
            let const_name = match puppet_tool::string::constant_value(name) {
                None => continue,
                Some(v) => v,
            };

            if !arguments.iter().any(|arg| arg.name == const_name)
                && !ctx
                    .resource_metaparameters
                    .contains_key(const_name.as_str())
            {
                errors.push(LintError::new(
                    Box::new(linter.clone()),
                    &format!(
                        "Resource {:?} does not accept argument {:?}",
                        elt.name.name.join("::"),
                        const_name
                    ),
                    &name.extra,
                ))
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InvalidResourceSetInvocation;

impl LintPass for InvalidResourceSetInvocation {
    fn name(&self) -> &str {
        "InvalidResourceSetInvocation"
    }
}

impl EarlyLintPass for InvalidResourceSetInvocation {
    fn check_resource_set(
        &self,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::statement::ResourceSet<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        let name: Vec<_> = elt.name.name.iter().map(|v| v.to_lowercase()).collect();

        let mut known_resource = false;

        if let Some(named_block) = ctx.block_of_name(name.as_slice()).as_ref() {
            known_resource = true;
            check_defined_resource_invocation(self, ctx, &mut errors, elt, named_block)
        }

        if !known_resource && name.len() == 1 {
            if let Some(builtin) = ctx.builtin_resources.get(name.first().unwrap().as_str()) {
                known_resource = true;
                check_builtin_invocation(self, ctx, &mut errors, elt, builtin)
            }

            if name.first().unwrap() == "class" {
                known_resource = true;
                for resource in &elt.list.value {
                    let title = match &resource.title.value {
                        puppet_lang::expression::ExpressionVariant::Term(term) => {
                            match &term.value {
                                puppet_lang::expression::TermVariant::String(v) => {
                                    match puppet_tool::string::constant_value(v) {
                                        None => continue,
                                        Some(v) => v,
                                    }
                                }
                                _ => continue,
                            }
                        }
                        _ => continue,
                    };

                    let title = title.strip_prefix("::").unwrap_or(&title);
                    let title_as_list: Vec<_> = title.split("::").map(|v| v.to_string()).collect();

                    match ctx.block_of_name(title_as_list.as_slice()).as_ref() {
                        Some(_) => (),
                        None => {
                            errors.push(LintError::new(
                                Box::new(self.clone()),
                                &format!("Reference to undefined class {:?}", title,),
                                &resource.title.extra,
                            ));
                        }
                    }
                }
            }
        }

        if !known_resource {
            errors.push(LintError::new(
                Box::new(self.clone()),
                &format!(
                    "Reference to undefined resource {:?}",
                    elt.name.name.join("::")
                ),
                &elt.name.extra,
            ))
        }

        errors
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InvalidResourceCollectionInvocation;

impl LintPass for InvalidResourceCollectionInvocation {
    fn name(&self) -> &str {
        "InvalidResourceCollectionInvocation"
    }
}

impl EarlyLintPass for InvalidResourceCollectionInvocation {
    fn check_resource_collection(
        &self,
        ctx: &crate::ctx::Ctx,
        elt: &puppet_lang::resource_collection::ResourceCollection<Range>,
    ) -> Vec<LintError> {
        let mut errors = Vec::new();

        let type_specification = match &elt.type_specification.data {
            puppet_lang::typing::TypeSpecificationVariant::ExternalType(elt) => elt,
            puppet_lang::typing::TypeSpecificationVariant::Float(_)
            | puppet_lang::typing::TypeSpecificationVariant::Integer(_)
            | puppet_lang::typing::TypeSpecificationVariant::Numeric(_)
            | puppet_lang::typing::TypeSpecificationVariant::String(_)
            | puppet_lang::typing::TypeSpecificationVariant::Pattern(_)
            | puppet_lang::typing::TypeSpecificationVariant::Regex(_)
            | puppet_lang::typing::TypeSpecificationVariant::Hash(_)
            | puppet_lang::typing::TypeSpecificationVariant::Boolean(_)
            | puppet_lang::typing::TypeSpecificationVariant::Array(_)
            | puppet_lang::typing::TypeSpecificationVariant::Undef(_)
            | puppet_lang::typing::TypeSpecificationVariant::Any(_)
            | puppet_lang::typing::TypeSpecificationVariant::Optional(_)
            | puppet_lang::typing::TypeSpecificationVariant::Variant(_)
            | puppet_lang::typing::TypeSpecificationVariant::Enum(_)
            | puppet_lang::typing::TypeSpecificationVariant::Struct(_)
            | puppet_lang::typing::TypeSpecificationVariant::Sensitive(_)
            | puppet_lang::typing::TypeSpecificationVariant::Tuple(_) => return Vec::new(),
        };

        let name: Vec<_> = type_specification
            .name
            .iter()
            .map(|v| v.to_lowercase())
            .collect();

        let mut known_resource = false;

        if let Some(_named_block) = ctx.block_of_name(name.as_slice()).as_ref() {
            // TODO check search expression of collection
            return Vec::new();
        }

        if name.len() == 1 {
            if let Some(_builtin) = ctx.builtin_resources.get(name.first().unwrap().as_str()) {
                // TODO check search expression of collection
                return Vec::new();
            }

            let name = name.first().unwrap();
            if name == "class" {
                known_resource = true;
                for arg in &type_specification.arguments {
                    let title = match &arg.value {
                        puppet_lang::expression::ExpressionVariant::Term(term) => {
                            match &term.value {
                                puppet_lang::expression::TermVariant::String(expr) => {
                                    match puppet_tool::string::constant_value(expr) {
                                        None => continue,
                                        Some(v) => v,
                                    }
                                }
                                _ => continue,
                            }
                        }
                        _ => continue,
                    };

                    let title = title.to_lowercase();
                    let title = title.strip_prefix("::").unwrap_or(&title);
                    let title_as_list: Vec<_> = title.split("::").map(|v| v.to_string()).collect();

                    match ctx.block_of_name(title_as_list.as_slice()).as_ref() {
                        Some(_) => (),
                        None => {
                            errors.push(LintError::new(
                                Box::new(self.clone()),
                                &format!(
                                    "Reference to undefined class {:?} in resource collection",
                                    title,
                                ),
                                &arg.extra,
                            ));
                        }
                    }
                }
            }
        }

        if !known_resource {
            errors.push(LintError::new(
                Box::new(self.clone()),
                &format!(
                    "Reference to undefined resource {:?} in resource collection",
                    name.join("::")
                ),
                &type_specification.extra,
            ))
        }

        errors
    }
}
