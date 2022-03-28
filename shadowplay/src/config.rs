use anyhow::{format_err, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Default, Serialize)]
pub struct ChecksHieraYaml {
    #[serde(default)]
    pub forced_modules_exists: std::collections::HashSet<String>,
    #[serde(default)]
    pub forced_values_exists: std::collections::HashSet<String>,
}

#[derive(Deserialize, Clone, Default, Serialize)]
pub struct Checks {
    pub hiera_yaml: ChecksHieraYaml,
    pub pp: puppet_pp_lint::lint::Storage,
}

#[derive(Deserialize, Clone, Default, Serialize)]
pub struct Config {
    pub checks: Checks,
}

impl Config {
    pub fn read(file: &std::path::Path) -> Result<Self> {
        let config = std::fs::read_to_string(file)
            .map_err(|err| format_err!("Failed to load config file {:?}: {}", file, err))?;
        let config: Self = serde_yaml::from_str(&config)
            .map_err(|err| format_err!("Failed to parse config file {:?}: {}", file, err))?;
        Ok(config)
    }
}
