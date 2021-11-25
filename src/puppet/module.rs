use anyhow::{bail, Result};

pub struct Module {
    pub module_name: String,
    pub subclasses: Vec<String>,
}

impl Module {
    /// Из строчки вида "norisk::client::install::version" получает имя модуля: ["norisk", "client", "install"] и имя параметра
    pub fn of_hiera<'a>(hiera_key: &'a str) -> Result<Option<(Self, &'a str)>> {
        let elts = hiera_key.split("::").collect::<Vec<&'a str>>();
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
                    bail!("Module name {:?} contains invalid characters", module_name)
                }
                for subclass in subclasses {
                    if !subclass
                        .chars()
                        .all(|c| char::is_alphanumeric(c) || c == '_')
                    {
                        bail!(
                            "Module subclass name {:?} contains invalid characters",
                            subclass
                        )
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

    /// Возвращает файл модуля:
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
