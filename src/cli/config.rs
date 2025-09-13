use std::collections::HashSet;

use serde::Deserialize;
use toml;
use toml::de;

use crate::rule::Rule;

use super::renderer::OutputFormat;

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    lint: LintConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
struct LintConfig {
    #[serde(default = "default_select")]
    select: HashSet<Rule>,
    #[serde(default = "default_ignore")]
    ignore: HashSet<Rule>,
    #[serde(default = "default_output_format")]
    output_format: OutputFormat,
}

impl Config {
    fn merge(&self, other: &Config) -> Self {
        Self {
            lint: self.lint.merge(&other.lint),
        }
    }

    fn from_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
}

impl LintConfig {
    fn merge(&self, other: &LintConfig) -> Self {
        Self {
            select: self.select.union(&other.select).copied().collect(),
            ignore: self.ignore.union(&other.ignore).copied().collect(),
            output_format: other.output_format,
        }
    }
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            select: default_select(),
            ignore: default_ignore(),
            output_format: default_output_format(),
        }
    }
}

fn default_select() -> HashSet<Rule> {
    HashSet::from(Rule::ALL)
}

fn default_ignore() -> HashSet<Rule> {
    HashSet::new()
}

fn default_output_format() -> OutputFormat {
    OutputFormat::Short
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let a = Config {
            lint: LintConfig::default(),
        };
        let b = Config {
            lint: LintConfig {
                select: HashSet::from([Rule::MissingTitle]),
                ignore: HashSet::from([Rule::InvalidTitle]),
                output_format: OutputFormat::Json,
            },
        };
        assert_eq!(
            a.merge(&b),
            Config {
                lint: LintConfig {
                    select: HashSet::from(Rule::ALL),
                    ignore: HashSet::from([Rule::InvalidTitle]),
                    output_format: OutputFormat::Json,
                }
            }
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            Config::from_str(
                r#"
            [lint]
            select = ["E001"]
            output_format = "json"
        "#
            ),
            Ok(Config {
                lint: LintConfig {
                    select: HashSet::from([Rule::MissingTitle]),
                    output_format: OutputFormat::Json,
                    ..Default::default()
                }
            })
        )
    }
}
