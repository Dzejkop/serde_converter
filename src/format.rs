use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Json,
    Yaml,
    Ron,
    Toml,
    Csv,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "json" => Format::Json,
            "yaml" => Format::Yaml,
            "ron" => Format::Ron,
            "toml" => Format::Toml,
            "csv" => Format::Csv,
            _ => return Err("".to_string()),
        })
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Json => write!(f, "json"),
            Format::Yaml => write!(f, "yaml"),
            Format::Ron => write!(f, "ron"),
            Format::Toml => write!(f, "toml"),
            Format::Csv => write!(f, "csv"),
        }
    }
}
