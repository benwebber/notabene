use std::fmt::Write as FmtWrite;
use std::io::Write;

use owo_colors::{OwoColorize, Stream};
use serde::Serialize;

use crate::diagnostic::Diagnostic;
use crate::span::Position;

use super::Context;

pub trait Formatter {
    fn format(
        &self,
        w: &mut dyn Write,
        diagnostics: &[Diagnostic<Position>],
        context: &Context,
    ) -> std::io::Result<()>;
}

pub struct ShortFormatter;
pub struct FullFormatter;
pub struct JsonFormatter;
pub struct JsonLinesFormatter;

#[derive(Clone, Debug, Serialize)]
pub struct JsonDiagnostic {
    pub code: String,
    pub position: Option<Position>,
    pub path: Option<String>,
    pub message: String,
}

impl Formatter for ShortFormatter {
    fn format(
        &self,
        w: &mut dyn Write,
        diagnostics: &[Diagnostic<Position>],
        context: &Context,
    ) -> std::io::Result<()> {
        for diagnostic in diagnostics {
            render_simple(w, diagnostic, context)?;
        }
        Ok(())
    }
}

impl Formatter for FullFormatter {
    fn format(
        &self,
        w: &mut dyn Write,
        diagnostics: &[Diagnostic<Position>],
        context: &Context,
    ) -> std::io::Result<()> {
        let mut buf = String::new();
        for diagnostic in diagnostics {
            render_simple(w, diagnostic, context)?;
            if let Some(p) = diagnostic.position(context.locator) {
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
                let gutter = format!("{:>width$}{}", " ", "|".dimmed(), width = gutter_width + 1);
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
                        context.locator.line(lines[0]),
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
                    context.locator.line(lines[1]),
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
                if lines[2] < context.locator.lines() {
                    // TODO: Make the index return None if the line number is 0.
                    writeln!(
                        &mut buf,
                        "{:>width$} {} {}",
                        format!("{}", lines[2]).dimmed(),
                        "|".dimmed(),
                        context.locator.line(lines[2]),
                        width = gutter_width
                    )
                    .unwrap();
                }
                writeln!(&mut buf, "{}", gutter).unwrap();
                writeln!(w, "{}", std::mem::take(&mut buf))?;
            }
        }
        Ok(())
    }
}

impl Formatter for JsonFormatter {
    fn format(
        &self,
        w: &mut dyn Write,
        diagnostics: &[Diagnostic<Position>],
        context: &Context,
    ) -> std::io::Result<()> {
        let json_diagnostics: Vec<JsonDiagnostic> = diagnostics
            .iter()
            .map(|diagnostic| JsonDiagnostic {
                code: diagnostic.code().to_string(),
                position: diagnostic.position(context.locator),
                path: context.path.map(|p| p.to_string_lossy().to_string()),
                message: diagnostic.message(context.source),
            })
            .collect();
        Ok(serde_json::to_writer(&mut *w, &json_diagnostics)?)
    }
}

impl Formatter for JsonLinesFormatter {
    fn format(
        &self,
        w: &mut dyn Write,
        diagnostics: &[Diagnostic<Position>],
        context: &Context,
    ) -> std::io::Result<()> {
        let json_diagnostics: Vec<JsonDiagnostic> = diagnostics
            .iter()
            .map(|diagnostic| JsonDiagnostic {
                code: diagnostic.code().to_string(),
                position: diagnostic.position(context.locator),
                path: context.path.map(|p| p.to_string_lossy().to_string()),
                message: diagnostic.message(context.source),
            })
            .collect();
        for diagnostic in json_diagnostics {
            serde_json::to_writer(&mut *w, &diagnostic)?;
            writeln!(w)?;
        }
        Ok(())
    }
}

pub fn render_simple(
    w: &mut dyn Write,
    diagnostic: &Diagnostic<Position>,
    context: &Context,
) -> std::io::Result<()> {
    let line = diagnostic
        .position(context.locator)
        .map(|p| p.start.line)
        .unwrap_or(1);
    let column = diagnostic
        .position(context.locator)
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
    writeln!(w, " {}", diagnostic.message(context.source))
}
