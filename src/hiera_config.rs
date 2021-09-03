use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Defaults {
    pub data_hash: String,
    pub datadir: std::path::PathBuf,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HierarchyElt {
    pub name: String,
    #[serde(default)]
    pub lookup_key: Option<String>,
    pub paths: Vec<String>,
    #[serde(default)]
    pub options: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HieraConfig {
    pub version: u16,
    pub defaults: Defaults,
    pub hierarchy: Vec<HierarchyElt>,
}

lazy_static! {
    static ref INTERPOLATION_RE: regex::Regex = regex::Regex::new("%\\{([^\\}]+)\\}").unwrap();
}

impl HieraConfig {
    pub fn read(path: &std::path::Path) -> Result<Self> {
        let str = std::fs::read_to_string(path)
            .map_err(|err| anyhow::format_err!("Failed to read {:?}: {}", path, err))?;
        let r: Self = serde_yaml::from_str(&str)
            .map_err(|err| anyhow::format_err!("Failed to parse {:?}: {}", path, err))?;
        Ok(r)
    }

    pub fn substitude_paths(
        &self,
        substitutions: &std::collections::HashMap<String, String>,
    ) -> Self {
        let mut hierarchy = Vec::new();
        for elt in &self.hierarchy {
            let mut paths = Vec::new();
            for path in &elt.paths {
                let mut all_replaced = true;
                let new_path = INTERPOLATION_RE.replace_all(&path, |caps: &regex::Captures| {
                    let key: String = caps[1].to_string();
                    match substitutions.get(&key) {
                        None => {
                            log::error!("Failed to substitude hiera value, key {:?} not found in substitutions", key);
                            all_replaced = false;
                            "".to_owned()
                        },
                        Some(replacement) => replacement.to_string()
                    }
                });
                if all_replaced {
                    paths.push(new_path.to_string())
                } else {
                    log::error!("Some substitutions failed, removing path {:?}", path)
                }
            }
            hierarchy.push(HierarchyElt {
                paths,
                ..elt.clone()
            })
        }
        Self {
            hierarchy,
            ..self.clone()
        }
    }
}
