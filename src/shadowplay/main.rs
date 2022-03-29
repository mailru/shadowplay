pub mod check;
pub mod config;
pub mod hiera_config;

use std::io::Read;

use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

const APP_CONFIG: &str = "/etc/shadowplay.yaml";

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
pub enum CheckVariant {
    /// Check specified yaml files
    Yaml(crate::check::yaml::Check),
    /// Check specified hiera yaml files
    Hiera(crate::check::hiera_yaml::Check),
    /// Check specified *.pp files
    Pp(crate::check::pp::Check),
}

#[derive(Debug, StructOpt)]
pub struct Check {
    /// Output format. Possible values: "one-line", "json"
    #[structopt(short, default_value = "one-line")]
    pub format: crate::check::error::OutputFormat,
    #[structopt(subcommand)]
    pub variant: CheckVariant,
}

#[derive(Debug, StructOpt)]
pub struct Dump {
    /// List of *.pp files
    pub paths: Vec<std::path::PathBuf>,
}

impl Dump {
    pub fn dump(&self) {
        for path in &self.paths {
            let pp = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("Cannot load {:?}: {}", &path, err));

            let ast = match crate::check::PuppetAst::parse(&pp) {
                Err(err) => {
                    let err = match err {
                        nom::Err::Incomplete(_) => {
                            // nom::complete doesn't generate this state
                            unreachable!()
                        }
                        nom::Err::Error(v) => v,
                        nom::Err::Failure(v) => v,
                    };
                    panic!("Cannot parse {:?}: {}", &path, &err)
                }
                Ok(v) => v,
            };
            println!("{}", serde_json::to_string(&ast.data).unwrap())
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct PrettyPrint {
    #[structopt(long, default_value = "120")]
    pub width: usize,
}

impl PrettyPrint {
    pub fn pretty_print(&self) {
        let mut buf = String::new();
        let _ = std::io::stdin()
            .read_to_string(&mut buf)
            .expect("Read STDIN");

        let ast = match crate::check::PuppetAst::parse(&buf) {
            Err(err) => {
                let err = match err {
                    nom::Err::Incomplete(_) => {
                        // nom::complete doesn't generate this state
                        unreachable!()
                    }
                    nom::Err::Error(v) => v,
                    nom::Err::Failure(v) => v,
                };
                panic!("Cannot parse STDIN: {}", &err)
            }
            Ok(v) => v,
        };

        let mut w = Vec::new();
		  shadowplay::puppet_pp_printer::statement::statement_block_to_doc(&ast.data, false)
            .render(self.width, &mut w)
            .unwrap();
        let pretty = String::from_utf8(w).unwrap();
        println!("{}", pretty)
    }
}

#[derive(Debug, StructOpt)]
pub enum Query {
    /// Get value for specific host
    Get(Get),
    /// Checks subcommand
    Check(Check),
    /// Pretty printing subcommand
    PrettyPrintPp(PrettyPrint),
    /// Dump *.pp files
    Dump(Dump),
    /// Generates default config
    GenerateConfig,
    /// Prints list of available PP lints
    PrintPpLints,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "shadowplay", about = "Puppet checker, linter and explorer.")]
struct Opt {
    #[structopt(default_value = "./", long)]
    pub repo_path: std::path::PathBuf,
    #[structopt(long)]
    pub config: Option<std::path::PathBuf>,
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
        key: &located_yaml::Yaml,
        value: &located_yaml::Yaml,
        traverse_path: &[&str],
    ) {
        let s = match &value.yaml {
            located_yaml::YamlElt::Real(v) => {
                format!("{:?}", v)
            }
            located_yaml::YamlElt::String(v) => {
                format!("{:?}", v)
            }
            located_yaml::YamlElt::Integer(v) => {
                format!("{:?}", v)
            }
            located_yaml::YamlElt::Boolean(v) => {
                format!("{:?}", v)
            }
            located_yaml::YamlElt::Array(_) => "<ARRAY VALUE>".to_owned(),
            located_yaml::YamlElt::Hash(_) => "<HASH VALUE>".to_owned(),
            located_yaml::YamlElt::Alias(_) => "<YAML ALIAS VALUE>".to_owned(),
            located_yaml::YamlElt::Null => "<NULL VALUE>".to_owned(),
            located_yaml::YamlElt::BadValue => "<BAD VALUE>".to_owned(),
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

        self.git_blame(repo_path, yaml_path, min_line, max_line)
            .unwrap()
    }

    fn show(
        &self,
        repo_path: &std::path::Path,
        yaml_path: &std::path::Path,
        key: &located_yaml::Yaml,
        value: &located_yaml::Yaml,
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
                serde_yaml::to_string(&located_yaml::Untagged::of_yaml(value)).unwrap()
            ),
            ValuePrintFormat::Json => println!(
                "{}",
                serde_json::to_string(&located_yaml::Untagged::of_yaml(value)).unwrap()
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

                let yaml_str = match std::fs::read_to_string(&yaml_path) {
                    Ok(v) => v,
                    Err(err) => {
                        log::error!("Failed to load file {:?}: {}", yaml_path, err);
                        continue;
                    }
                };

                let yaml = match located_yaml::YamlLoader::load_from_str(&yaml_str) {
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
                    .and_then(|v| v.get_string());

                let new_extgrpbase2 = yaml.docs[0]
                    .get_string_key("extgrpbase2")
                    .and_then(|v| v.get_string());

                let new_inventory_group_name = yaml.docs[0]
                    .get_string_key("group_name")
                    .and_then(|v| v.get_string());

                let new_ext_slave_group = yaml.docs[0]
                    .get_string_key("ext_slave_group")
                    .and_then(|v| v.get_string());

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
                    located_yaml::YamlElt::Hash(v) => v,
                    _ => {
                        log::error!("Top value of {:?} is not a map", yaml_path);
                        continue;
                    }
                };

                for (k, v) in hash {
                    if k.yaml == located_yaml::YamlElt::String(self.key.clone()) {
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
    pub fn check(&self, repo_path: &std::path::Path, config: crate::config::Config) {
        let summary = match &self.variant {
            CheckVariant::Yaml(v) => v.check(repo_path, &config, &self.format),
            CheckVariant::Hiera(v) => v.check(repo_path, &config, &self.format),
            CheckVariant::Pp(v) => v.check(repo_path, &config, &self.format),
        };
        if self.format.is_human() {
            println!(
                "Checked {} files, detected {} issues",
                summary.files_checked, summary.errors_count
            )
        }
        if summary.errors_count > 0 {
            std::process::exit(1)
        }
    }
}

fn print_pp_lints() {
    let mut lints = shadowplay::puppet_pp_lint::lint::Storage::default()
        .early_pass()
        .to_vec();
    lints.sort_by(|a, b| a.inner().name().cmp(b.inner().name()));
    for lint in lints {
        println!("{}: {}", lint.inner().name(), lint.inner().description())
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let config = opt
        .config
        .map(|path| crate::config::Config::read(&path).unwrap())
        .unwrap_or_default();

    match &opt.query {
        Query::Get(v) => v.get(&opt.repo_path),
        Query::Dump(v) => v.dump(),
        Query::Check(v) => v.check(&opt.repo_path, config),
        Query::PrettyPrintPp(v) => v.pretty_print(),
        Query::GenerateConfig => {
            print!(
                "Below is default configuration. Save it to {}\n\n{}",
                APP_CONFIG,
                serde_yaml::to_string(&crate::config::Config::default()).unwrap()
            )
        }
        Query::PrintPpLints => print_pp_lints(),
    }
}
