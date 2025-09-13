use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::path::Path;

use owo_colors::{OwoColorize, Stream};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::diagnostic::Diagnostic;
use crate::span::Index;
use crate::unist::Position;

// TODO: Limit allocations.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonDiagnostic {
    pub code: String,
    pub position: Option<Position>,
    pub path: Option<String>,
    pub message: String,
}

/// A render output format.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum OutputFormat {
    /// One-line output.
    Short,
    /// Multi-line output with context.
    Full,
    /// JSON output.
    Json,
    /// JSON Lines output.
    JsonLines,
}

impl<'de> Deserialize<'de> for OutputFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutputFormatVisitor;

        impl<'de> Visitor<'de> for OutputFormatVisitor {
            type Value = OutputFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an output format")
            }

            fn visit_str<E>(self, value: &str) -> Result<OutputFormat, E>
            where
                E: de::Error,
            {
                match value {
                    "short" => Ok(OutputFormat::Short),
                    "full" => Ok(OutputFormat::Full),
                    "json" => Ok(OutputFormat::Json),
                    "jsonl" => Ok(OutputFormat::JsonLines),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["short", "full", "json", "jsonl"],
                    )),
                }
            }
        }

        deserializer.deserialize_str(OutputFormatVisitor)
    }
}

/// Render diagnostics in a given output format.
///
/// This function  is public so that the `nb` binary can use it. Do not depend on it for your own use.
pub fn render<W: IoWrite>(
    w: &mut W,
    diagnostics: &[Diagnostic],
    source: &str,
    path: Option<&Path>,
    index: &Index,
    format: OutputFormat,
) -> std::io::Result<()> {
    match format {
        OutputFormat::Short => {
            for diagnostic in diagnostics {
                render_simple(w, source, index, diagnostic)?;
            }
        }
        OutputFormat::Full => {
            let mut buf = String::new();
            for diagnostic in diagnostics {
                render_simple(w, source, index, diagnostic)?;
                if let Some(p) = diagnostic.position(index) {
                    // The diagnostic has context. Render it like:
                    //     |
                    //  99 |
                    // 100 | # Changelog
                    //     | ^ E000
                    // 101 |
                    //     |
                    //     = help:

                    // Previous, current, next.
                    let lines = [
                        p.start.line.saturating_sub(1),
                        p.start.line,
                        p.start.line + 1,
                    ];
                    let gutter_width =
                        (lines.iter().max().unwrap().checked_ilog10().unwrap_or(0) + 1) as usize;
                    let gutter =
                        format!("{:>width$}{}", " ", "|".dimmed(), width = gutter_width + 1);
                    //     |
                    writeln!(&mut buf, "{}", gutter).unwrap();
                    // 99  |
                    if lines[0] > 0 {
                        // TODO: Make the index return None if the line number is 0.
                        writeln!(
                            &mut buf,
                            "{:>width$} {} {}",
                            format!("{}", lines[0]).dimmed(),
                            "|".dimmed(),
                            index.line(lines[0]),
                            width = gutter_width
                        )
                        .unwrap();
                    }
                    // 100 |
                    writeln!(
                        &mut buf,
                        "{:>width$} {} {}",
                        format!("{}", lines[1]).dimmed(),
                        "|".dimmed(),
                        index.line(lines[1]),
                        width = gutter_width
                    )
                    .unwrap();
                    //     | ^E000
                    write!(&mut buf, "{}", gutter).unwrap();
                    buf.push_str(&" ".repeat(p.start.column));
                    writeln!(
                        &mut buf,
                        "{} {}",
                        "^".if_supports_color(Stream::Stdout, |text| text.bright_red())
                            .bold(),
                        diagnostic
                            .rule
                            .code()
                            .if_supports_color(Stream::Stdout, |text| text.bright_red())
                            .bold()
                    )
                    .unwrap();
                    // 101 |
                    if lines[2] < index.lines() {
                        // TODO: Make the index return None if the line number is 0.
                        writeln!(
                            &mut buf,
                            "{:>width$} {} {}",
                            format!("{}", lines[2]).dimmed(),
                            "|".dimmed(),
                            index.line(lines[2]),
                            width = gutter_width
                        )
                        .unwrap();
                    }
                    writeln!(&mut buf, "{}", gutter).unwrap();
                    writeln!(w, "{}", std::mem::take(&mut buf))?;
                }
            }
        }
        OutputFormat::Json | OutputFormat::JsonLines => {
            let json_diagnostics: Vec<JsonDiagnostic> = diagnostics
                .iter()
                .map(|diagnostic| JsonDiagnostic {
                    code: diagnostic.code().to_string(),
                    position: diagnostic.position(index),
                    path: path.map(|p| p.to_string_lossy().to_string()),
                    message: diagnostic.message(source),
                })
                .collect();
            if matches!(format, OutputFormat::Json) {
                serde_json::to_writer(&mut *w, &json_diagnostics)?;
            } else {
                for diagnostic in json_diagnostics {
                    serde_json::to_writer(&mut *w, &diagnostic)?;
                    writeln!(w)?;
                }
            }
        }
    }
    Ok(())
}

pub fn render_simple<W: IoWrite>(
    w: &mut W,
    source: &str,
    index: &Index,
    diagnostic: &Diagnostic,
) -> std::io::Result<()> {
    let line = diagnostic
        .position(index)
        .map(|p| p.start.line)
        .unwrap_or(1);
    let column = diagnostic
        .position(index)
        .map(|p| p.start.column)
        .unwrap_or(1);
    write!(
        w,
        "{}:{}:{}: ",
        diagnostic
            .path
            .clone()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or("".to_string()),
        line,
        column
    )?;
    write!(
        w,
        "{}",
        diagnostic
            .rule
            .code()
            .if_supports_color(Stream::Stdout, |text| text.bright_red())
    )?;
    writeln!(w, " {}", diagnostic.message(source))
}
