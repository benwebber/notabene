use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[cfg(feature = "cli")]
use clap::builder::Styles;
use clap::builder::ValueParser;
#[cfg(feature = "cli")]
use clap::{Arg, Command, builder::ArgGroup, value_parser};

use notabene::span::Index;
#[cfg(feature = "cli")]
use notabene::{OutputFormat, Rule, parse_file, render};

fn parse_rule_code(code: &str) -> Result<String, String> {
    let code = code.to_uppercase();
    if Rule::ALL.iter().any(|rule| rule.code() == code) {
        Ok(code)
    } else {
        Err(format!(
            "{}",
            Rule::ALL
                .iter()
                .map(|rule| rule.code())
                .collect::<Vec<&str>>()
                .join(", ")
        ))
    }
}

#[cfg(feature = "cli")]
fn main() -> std::process::ExitCode {
    let mut rules_by_code = HashMap::new();
    for rule in Rule::ALL {
        rules_by_code.insert(rule.code().to_string(), rule);
    }
    let matches = Command::new("nb")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .styles(Styles::plain())
        .subcommand(
            Command::new("check")
                .about("Check a changelog")
                .arg(Arg::new("FILE").value_parser(value_parser!(PathBuf)))
                .arg(
                    Arg::new("output_format")
                        .long("output-format")
                        .value_parser(["full", "json", "jsonl", "short"])
                        .default_value("short"),
                ),
        )
        .subcommand(
            Command::new("rule")
                .about("Explain a rule")
                .group(ArgGroup::new("rule").required(true).args(["RULE", "all"]))
                .arg(
                    Arg::new("RULE")
                        .help("The rule code (e.g., E100)")
                        .value_parser(ValueParser::new(parse_rule_code))
                        .conflicts_with("all"),
                )
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Explain all rules")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("check", submatches)) => {
            let path = submatches
                .get_one::<PathBuf>("FILE")
                .unwrap_or(&PathBuf::from("CHANGELOG.md"))
                .clone();
            let format: &String = submatches.get_one("output_format").expect("default");
            let output_format = match format.as_str() {
                "short" => OutputFormat::Short,
                "json" => OutputFormat::Json,
                "jsonl" => OutputFormat::JsonLines,
                "full" => OutputFormat::Full,
                _ => unreachable!(),
            };
            let (_, diagnostics) = parse_file(&path).unwrap();
            if diagnostics.is_empty() {
                ExitCode::SUCCESS
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
                .unwrap();
                ExitCode::FAILURE
            }
        }
        Some(("rule", submatches)) => {
            let mut output = io::stdout();
            if submatches.get_flag("all") {
                for (i, rule) in Rule::ALL.iter().enumerate() {
                    if i > 0 {
                        writeln!(output).unwrap();
                    }
                    write!(
                        output,
                        "# {}

{}
",
                        rule.code(),
                        rule.doc()
                    )
                    .unwrap();
                }
                ExitCode::SUCCESS
            } else if let Some(code) = submatches.get_one::<String>("RULE") {
                let rule = rules_by_code.get(code).unwrap();
                write!(
                    output,
                    "# {}

{}
",
                    rule.code(),
                    rule.doc()
                )
                .unwrap();
                ExitCode::SUCCESS
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}
