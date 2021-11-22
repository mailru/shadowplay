pub mod check;
pub mod hiera_config;
pub mod puppet;
pub mod yaml;

use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, StructOpt)]
pub enum ValuePrintFormat {
    Human,
    MarkedYaml,
    Yaml,
    Json,
}

impl std::str::FromStr for ValuePrintFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "human" => Self::Human,
            "marked-yaml" => Self::MarkedYaml,
            "yaml" => Self::Yaml,
            "json" => Self::Json,
            _ => anyhow::bail!("Invalid format: {}", s),
        };
        Ok(r)
    }
}

#[derive(Debug, StructOpt)]
pub struct Get {
    /// Operating system name as returned by facter
    #[structopt(default_value = "CentOS", long)]
    pub os: String,
    /// Operating system major release as returned by facter
    #[structopt(default_value = "7", long)]
    pub os_release: String,
    /// extsite, for example "mycom"
    #[structopt(long)]
    pub extsite: Option<String>,
    /// FQDN of the host being investigated
    pub fqdn: String,
    /// Hiera's key name, for example "zabbix_agent::install::version"
    pub key: String,
    /// Output format. "human" shows all related data in human readable format, including output of git blame. Other values are: yaml, json,
    /// marked-yaml
    #[structopt(short, default_value = "human")]
    pub format: ValuePrintFormat,
    /// Skip hiera groups with specified names
    #[structopt(long, default_value = "secrets")]
    pub skip_groups: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub enum Check {
    /// Check specified yaml file
    Yaml(crate::check::yaml::Check),
    /// Check specified hiera yaml file
    Hiera(crate::check::hiera_yaml::Check),
}

#[derive(Debug, StructOpt)]
pub enum Query {
    /// Get value for specific host
    Get(Get),
    /// Checks subcommand
    Check(Check),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "hixplorer", about = "Hiera explorer.")]
struct Opt {
    #[structopt(default_value = "./", long)]
    pub repo_path: std::path::PathBuf,
    #[structopt(subcommand)]
    pub query: Query,
}

fn substitutions(
    fqdn: &str,
    os_release: &str,
    extsite: Option<String>,
    extgrpbase1: &Option<String>,
    extgrpbase2: &Option<String>,
    inventory_group_name: &Option<String>,
    ext_slave_group: &Option<String>,
) -> std::collections::HashMap<String, String> {
    lazy_static! {
        static ref EXTGRP_RE: regex::Regex = regex::Regex::new("^(\\D+)").unwrap();
    }

    let mut substitutions = std::collections::HashMap::new();
    substitutions.insert("::fqdn".to_owned(), fqdn.to_owned());
    substitutions.insert("::operatingsystem".to_owned(), "CentOS".to_owned());
    substitutions.insert(
        "::operatingsystemmajrelease".to_owned(),
        os_release.to_owned(),
    );
    substitutions.insert(
        "extsite".to_owned(),
        extsite.map(|v| format!("{}/", v)).unwrap_or_default(),
    );
    if let Some(extgrpbase1) = extgrpbase1 {
        substitutions.insert("::extgrpbase1".to_owned(), extgrpbase1.clone());
    }
    if let Some(extgrpbase2) = extgrpbase2 {
        substitutions.insert("::extgrpbase2".to_owned(), extgrpbase2.clone());
    }
    if let Some(inventory_group_name) = inventory_group_name {
        substitutions.insert(
            "::inventory_group_name".to_owned(),
            inventory_group_name.clone(),
        );
    }
    if let Some(ext_slave_group) = ext_slave_group {
        substitutions.insert("::ext_slave_group".to_owned(), ext_slave_group.clone());
    }

    if let Some(caps) = EXTGRP_RE.captures(fqdn) {
        if let Some(extgrp) = caps.get(1) {
            substitutions.insert("::extgrp".to_owned(), extgrp.as_str().to_owned());
        }
    }

    substitutions
}

impl Get {
    fn git_blame(
        &self,
        repo_path: &std::path::Path,
        file_path: &std::path::Path,
        min_line: usize,
        max_line: usize,
    ) -> anyhow::Result<()> {
        let _ = std::process::Command::new("git")
            .current_dir(repo_path)
            .args(&[
                "--no-pager",
                "blame",
                "-w",
                "-L",
                &format!("{},{}", min_line, max_line,),
                "--",
                file_path.to_str().unwrap(),
            ])
            .stderr(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .output();
        Ok(())
    }

    fn show_human(
        &self,
        repo_path: &std::path::Path,
        yaml_path: &std::path::Path,
        key: &crate::yaml::Yaml,
        value: &crate::yaml::Yaml,
        traverse_path: &[&str],
    ) {
        let s = match &value.yaml {
            yaml::YamlElt::Real(v) => {
                format!("{:?}", v)
            }
            yaml::YamlElt::String(v) => {
                format!("{:?}", v)
            }
            yaml::YamlElt::Integer(v) => {
                format!("{:?}", v)
            }
            yaml::YamlElt::Boolean(v) => {
                format!("{:?}", v)
            }
            yaml::YamlElt::Array(_) => "<ARRAY VALUE>".to_owned(),
            yaml::YamlElt::Hash(_) => "<HASH VALUE>".to_owned(),
            yaml::YamlElt::Alias(_) => "<YAML ALIAS VALUE>".to_owned(),
            yaml::YamlElt::Null => "<NULL VALUE>".to_owned(),
            yaml::YamlElt::BadValue => "<BAD VALUE>".to_owned(),
        };
        let (key_min_line, key_max_line) = key.lines_range();
        let (val_min_line, val_max_line) = value.lines_range();
        let min_line = std::cmp::min(key_min_line, val_min_line);
        let max_line = std::cmp::max(key_max_line, val_max_line);

        println!(
            "Value: {}\nFound in {:?} at lines {}:{}\nValue lookup path was: {}",
            s,
            yaml_path,
            min_line,
            max_line,
            traverse_path.join(" -> ")
        );
        println!("===================================\nGit information:");

        self.git_blame(repo_path, &yaml_path, min_line, max_line)
            .unwrap()
    }

    fn show(
        &self,
        repo_path: &std::path::Path,
        yaml_path: &std::path::Path,
        key: &crate::yaml::Yaml,
        value: &crate::yaml::Yaml,
        traverse_path: &[&str],
    ) {
        match self.format {
            ValuePrintFormat::Human => {
                self.show_human(repo_path, yaml_path, key, value, traverse_path)
            }
            ValuePrintFormat::MarkedYaml => {
                println!("{}", serde_yaml::to_string(value).unwrap())
            }
            ValuePrintFormat::Yaml => println!(
                "{}",
                serde_yaml::to_string(&crate::yaml::Untagged::of_yaml(value)).unwrap()
            ),
            ValuePrintFormat::Json => println!(
                "{}",
                serde_json::to_string(&crate::yaml::Untagged::of_yaml(value)).unwrap()
            ),
        }
    }

    fn get_substituted(
        &self,
        repo_path: &std::path::Path,
        hiera_config: &crate::hiera_config::HieraConfig,
        extgrpbase1: &Option<String>,
        extgrpbase2: &Option<String>,
        inventory_group_name: &Option<String>,
        ext_slave_group: &Option<String>,
    ) {
        let substitutions = substitutions(
            &self.fqdn.clone(),
            &self.os_release,
            self.extsite.clone(),
            extgrpbase1,
            extgrpbase2,
            inventory_group_name,
            ext_slave_group,
        );

        log::debug!("Current substitutions: {:#?}", &substitutions);

        let hiera_config = hiera_config.substitude_paths(&substitutions);

        let default_paths = Vec::new();

        let mut traverse_path = Vec::new();

        for elt in &hiera_config.hierarchy {
            if self.skip_groups.contains(&elt.name) {
                log::debug!("Skipping hiera group {:?}", elt.name);
                continue;
            }
            for path in elt.paths.as_ref().unwrap_or(&default_paths) {
                traverse_path.push(path.as_str());
                let yaml_path = repo_path.join(&hiera_config.defaults.datadir).join(path);

                let yaml = match crate::yaml::load_file(&yaml_path) {
                    Ok(v) => v,
                    Err(err) => {
                        log::error!("Failed to parse {:?}: {}", yaml_path, err);
                        continue;
                    }
                };

                if yaml.docs.is_empty() {
                    log::error!("No documents found in yaml {:?}", yaml_path);
                    continue;
                }

                if yaml.docs.len() > 1 {
                    log::error!("Hiera YAML {:?} contains multiple documents", yaml_path);
                    continue;
                }

                if !yaml.errors.is_empty() {
                    for err in &yaml.errors {
                        log::warn!("Static checker detected error in {:?}: {}", yaml_path, err)
                    }
                }

                let new_extgrpbase1 = yaml.docs[0]
                    .get_string_key("extgrpbase1")
                    .map(|v| v.get_string())
                    .flatten();

                let new_extgrpbase2 = yaml.docs[0]
                    .get_string_key("extgrpbase2")
                    .map(|v| v.get_string())
                    .flatten();

                let new_inventory_group_name = yaml.docs[0]
                    .get_string_key("group_name")
                    .map(|v| v.get_string())
                    .flatten();

                let new_ext_slave_group = yaml.docs[0]
                    .get_string_key("ext_slave_group")
                    .map(|v| v.get_string())
                    .flatten();

                if new_extgrpbase1 > *extgrpbase1
                    || new_extgrpbase2 > *extgrpbase2
                    || new_inventory_group_name > *inventory_group_name
                    || new_ext_slave_group > *ext_slave_group
                {
                    return self.get_substituted(
                        repo_path,
                        &hiera_config,
                        std::cmp::max(extgrpbase1, &new_extgrpbase1),
                        std::cmp::max(extgrpbase2, &new_extgrpbase2),
                        std::cmp::max(inventory_group_name, &new_inventory_group_name),
                        std::cmp::max(ext_slave_group, &new_ext_slave_group),
                    );
                }

                let hash = match &yaml.docs[0].yaml {
                    crate::yaml::YamlElt::Hash(v) => v,
                    _ => {
                        log::error!("Top value of {:?} is not a hash", yaml_path);
                        continue;
                    }
                };

                for (k, v) in hash {
                    if k.yaml == crate::yaml::YamlElt::String(self.key.clone()) {
                        self.show(repo_path, &yaml_path, k, v, &traverse_path);
                        return;
                    }
                }
            }
        }
    }

    fn get(&self, repo_path: &std::path::Path) {
        let hiera_config =
            crate::hiera_config::HieraConfig::read(&repo_path.join("hiera.yaml")).unwrap();

        self.get_substituted(repo_path, &hiera_config, &None, &None, &None, &None)
    }
}

impl Check {
    pub fn check(&self, repo_path: &std::path::Path) {
        match self {
            Check::Yaml(v) => v.check(repo_path),
            Check::Hiera(v) => v.check(repo_path),
        }
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    match &opt.query {
        Query::Get(v) => v.get(&opt.repo_path),
        Query::Check(v) => v.check(&opt.repo_path),
    }
}
