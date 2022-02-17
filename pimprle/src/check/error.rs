use std::fmt::Display;

use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum OutputFormat {
    OneLine,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::OneLine
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one-line" => Ok(Self::OneLine),
            "json" => Ok(Self::Json),
            _ => anyhow::bail!("Invalid format: {}", s),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Type {
    FileError,
    Yaml,
    Hiera,
    ManifestSyntax,
    ManifestLint,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::FileError => write!(f, "File"),
            Type::Yaml => write!(f, "YAML"),
            Type::Hiera => write!(f, "Hiera"),
            Type::ManifestSyntax => write!(f, "Puppet manifest syntax"),
            Type::ManifestLint => write!(f, "Puppet manifest lint"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Location {
    path: std::path::PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)?;
        if self.line.is_some() || self.column.is_some() {
            write!(f, " at")?;
            if let Some(line) = self.line {
                write!(f, " line {}", line)?;
            }
            if let Some(col) = self.column {
                write!(f, " column {}", col)?;
            }
        }
        Ok(())
    }
}

impl<'a> From<(&std::path::Path, &puppet_parser::parser::Span<'a>)> for Location {
    fn from(pair: (&std::path::Path, &puppet_parser::parser::Span)) -> Self {
        let (path, span) = pair;
        Location {
            path: std::path::PathBuf::from(path),
            line: Some(span.location_line() as usize),
            column: Some(span.get_utf8_column()),
            index: Some(span.location_offset()),
        }
    }
}

impl<'a> From<(&std::path::Path, &puppet_parser::parser::Location)> for Location {
    fn from(pair: (&std::path::Path, &puppet_parser::parser::Location)) -> Self {
        let (path, location) = pair;
        Location {
            path: std::path::PathBuf::from(path),
            line: Some(location.line() as usize),
            column: Some(location.column() as usize),
            index: Some(location.offset() as usize),
        }
    }
}

impl<'a> From<(&std::path::Path, &located_yaml::Marker)> for Location {
    fn from(pair: (&std::path::Path, &located_yaml::Marker)) -> Self {
        let (path, location) = pair;
        Location {
            path: std::path::PathBuf::from(path),
            line: Some(location.line as usize),
            column: Some(location.col as usize),
            index: Some(location.index as usize),
        }
    }
}

impl<'a> From<(&std::path::Path, &yaml_rust::scanner::Marker)> for Location {
    fn from(pair: (&std::path::Path, &yaml_rust::scanner::Marker)) -> Self {
        let (path, location) = pair;
        Location {
            path: std::path::PathBuf::from(path),
            line: Some(location.line() as usize),
            column: Some(location.col() as usize),
            index: Some(location.index() as usize),
        }
    }
}

impl<'a> From<&std::path::Path> for Location {
    fn from(path: &std::path::Path) -> Self {
        Location {
            path: std::path::PathBuf::from(path),
            line: None,
            column: None,
            index: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub error_type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub location: Location,
}

impl<'a> From<(&std::path::Path, &puppet_parser::parser::ParseError<'a>)> for Error {
    fn from(pair: (&std::path::Path, &puppet_parser::parser::ParseError)) -> Self {
        let (path, parse_error) = pair;
        Self {
            error_type: Type::ManifestSyntax,
            message: parse_error.message().clone(),
            url: parse_error.url().clone(),
            location: Location::from((path, parse_error.span())),
        }
    }
}

impl From<(&std::path::Path, &puppet_pp_lint::lint::LintError)> for Error {
    fn from(pair: (&std::path::Path, &puppet_pp_lint::lint::LintError)) -> Self {
        let (path, lint_error) = pair;
        Self {
            error_type: Type::ManifestLint,
            message: Some(lint_error.message.clone()),
            url: lint_error.url.clone(),
            location: Location::from((path, &lint_error.location)),
        }
    }
}

impl From<(&std::path::Path, &located_yaml::error::Error)> for Error {
    fn from(pair: (&std::path::Path, &located_yaml::error::Error)) -> Self {
        let (path, yaml_error) = pair;
        Self {
            error_type: Type::Yaml,
            message: Some(yaml_error.to_string()),
            url: None,
            location: Location::from((path, &yaml_error.mark())),
        }
    }
}

impl From<(&std::path::Path, Type, &str, &located_yaml::Marker)> for Error {
    fn from(tuple: (&std::path::Path, Type, &str, &located_yaml::Marker)) -> Self {
        let (path, error_type, message, marker) = tuple;
        Self {
            error_type,
            message: Some(message.to_string()),
            url: None,
            location: Location::from((path, marker)),
        }
    }
}

impl From<(&std::path::Path, &yaml_rust::scanner::ScanError)> for Error {
    fn from(pair: (&std::path::Path, &yaml_rust::scanner::ScanError)) -> Self {
        let (path, yaml_error) = pair;
        Self {
            error_type: Type::Yaml,
            message: Some(yaml_error.to_string()),
            url: None,
            location: Location::from((path, yaml_error.marker())),
        }
    }
}

impl Error {
    pub fn of_file(path: &std::path::Path, error_type: Type, message: &str) -> Self {
        Self {
            error_type,
            message: Some(message.to_string()),
            url: None,
            location: Location::from(path),
        }
    }

    pub fn one_line(&self) -> String {
        let url = match &self.url {
            None => "".to_owned(),
            Some(v) => format!(" // See {}", v),
        };
        let message = match &self.message {
            Some(message) => message,
            None => "(no error message)",
        };
        format!(
            "{} error in {}: {}{}",
            self.error_type, self.location, message, url
        )
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn output(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::OneLine => self.one_line(),
            OutputFormat::Json => self.as_json(),
        }
    }
}
