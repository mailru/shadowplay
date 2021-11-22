pub struct Module {
    pub module_name: String,
    pub subclasses: Vec<String>,
}

impl Module {
    /// Из строчки вида "norisk::client::install::version" получает имя модуля: ["norisk", "client", "install"]
    pub fn of_hiera(hiera_key: &str) -> Option<Self> {
        let elts = hiera_key
            .split("::")
            .map(|v| v.to_owned())
            .collect::<Vec<String>>();
        match elts.as_slice() {
            [] => {
                // empty key name
                None
            }
            [_local_value] => {
                // some local value
                None
            }
            [module_name, subclasses @ .., _class_parameter] => Some(Self {
                module_name: module_name.to_owned(),
                subclasses: subclasses.to_vec(),
            }),
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
}
