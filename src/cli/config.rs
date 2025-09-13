use std::collections::HashSet;
use std::path::Path;

use serde::{Deserialize, Serialize};
use toml;
use toml::de;

use crate::rule::Rule;

use super::error::Result;
use super::renderer::OutputFormat;

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    // When deserializing from a file, we want to consider empty values unset instead of their
    // default values.
    #[serde(default = "Lint::empty")]
    pub lint: Lint,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Lint {
    #[serde(default = "default_select")]
    pub select: Option<HashSet<Rule>>,
    pub ignore: Option<HashSet<Rule>>,
    pub output_format: Option<OutputFormat>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct PyProjectConfig {
    tool: PyProjectTool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct PyProjectTool {
    nb: Config,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Self {
            lint: Lint::empty(),
        }
    }

    pub fn merge(&self, other: &Config) -> Self {
        Self {
            lint: self.lint.merge(&other.lint),
        }
    }

    // Load default configuration
    pub fn load(path: Option<&Path>) -> Result<Config> {
        let mut config = Config::default();
        // TODO: Add debug logging for I/O error.
        if let Ok(s) = std::fs::read_to_string("pyproject.toml") {
            let pyproject = PyProjectConfig::from_str(&s)?;
            config = config.merge(&pyproject.tool.nb);
        }
        if let Ok(s) = std::fs::read_to_string("nb.toml") {
            let other = Config::from_str(&s)?;
            config = config.merge(&Config::from_str(&s)?);
        }
        // Return an error if opening the user-specified file fails.
        if let Some(path) = path {
            let s = std::fs::read_to_string(path)?;
            let other = Config::from_str(&s)?;
            config = config.merge(&other);
        }
        Ok(config)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;
        Self::from_str(&s)
    }
}

impl PyProjectConfig {
    pub fn from_str(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }
}

impl Lint {
    pub fn empty() -> Self {
        Self {
            select: Default::default(),
            ignore: Default::default(),
            output_format: Default::default(),
        }
    }

    pub fn merge(&self, other: &Lint) -> Self {
        Self {
            select: other.select.clone().or(self.select.clone()),
            ignore: other.ignore.clone().or(self.ignore.clone()),
            output_format: other.output_format.clone().or(self.output_format.clone()),
        }
    }
}

impl Default for Lint {
    fn default() -> Self {
        Self {
            select: default_select(),
            ignore: Some(Default::default()),
            output_format: Some(Default::default()),
        }
    }
}

fn default_select() -> Option<HashSet<Rule>> {
    Some(HashSet::from(Rule::ALL))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            Config::default(),
            Config {
                lint: Lint {
                    select: Some(HashSet::from(Rule::ALL)),
                    ignore: Some(HashSet::new()),
                    output_format: Some(OutputFormat::Short),
                }
            },
        );
    }

    #[test]
    fn test_merge() {
        let empty = Config::empty();
        let default = Config::default();
        assert_eq!(empty.merge(&default), default);
        assert_eq!(default.merge(&empty), default);
        let user = Config {
            lint: Lint {
                select: Some(HashSet::from([Rule::MissingTitle])),
                ignore: Some(HashSet::from([Rule::InvalidTitle])),
                output_format: Some(OutputFormat::Json),
            },
        };
        assert_eq!(default.merge(&user), user);
    }

    #[test]
    fn test_from_str() {
        let config = Config::from_str("").unwrap();
        assert_eq!(config, Config::empty());

        let config = Config::from_str(
            r#"
            [lint]
            select = ["E001"]
            ignore = ["E002"]
            output_format = "json"
        "#,
        )
        .unwrap();
        assert_eq!(
            config,
            Config {
                lint: Lint {
                    select: Some(HashSet::from([Rule::MissingTitle])),
                    ignore: Some(HashSet::from([Rule::InvalidTitle])),
                    output_format: Some(OutputFormat::Json),
                }
            }
        );
    }
}
