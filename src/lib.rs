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
//! let parsed = parse(&s);
//! let diagnostics = lint(&parsed, &RuleSet::default(), None);
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

pub use changelog::Changelog;
pub use diagnostic::Diagnostic;
pub use linter::lint;
pub use parser::parse;
pub use rule::Rule;
pub use ruleset::RuleSet;
pub use span::Span;
