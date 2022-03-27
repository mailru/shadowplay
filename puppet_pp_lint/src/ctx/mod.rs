pub mod builtin_resources;
pub mod erb_template;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use puppet_parser::range::Range;

use crate::ctx::builtin_resources::Attribute;

pub enum VariableVariant {
    Builtin,
    Defined(puppet_lang::expression::Variable<Range>),
    Argument(Box<puppet_lang::argument::Argument<Range>>),
    /// Implicit variables like $name or $title
    Phantom,
}

pub struct Variable {
    pub variant: VariableVariant,
    pub use_count: std::cell::RefCell<usize>,
}

impl Variable {
    pub fn builtin() -> Self {
        Self {
            variant: VariableVariant::Builtin,
            use_count: RefCell::new(0),
        }
    }

    pub fn phantom() -> Self {
        Self {
            variant: VariableVariant::Phantom,
            use_count: RefCell::new(0),
        }
    }

    pub fn defined(variable: &puppet_lang::expression::Variable<Range>) -> Self {
        Self {
            variant: VariableVariant::Defined(variable.clone()),
            use_count: RefCell::new(0),
        }
    }

    pub fn argument(argument: &puppet_lang::argument::Argument<Range>) -> Self {
        Self {
            variant: VariableVariant::Argument(Box::new(argument.clone())),
            use_count: RefCell::new(0),
        }
    }
}

impl Variable {
    pub fn incr_use_count(&self) {
        let mut use_count = self.use_count.borrow_mut();
        *use_count += 1
    }
}

pub struct NamedBlock {
    pub value: puppet_lang::toplevel::Toplevel<Range>,
}

type KnownResources = Rc<std::cell::RefCell<HashMap<Vec<String>, Rc<Option<NamedBlock>>>>>;
type KnownErbTemplates = Rc<
    std::cell::RefCell<HashMap<std::path::PathBuf, Rc<Option<crate::ctx::erb_template::Template>>>>,
>;

#[derive(Clone)]
pub struct Ctx {
    pub repository_path: Rc<std::path::PathBuf>,
    pub resources: KnownResources,
    pub builtin_resources: Rc<HashMap<&'static str, crate::ctx::builtin_resources::Resource>>,
    pub resource_metaparameters: Rc<HashMap<&'static str, Attribute>>,
    pub variables: std::cell::RefCell<HashMap<String, Rc<Variable>>>,
    pub erb_templates: KnownErbTemplates,
}

impl Ctx {
    pub fn new(repository_path: &std::path::Path) -> Self {
        let mut resource_metaparameters = HashMap::new();
        let _ = resource_metaparameters.insert("alias", Attribute::default());
        let _ = resource_metaparameters.insert("audit", Attribute::default());
        let _ = resource_metaparameters.insert("before", Attribute::default());
        let _ = resource_metaparameters.insert("loglevel", Attribute::default());
        let _ = resource_metaparameters.insert("noop", Attribute::default());
        let _ = resource_metaparameters.insert("notify", Attribute::default());
        let _ = resource_metaparameters.insert("require", Attribute::default());
        let _ = resource_metaparameters.insert("schedule", Attribute::default());
        let _ = resource_metaparameters.insert("stage", Attribute::default());
        let _ = resource_metaparameters.insert("subscribe", Attribute::default());
        let _ = resource_metaparameters.insert("tag", Attribute::default());

        let mut variables = HashMap::new();
        let _ = variables.insert("facts".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("trusted".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("clientcert".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("clientversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("puppetversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("clientnoop".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "agent_specified_environment".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("environment".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("servername".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("serverip".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("serverversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("module_name".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "caller_module_name".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("authenticated".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("certname".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("domain".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("extensions".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("hostname".to_string(), Rc::new(Variable::builtin()));

        // core facts
        let _ = variables.insert(
            "aio_agent_version".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("augeas".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("az_metadata".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("cloud".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("disks".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("dmi".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ec2_metadata".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ec2_userdata".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "env_windows_installdir".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("facterversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("filesystems".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("fips_enabled".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("gce".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("hypervisors".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("identity".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("is_virtual".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("kernel".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("kernelmajversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("kernelrelease".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("kernelversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ldom".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("load_averages".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("memory".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("mountpoints".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("networking".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("os".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("partitions".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("path".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("processors".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ruby".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("solaris_zones".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ssh".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("system_profiler".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("system_uptime".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("timezone".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("virtual".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("xen".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "zfs_featurenumbers".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("zfs_version".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "zpool_featureflags".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "zpool_featurenumbers".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("zpool_version".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("nim_type".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("architecture".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("augeasversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("blockdevices".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "bios_release_date".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("bios_vendor".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("bios_version".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("boardassettag".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "boardmanufacturer".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("boardproductname".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "boardserialnumber".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("chassisassettag".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("chassistype".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("dhcp_servers".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("domain".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("fqdn".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("gid".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("hardwareisa".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("hardwaremodel".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("hostname".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("id".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("interfaces".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ipaddress".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("ipaddress6".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("lsbdistcodename".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "lsbdistdescription".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("lsbdistid".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("lsbdistrelease".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "lsbmajdistrelease".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "lsbminordistrelease".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("lsbrelease".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("macaddress".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "macosx_buildversion".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "macosx_productname".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "macosx_productversion".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "macosx_productversion_major".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "macosx_productversion_minor".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "macosx_productversion_patch".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("manufacturer".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("memoryfree".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("memoryfree_mb".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("memorysize".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("memorysize_mb".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("netmask".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("netmask6".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("network".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("network6".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("operatingsystem".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "operatingsystemmajrelease".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "operatingsystemrelease".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("osfamily".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "physicalprocessorcount".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("processorcount".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("productname".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("rubyplatform".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("rubysitedir".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("rubyversion".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("scope6".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("selinux".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "selinux_config_mode".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "selinux_config_policy".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "selinux_current_mode".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("selinux_enforced".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "selinux_policyversion".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("serialnumber".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("swapencrypted".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("swapfree".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("swapfree_mb".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("swapsize".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("swapsize_mb".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert(
            "windows_edition_id".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "windows_installation_type".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "windows_product_name".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert(
            "windows_release_id".to_string(),
            Rc::new(Variable::builtin()),
        );
        let _ = variables.insert("system32".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("uptime".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("uptime_days".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("uptime_hours".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("uptime_seconds".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("uuid".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("xendomains".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("zonename".to_string(), Rc::new(Variable::builtin()));
        let _ = variables.insert("zones".to_string(), Rc::new(Variable::builtin()));

        Self {
            repository_path: Rc::new(repository_path.to_path_buf()),
            resources: Rc::new(std::cell::RefCell::new(HashMap::new())),
            builtin_resources: Rc::new(crate::ctx::builtin_resources::generate()),
            resource_metaparameters: Rc::new(resource_metaparameters),
            variables: RefCell::new(variables),
            erb_templates: Rc::new(std::cell::RefCell::new(HashMap::new())),
        }
    }

    fn fill_named_blocks(&self, name: &[String]) {
        // Lookup recursively
        if name.len() > 1 {
            if let &[list @ .., _suffix] = &name {
                self.fill_named_blocks(list)
            }
        }

        let module = match puppet_tool::module::Module::of_identifier(name) {
            None => {
                return;
            }
            Some(v) => v,
        };

        let file_content =
            match std::fs::read_to_string(module.full_file_path(&self.repository_path)) {
                Err(_err) => {
                    return;
                }
                Ok(v) => v,
            };

        let (_, statement_list) = match puppet_parser::toplevel::parse_file(
            puppet_parser::Span::new(file_content.as_str()),
        ) {
            Err(_err) => {
                return;
            }
            Ok(v) => v,
        };

        for statement in statement_list.value {
            match statement.value {
                puppet_lang::statement::StatementVariant::Toplevel(toplevel) => {
                    let name = match &toplevel.data {
                        puppet_lang::toplevel::ToplevelVariant::Class(v) => {
                            Some(&v.identifier.name)
                        }
                        puppet_lang::toplevel::ToplevelVariant::Definition(v) => {
                            Some(&v.identifier.name)
                        }
                        puppet_lang::toplevel::ToplevelVariant::Plan(v) => Some(&v.identifier.name),
                        puppet_lang::toplevel::ToplevelVariant::TypeDef(_) => {
                            // TODO
                            None
                        }
                        puppet_lang::toplevel::ToplevelVariant::FunctionDef(_) => {
                            // TODO
                            None
                        }
                    };

                    if let Some(name) = name {
                        let mut resources = self.resources.borrow_mut();
                        let _ = resources
                            .insert(name.clone(), Rc::new(Some(NamedBlock { value: toplevel })));
                    }
                }
                puppet_lang::statement::StatementVariant::Expression(_)
                | puppet_lang::statement::StatementVariant::RelationList(_)
                | puppet_lang::statement::StatementVariant::IfElse(_)
                | puppet_lang::statement::StatementVariant::Unless(_)
                | puppet_lang::statement::StatementVariant::Case(_)
                | puppet_lang::statement::StatementVariant::ResourceDefaults(_) => (),
            }
        }
    }

    pub fn block_of_name(&self, name: &[String]) -> Rc<Option<NamedBlock>> {
        {
            if let Some(v) = self.resources.borrow().get(&name.to_vec()) {
                return v.clone();
            }
        }

        self.fill_named_blocks(name);

        let mut resources = self.resources.borrow_mut();

        resources
            .entry(name.to_vec())
            .or_insert_with(|| Rc::new(None))
            .clone()
    }

    pub fn new_scope(&self) -> Self {
        let new_ctx = self.clone();

        // cleanup local variables
        {
            let mut variables = new_ctx.variables.borrow_mut();
            variables.retain(|_, v| match &v.variant {
                VariableVariant::Builtin => true,
                VariableVariant::Argument(_) => true,
                VariableVariant::Phantom => true,
                VariableVariant::Defined(v) => !v.is_local_scope,
            });
        }

        new_ctx
    }

    pub fn register_defined_variable(&self, variable: &puppet_lang::expression::Variable<Range>) {
        if variable.identifier.name.len() != 1 {
            return;
        }

        let mut variables = self.variables.borrow_mut();

        let _ = variables.insert(
            variable.identifier.name.first().unwrap().to_string(),
            Rc::new(Variable::defined(variable)),
        );
    }

    pub fn register_phantom_variable(&self, name: &str) {
        let mut variables = self.variables.borrow_mut();

        let _ = variables.insert(name.to_string(), Rc::new(Variable::phantom()));
    }

    pub fn register_argument_variable(&self, argument: &puppet_lang::argument::Argument<Range>) {
        let mut variables = self.variables.borrow_mut();

        let _ = variables.insert(argument.name.clone(), Rc::new(Variable::argument(argument)));
    }

    pub fn has_variable<EXTRA>(&self, variable: &puppet_lang::expression::Variable<EXTRA>) -> bool {
        // TODO lookup into foreign modules
        if variable.identifier.is_toplevel || variable.identifier.name.len() != 1 {
            return true;
        }

        let name = variable.identifier.name.first().unwrap();

        self.variables.borrow().contains_key(name)
    }

    pub fn erb_of_path(&self, path: &str) -> Rc<Option<crate::ctx::erb_template::Template>> {
        let path = std::path::Path::new(path);
        let mut path_components = path.components();

        let module_name = match path_components.next() {
            Some(v) => v,
            None => return Rc::new(None),
        };

        let full_path = self
            .repository_path
            .join("modules")
            .join(module_name)
            .join("templates")
            .join(path_components);

        {
            if let Some(v) = self.erb_templates.borrow().get(path) {
                return v.clone();
            }
        }

        let template = Rc::new(crate::ctx::erb_template::Template::read(&full_path));
        let mut erb_templates = self.erb_templates.borrow_mut();
        let _ = erb_templates.insert(full_path, template.clone());
        template
    }
}
