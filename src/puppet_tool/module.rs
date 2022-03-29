pub struct Module {
    pub module_name: String,
    pub subclasses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacters(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidCharacters(name) => write!(
                f,
                "Module or class name {:?} contains invalid characters",
                name
            ),
        }
    }
}

impl Module {
    pub fn of_identifier(identifier: &[String]) -> Option<Self> {
        match identifier {
            [] => None,
            [module_name, subclasses @ ..] => Some(Self {
                module_name: module_name.clone(),
                subclasses: subclasses.to_vec(),
            }),
        }
    }

    /// From string "norisk::client::install::version" extracts: ["norisk", "client", "install"] + parameter name
    pub fn of_hiera(hiera_key: &str) -> Result<Option<(Self, &str)>, Error> {
        let elts = hiera_key.split("::").collect::<Vec<&str>>();
        match elts.as_slice() {
            [] => {
                // empty key name
                Ok(None)
            }
            [_local_value] => {
                // some local value
                Ok(None)
            }
            [module_name, subclasses @ .., class_parameter] => {
                if !module_name
                    .chars()
                    .all(|c| char::is_alphanumeric(c) || c == '_')
                {
                    return Err(Error::InvalidCharacters(module_name.to_string()));
                }
                for subclass in subclasses {
                    if !subclass
                        .chars()
                        .all(|c| char::is_alphanumeric(c) || c == '_')
                    {
                        return Err(Error::InvalidCharacters(subclass.to_string()));
                    }
                }
                let module = Self {
                    module_name: module_name.to_string(),
                    subclasses: subclasses.iter().map(|v| v.to_string()).collect(),
                };
                Ok(Some((module, class_parameter)))
            }
        }
    }

    /// Returns file path:
    ///  - some_module/init.pp
    ///  - some_module/subclass.pp
    ///  - some_module/subclass/subsubclass.pp
    pub fn file_path(&self) -> std::path::PathBuf {
        match self.subclasses.as_slice() {
            [] => std::path::Path::new(&self.module_name)
                .join("manifests")
                .join("init.pp"),
            [middle_elts @ .., last_name] => {
                let mut path = std::path::Path::new(&self.module_name).join("manifests");
                for elt in middle_elts {
                    path = path.join(elt);
                }
                path = path.join(format!("{}.pp", last_name));
                path
            }
        }
    }

    pub fn full_file_path(&self, repo_path: &std::path::Path) -> std::path::PathBuf {
        repo_path.join("modules").join(self.file_path())
    }

    pub fn name(&self) -> String {
        format!(
            "{}{}",
            self.module_name,
            self.subclasses
                .iter()
                .map(|v| format!("::{}", v))
                .collect::<Vec<_>>()
                .join("")
        )
    }

    pub fn identifier(&self) -> Vec<&str> {
        let mut r = vec![self.module_name.as_str()];
        r.extend(self.subclasses.iter().map(|v| v.as_str()));
        r
    }
}
