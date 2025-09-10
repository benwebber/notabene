use std::collections::HashMap;
use std::io::{self, Result, Write};
use std::path::PathBuf;

use clap::ArgMatches;

use crate::parse_file;
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
    let (_, diagnostics) = parse_file(&path).unwrap();
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
    } else if let Some(code) = matches.get_one::<String>("RULE") {
        let rule = rules_by_code.get(code).unwrap();
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
