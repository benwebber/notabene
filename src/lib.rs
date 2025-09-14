//! A fast linter for changelogs in the Keep a Changelog format.
//!
//! # Example
//!
//! ```
//! use notabene::{Changelog, RuleSet, lint, parse};
//!
//! let s = r#"
//! ## Changelog
//!
//! ### [Unreleased]
//!
//! #### Added
//!
//! * Add foo
//!
//! [Unreleased]: https://example.org/
//! "#;
//! let parsed = parse(&s).unwrap();
//! let diagnostics = lint(&parsed, None, &RuleSet::default());
//! let changelog: Changelog = parsed.into();
//!
//! assert_eq!(changelog.title, Some("Changelog".into()));
//! let unreleased_changes = changelog.unreleased.unwrap().changes;
//! assert_eq!(unreleased_changes[0].kind, "Added".to_string());
//! assert_eq!(unreleased_changes[0].changes, vec!["Add foo".to_string()]);
//! ```

use std::path::Path;

pub(crate) mod ast;
pub(crate) mod diagnostic;
pub mod ir;
pub(crate) mod linter;
pub(crate) mod parser;
pub(crate) mod rule;
pub(crate) mod ruleset;
pub(crate) mod span;
pub(crate) mod unist;

pub mod changelog;
#[cfg(feature = "cli")]
pub mod cli;
pub mod error;

pub use changelog::Changelog;
pub use diagnostic::Diagnostic;
pub use error::ParseError;
pub use rule::Rule;
pub use ruleset::RuleSet;
pub use span::Span;

/// Parse a changelog into its intermediate representation.
///
/// Parsing a changelog will always succeed. This function returns a [`Result`] to support future
/// fatal errors (e.g. if the document is not a changelog at all).
pub fn parse<'a>(s: &'a str) -> Result<ir::Changelog<'a>, ParseError> {
    Ok(parser::parse(s))
}

/// Lint a changelog in its intermediate representation.
pub fn lint<'a>(
    changelog: &ir::Changelog<'a>,
    path: Option<&Path>,
    ruleset: &RuleSet,
) -> Vec<Diagnostic> {
    let mut diagnostics = linter::lint(&changelog, ruleset);
    diagnostics.sort_by_key(|d| d.span);
    for diagnostic in &mut diagnostics {
        diagnostic.path = path.map(|p| p.to_path_buf());
    }
    diagnostics
}
