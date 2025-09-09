use std::path::Path;

pub(crate) mod ast;
pub(crate) mod diagnostic;
pub(crate) mod ir;
pub(crate) mod linter;
pub(crate) mod parser;
#[cfg(feature = "cli")]
pub(crate) mod renderer;
pub(crate) mod rule;

pub mod changelog;
pub mod error;
pub mod span;
pub mod unist;

pub use changelog::Changelog;
pub use diagnostic::Diagnostic;
pub use error::{Error, ParseError};
pub use rule::Rule;

#[cfg(feature = "cli")]
pub use renderer::{OutputFormat, render};

/// Parse a changelog from a string.
///
/// Parsing a changelog will nearly always succeed. This function returns a [`Result`] to support
/// future fatal errors (e.g. if the document is not a changelog at all).
pub fn parse_str(s: &str) -> Result<(Changelog, Vec<Diagnostic>), ParseError> {
    parse(s, None)
}

/// Parse a changelog from a file.
///
/// As with [`parse_str()`], parsing the changelog document will nearly always succeed.
/// `parse_file()` may additionally return a [`std::io::Error`] ([`Error::Io`]).
pub fn parse_file(path: &Path) -> Result<(Changelog, Vec<Diagnostic>), Error> {
    let s = std::fs::read_to_string(path)?;
    Ok(parse(&s, Some(path))?)
}

fn parse(s: &str, path: Option<&Path>) -> Result<(Changelog, Vec<Diagnostic>), ParseError> {
    let (changelog, mut diagnostics) = parser::parse(s);
    diagnostics.append(&mut linter::lint(&changelog));
    diagnostics.sort_by_key(|d| d.span);
    for diagnostic in &mut diagnostics {
        diagnostic.path = path.map(|p| p.to_path_buf());
    }
    Ok((changelog.into(), diagnostics))
}
