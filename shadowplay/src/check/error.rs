use std::fmt::Display;

use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum OutputFormat {
    OneLine,
    Json,
}

impl OutputFormat {
    pub fn is_human(&self) -> bool {
        match self {
            OutputFormat::OneLine => true,
            OutputFormat::Json => false,
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct Range {
    path: std::path::PathBuf,
    start: Location,
    end: Location,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)?;
        write!(f, "{}", self.start)?;
        Ok(())
    }
}

impl<'a> From<(&std::path::Path, &puppet_parser::Span<'a>)> for Range {
    fn from(pair: (&std::path::Path, &puppet_parser::Span)) -> Self {
        let (path, span) = pair;
        Range {
            path: std::path::PathBuf::from(path),
            start: Location {
                line: Some(span.location_line() as usize),
                column: Some(span.get_utf8_column()),
                index: Some(span.location_offset()),
            },
            end: Location {
                line: None,
                column: None,
                index: None,
            },
        }
    }
}

impl<'a> From<(&std::path::Path, &puppet_parser::range::Range)> for Range {
    fn from(pair: (&std::path::Path, &puppet_parser::range::Range)) -> Self {
        let (path, range) = pair;
        Range {
            path: std::path::PathBuf::from(path),
            start: Location {
                line: Some(range.start().line() as usize),
                column: Some(range.start().column() as usize),
                index: Some(range.start().offset() as usize),
            },
            end: Location {
                line: Some(range.end().line() as usize),
                column: Some(range.end().column() as usize),
                index: Some(range.end().offset() as usize),
            },
        }
    }
}

impl<'a> From<(&std::path::Path, &located_yaml::Marker)> for Range {
    fn from(pair: (&std::path::Path, &located_yaml::Marker)) -> Self {
        let (path, location) = pair;
        Range {
            path: std::path::PathBuf::from(path),
            start: Location {
                line: Some(location.line as usize),
                column: Some(location.col as usize),
                index: Some(location.index as usize),
            },
            end: Location {
                line: None,
                column: None,
                index: None,
            },
        }
    }
}

impl<'a> From<(&std::path::Path, &yaml_rust::scanner::Marker)> for Range {
    fn from(pair: (&std::path::Path, &yaml_rust::scanner::Marker)) -> Self {
        let (path, location) = pair;
        Range {
            path: std::path::PathBuf::from(path),
            start: Location {
                line: Some(location.line() as usize),
                column: Some(location.col() as usize),
                index: Some(location.index() as usize),
            },
            end: Location {
                line: None,
                column: None,
                index: None,
            },
        }
    }
}

impl<'a> From<&std::path::Path> for Range {
    fn from(path: &std::path::Path) -> Self {
        Range {
            path: std::path::PathBuf::from(path),
            start: Location {
                line: None,
                column: None,
                index: None,
            },
            end: Location {
                line: None,
                column: None,
                index: None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub error_type: Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub range: Range,
}

impl<'a> From<(&std::path::Path, &puppet_parser::ParseError<'a>)> for Error {
    fn from(pair: (&std::path::Path, &puppet_parser::ParseError)) -> Self {
        let (path, parse_error) = pair;
        Self {
            error_type: Type::ManifestSyntax,
            message: parse_error.message().clone(),
            url: parse_error.url().clone(),
            range: Range::from((path, parse_error.span())),
            error_subtype: None,
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
            range: Range::from((path, &lint_error.location)),
            error_subtype: Some(lint_error.linter.name().to_string()),
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
            range: Range::from((path, &yaml_error.mark())),
            error_subtype: None,
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
            range: Range::from((path, marker)),
            error_subtype: None,
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
            range: Range::from((path, yaml_error.marker())),
            error_subtype: Some("scan_error".to_string()),
        }
    }
}

impl Error {
    pub fn of_file(path: &std::path::Path, error_type: Type, message: &str) -> Self {
        Self {
            error_type,
            message: Some(message.to_string()),
            url: None,
            range: Range::from(path),
            error_subtype: None,
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
            self.error_type, self.range, message, url
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
