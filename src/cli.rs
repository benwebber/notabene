use std::io::Result as IoResult;
use std::path::PathBuf;

use clap::builder::{Styles, ValueParser};
use clap::{Arg, ArgGroup, Command, value_parser};

use crate::rule::Rule;

mod commands;
mod renderer;

pub fn main() -> IoResult<()> {
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
        Some(("check", submatches)) => commands::check(submatches),
        Some(("rule", submatches)) => commands::rule(submatches),
        _ => unreachable!(),
    }
}

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
