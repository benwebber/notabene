use std::collections::{HashMap, HashSet};
use std::io::{self, Result, Write};
use std::path::PathBuf;

use clap::ArgMatches;

use crate::parse_and_lint_file;
use crate::profile::Profile;
use crate::rule::Rule;
use crate::span::Index;

use super::renderer::{OutputFormat, render};

pub fn check(matches: &ArgMatches) -> Result<()> {
    let path = matches
        .get_one::<PathBuf>("FILE")
        .unwrap_or(&PathBuf::from("CHANGELOG.md"))
        .clone();
    let format: &String = matches.get_one("output_format").expect("default");
    let output_format = match format.as_str() {
        "short" => OutputFormat::Short,
        "json" => OutputFormat::Json,
        "jsonl" => OutputFormat::JsonLines,
        "full" => OutputFormat::Full,
        _ => unreachable!(),
    };
    let selected: HashSet<Rule> = match matches.get_many::<Rule>("select") {
        Some(values) => values.copied().collect(),
        None => Rule::ALL.iter().copied().collect(),
    };
    let ignored: HashSet<Rule> = match matches.get_many::<Rule>("ignore") {
        Some(values) => values.copied().collect(),
        None => HashSet::new(),
    };
    let rules: HashSet<Rule> = selected.difference(&ignored).copied().collect();
    let profile = Profile::new(rules);
    let (_, diagnostics) = parse_and_lint_file(&path, &profile).unwrap();
    if diagnostics.is_empty() {
        Ok(())
    } else {
        // TODO: How to avoid reading again to create index?
        let Ok(content) = std::fs::read_to_string(&path) else {
            todo!();
        };
        let index = Index::new(&content);
        let mut output = io::stdout();
        render(
            &mut output,
            diagnostics.as_slice(),
            &content,
            Some(&path),
            &index,
            output_format,
        )
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
