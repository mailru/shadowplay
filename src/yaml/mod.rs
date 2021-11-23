#[cfg(test)]
mod tests;

use anyhow::Result;
use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use std::collections::BTreeMap;
use std::f64;
use std::i64;
use std::mem;
use std::string;
use yaml_rust::Event;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Serialize)]
pub struct Marker {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.col)
    }
}

impl From<&yaml_rust::scanner::Marker> for Marker {
    fn from(marker: &yaml_rust::scanner::Marker) -> Self {
        Self {
            index: marker.index(),
            line: marker.line(),
            col: marker.col(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum YamlElt {
    /// Float types are stored as String and parsed on demand.
    /// Note that f64 does NOT implement Eq trait and can NOT be stored in BTreeMap.
    Real(string::String),
    /// YAML int is stored as i64.
    Integer(i64),
    /// YAML scalar.
    String(string::String),
    /// YAML bool, e.g. `true` or `false`.
    Boolean(bool),
    /// YAML array, can be accessed as a `Vec`.
    Array(self::Array),
    /// YAML hash, can be accessed as a `LinkedHashMap`.
    ///
    /// Insertion order will match the order of insertion into the map.
    Hash(self::Hash),
    /// Alias, not fully supported yet.
    Alias(usize),
    /// YAML null, e.g. `null` or `~`.
    Null,
    /// Accessing a nonexistent node via the Index trait returns `BadValue`. This
    /// simplifies error handling in the calling code. Invalid type conversion also
    /// returns `BadValue`.
    BadValue,
}

impl YamlElt {
    pub fn type_name(&self) -> &str {
        match self {
            YamlElt::Real(_) => "real",
            YamlElt::Integer(_) => "integer",
            YamlElt::String(_) => "string",
            YamlElt::Boolean(_) => "boolean",
            YamlElt::Array(_) => "array",
            YamlElt::Hash(_) => "map",
            YamlElt::Alias(_) => "alias",
            YamlElt::Null => "null",
            YamlElt::BadValue => "badvalue",
        }
    }
}

#[derive(Clone, Debug, Eq, Serialize)]
pub struct Yaml {
    pub yaml: YamlElt,
    pub marker: Marker,
}

impl std::cmp::PartialEq for Yaml {
    fn eq(&self, other: &Self) -> bool {
        self.yaml == other.yaml
    }
}

impl std::hash::Hash for Yaml {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.yaml.hash(state)
    }
}

pub type Array = Vec<Yaml>;
pub type Hash = LinkedHashMap<Yaml, Yaml>;

impl Yaml {
    pub fn new(yaml: YamlElt, marker: &yaml_rust::scanner::Marker) -> Self {
        Self {
            yaml,
            marker: Marker::from(marker),
        }
    }

    // Not implementing FromStr because there is no possibility of Error.
    // This function falls back to Yaml::String if nothing else matches.
    pub fn from_str(v: &str, marker: &yaml_rust::scanner::Marker) -> Yaml {
        if let Some(suffix) = v.strip_prefix("0x") {
            if let Ok(i) = i64::from_str_radix(suffix, 16) {
                return Self::new(YamlElt::Integer(i), marker);
            }
        }
        if let Some(suffix) = v.strip_prefix("0o") {
            if let Ok(i) = i64::from_str_radix(suffix, 8) {
                return Self::new(YamlElt::Integer(i), marker);
            }
        }
        if let Some(suffix) = v.strip_prefix('+') {
            if let Ok(i) = suffix.parse::<i64>() {
                return Self::new(YamlElt::Integer(i), marker);
            }
        }
        match v {
            "~" | "null" => Self::new(YamlElt::Null, marker),
            "true" => Self::new(YamlElt::Boolean(true), marker),
            "false" => Self::new(YamlElt::Boolean(false), marker),
            _ if v.parse::<i64>().is_ok() => {
                Self::new(YamlElt::Integer(v.parse::<i64>().unwrap()), marker)
            }
            // try parsing as f64
            _ if parse_f64(v).is_some() => Self::new(YamlElt::Real(v.to_owned()), marker),
            _ => Self::new(YamlElt::String(v.to_owned()), marker),
        }
    }

    pub fn is_badvalue(&self) -> bool {
        (*self).yaml == YamlElt::BadValue
    }

    pub fn lines_range(&self) -> (usize, usize) {
        let (child_min, child_max) = match &self.yaml {
            YamlElt::Real(_)
            | YamlElt::Integer(_)
            | YamlElt::String(_)
            | YamlElt::Alias(_)
            | YamlElt::Null
            | YamlElt::BadValue
            | YamlElt::Boolean(_) => (self.marker.line, self.marker.line),
            YamlElt::Array(v) => match v.last() {
                None => (self.marker.line, self.marker.line),
                Some(v) => v.lines_range(),
            },
            YamlElt::Hash(v) => {
                let mut min = std::usize::MAX;
                let mut max = std::usize::MIN;
                for (key, val) in v {
                    let (key_min, _) = key.lines_range();
                    let (_, val_max) = val.lines_range();
                    min = std::cmp::min(min, key_min);
                    max = std::cmp::max(max, val_max);
                }
                (min, max)
            }
        };

        (
            std::cmp::min(self.marker.line, child_min),
            std::cmp::max(self.marker.line, child_max),
        )
    }

    pub fn get_string_key(&self, key: &str) -> Option<Self> {
        if let YamlElt::Hash(hash) = &self.yaml {
            return hash
                .get(&Yaml {
                    yaml: YamlElt::String(key.to_owned()),
                    marker: Marker {
                        index: 0,
                        line: 0,
                        col: 0,
                    },
                })
                .cloned();
        }
        None
    }

    pub fn get_string(&self) -> Option<String> {
        match &self.yaml {
            YamlElt::Real(v) | YamlElt::String(v) => Some(v.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum Untagged {
    String(string::String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<Option<Untagged>>),
    Hash(LinkedHashMap<Untagged, Option<Untagged>>),
}

impl Untagged {
    pub fn of_yaml(elt: &Yaml) -> Option<Self> {
        match &elt.yaml {
            YamlElt::Null => None,
            YamlElt::String(v) | YamlElt::Real(v) => Some(Untagged::String(v.clone())),
            YamlElt::Integer(v) => Some(Untagged::Integer(*v)),
            YamlElt::Boolean(v) => Some(Untagged::Boolean(*v)),
            YamlElt::Array(v) => {
                let v = v.iter().map(Self::of_yaml).collect();
                Some(Untagged::Array(v))
            }
            YamlElt::Hash(v) => {
                let mut r = LinkedHashMap::new();
                for (k, v) in v {
                    let k = Self::of_yaml(k).unwrap();
                    let v = Self::of_yaml(v);
                    let _ = r.insert(k, v);
                }
                Some(Untagged::Hash(r))
            }
            YamlElt::Alias(_) | YamlElt::BadValue => {
                panic!("Cannot translate yaml to untagged")
            }
        }
    }
}

pub mod error {
    use super::{Marker, YamlElt};

    #[derive(Debug, PartialEq)]
    pub struct DuplicateKey {
        pub key: YamlElt,
        pub first_mark: Marker,
        pub first_value: YamlElt,
        pub second_mark: Marker,
        pub second_value: YamlElt,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        DuplicateKey(DuplicateKey),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::DuplicateKey(v) => {
                    let key = match &v.key {
                        YamlElt::Real(v) => v.to_string(),
                        YamlElt::Integer(v) => format!("{}", v),
                        YamlElt::String(v) => v.to_string(),
                        YamlElt::Boolean(v) => format!("{}", v),
                        YamlElt::Array(_) => "[array value]".to_owned(),
                        YamlElt::Hash(_) => "[hash value]".to_owned(),
                        YamlElt::Alias(_) => "[alias value]".to_owned(),
                        YamlElt::Null => "[null value]".to_owned(),
                        YamlElt::BadValue => unreachable!(),
                    };
                    write!(
                        f,
                        "Duplicate key {} at {}. First occurred at {}",
                        key, v.second_mark, v.first_mark
                    )
                }
            }
        }
    }
}

pub struct YamlLoader {
    pub docs: Vec<Yaml>,
    // states
    // (current node, anchor_id) tuple
    doc_stack: Vec<(Yaml, usize)>,
    key_stack: Vec<Yaml>,
    anchor_map: BTreeMap<usize, Yaml>,
    pub errors: Vec<error::Error>,
}

impl yaml_rust::parser::MarkedEventReceiver for YamlLoader {
    fn on_event(&mut self, ev: Event, marker: yaml_rust::scanner::Marker) {
        // println!("EV {:?}", ev);
        match ev {
            Event::DocumentStart => {
                // do nothing
            }
            Event::DocumentEnd => {
                match self.doc_stack.len() {
                    // empty document
                    0 => self.docs.push(Yaml::new(YamlElt::BadValue, &marker)),
                    1 => self.docs.push(self.doc_stack.pop().unwrap().0),
                    _ => unreachable!(),
                }
            }
            Event::SequenceStart(aid) => {
                self.doc_stack
                    .push((Yaml::new(YamlElt::Array(Vec::new()), &marker), aid));
            }
            Event::SequenceEnd => {
                let node = self.doc_stack.pop().unwrap();
                self.insert_new_node(node, &marker);
            }
            Event::MappingStart(aid) => {
                self.doc_stack
                    .push((Yaml::new(YamlElt::Hash(Hash::new()), &marker), aid));
                self.key_stack.push(Yaml::new(YamlElt::BadValue, &marker));
            }
            Event::MappingEnd => {
                self.key_stack.pop().unwrap();
                let node = self.doc_stack.pop().unwrap();
                self.insert_new_node(node, &marker);
            }
            Event::Scalar(v, style, aid, tag) => {
                let node = if style != yaml_rust::scanner::TScalarStyle::Plain {
                    Yaml::new(YamlElt::String(v), &marker)
                } else if let Some(yaml_rust::scanner::TokenType::Tag(ref handle, ref suffix)) = tag
                {
                    // XXX tag:yaml.org,2002:
                    let elt = if handle == "!!" {
                        match suffix.as_ref() {
                            "bool" => {
                                // "true" or "false"
                                match v.parse::<bool>() {
                                    Err(_) => YamlElt::BadValue,
                                    Ok(v) => YamlElt::Boolean(v),
                                }
                            }
                            "int" => match v.parse::<i64>() {
                                Err(_) => YamlElt::BadValue,
                                Ok(v) => YamlElt::Integer(v),
                            },
                            "float" => match parse_f64(&v) {
                                Some(_) => YamlElt::Real(v),
                                None => YamlElt::BadValue,
                            },
                            "null" => match v.as_ref() {
                                "~" | "null" => YamlElt::Null,
                                _ => YamlElt::BadValue,
                            },
                            _ => YamlElt::String(v),
                        }
                    } else {
                        YamlElt::String(v)
                    };
                    Yaml::new(elt, &marker)
                } else {
                    // Datatype is not specified, or unrecognized
                    Yaml::from_str(&v, &marker)
                };

                self.insert_new_node((node, aid), &marker);
            }
            Event::Alias(id) => {
                let n = match self.anchor_map.get(&id) {
                    Some(v) => v.clone(),
                    None => Yaml::new(YamlElt::BadValue, &marker),
                };
                self.insert_new_node((n, 0), &marker);
            }
            _ => { /* ignore */ }
        }
        // println!("DOC {:?}", self.doc_stack);
    }
}

impl YamlLoader {
    fn insert_new_node(&mut self, node: (Yaml, usize), marker: &yaml_rust::scanner::Marker) {
        // valid anchor id starts from 1
        if node.1 > 0 {
            self.anchor_map.insert(node.1, node.0.clone());
        }
        if self.doc_stack.is_empty() {
            self.doc_stack.push(node);
        } else {
            let parent = self.doc_stack.last_mut().unwrap();
            match (*parent).0.yaml {
                YamlElt::Array(ref mut v) => v.push(node.0),
                YamlElt::Hash(ref mut h) => {
                    let cur_key = self.key_stack.last_mut().unwrap();
                    // current node is a key
                    if cur_key.is_badvalue() {
                        *cur_key = node.0;
                    // current node is a value
                    } else {
                        let mut newkey = Yaml::new(YamlElt::BadValue, marker);
                        mem::swap(&mut newkey, cur_key);
                        if let Some(stored_value) = h.get(&newkey) {
                            self.errors
                                .push(error::Error::DuplicateKey(error::DuplicateKey {
                                    key: newkey.yaml.clone(),
                                    first_mark: stored_value.marker,
                                    first_value: stored_value.yaml.clone(),
                                    second_mark: cur_key.marker,
                                    second_value: node.0.yaml.clone(),
                                }));
                        }
                        h.insert(newkey, node.0);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn load_from_str(source: &str) -> Result<YamlLoader, yaml_rust::scanner::ScanError> {
        let mut loader = YamlLoader {
            docs: Vec::new(),
            doc_stack: Vec::new(),
            key_stack: Vec::new(),
            anchor_map: BTreeMap::new(),
            errors: Vec::new(),
        };
        let mut parser = yaml_rust::parser::Parser::new(source.chars());
        parser.load(&mut loader, true)?;
        Ok(loader)
    }
}

fn parse_f64(v: &str) -> Option<f64> {
    match v {
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Some(f64::INFINITY),
        "-.inf" | "-.Inf" | "-.INF" => Some(f64::NEG_INFINITY),
        ".nan" | "NaN" | ".NAN" => Some(f64::NAN),
        _ => v.parse::<f64>().ok(),
    }
}

pub fn load_file(file_path: &std::path::Path) -> Result<YamlLoader> {
    let yaml_str = std::fs::read_to_string(file_path)?;
    let r = YamlLoader::load_from_str(&yaml_str)?;
    Ok(r)
}
