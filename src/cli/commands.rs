use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::path::PathBuf;

use clap::ArgMatches;

use crate::profile::Profile;
use crate::rule::Rule;
use crate::span::Index;
use crate::{lint, parse};

use super::config::{Config, Lint};
use super::error::{Error, Result};
use super::report::{Format, report};

pub fn check(matches: &ArgMatches) -> Result<()> {
    let mut config = Config::load(None).unwrap();
    if let Some(path) = matches.get_one::<PathBuf>("config_file") {
        config = config.merge(&Config::from_file(path).unwrap());
    };
    let path = matches
        .get_one::<PathBuf>("FILE")
        .unwrap_or(&PathBuf::from("CHANGELOG.md"))
        .clone();
    let select: Option<HashSet<Rule>> = matches
        .get_many::<Rule>("select")
        .map(|values| values.copied().collect());
    let ignore: Option<HashSet<Rule>> = matches
        .get_many::<Rule>("ignore")
        .map(|values| values.copied().collect());
    let output_format = matches.get_one::<Format>("output_format").copied();
    let cli_config = Config {
        lint: Lint {
            select,
            ignore,
            output_format,
        },
    };
    config = config.merge(&cli_config);
    let rules = config
        .lint
        .select
        .unwrap()
        .difference(&config.lint.ignore.unwrap())
        .copied()
        .collect();
    let profile = Profile::new(rules);
    let content = std::fs::read_to_string(&path)?;
    let ir = parse(&content).unwrap();
    let diagnostics = lint(&ir, Some(&path), &profile);
    if diagnostics.is_empty() {
        Ok(())
    } else {
        let index = Index::new(&content);
        let mut output = io::stdout();
        report(
            &mut output,
            diagnostics.as_slice(),
            &content,
            Some(&path),
            &index,
            // TODO: Build final config.
            config.lint.output_format.unwrap(),
        )?;
        Err(Error::Check)
    }
}

pub fn rule(matches: &ArgMatches) -> Result<()> {
    let mut rules_by_code = HashMap::new();
    for rule in Rule::ALL {
        rules_by_code.insert(rule.code().to_string(), rule);
    }
    let mut output = io::stdout();
    if matches.get_flag("all") {
        for (i, rule) in Rule::ALL.iter().enumerate() {
            if i > 0 {
                writeln!(output)?;
            }
            write!(
                output,
                "# {}

{}
",
                rule.code(),
                rule.doc()
            )?;
        }
    } else if let Some(rule) = matches.get_one::<Rule>("RULE") {
        write!(
            output,
            "# {}

{}
",
            rule.code(),
            rule.doc()
        )?
    } else {
        unreachable!()
    }
    Ok(())
}
