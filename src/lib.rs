//! A fast linter for changelogs in the Keep a Changelog format.
//!
//! # Examples
//!
//! The examples below use this minimal changelog.
//!
//! ```markdown
//! # Changelog
//!
//! ## [Unreleased]
//!
//! ### Added
//!
//! * Add foo
//!
//! [Unreleased]: https://example.org/
//! ```
//!
//! ## Use the default rules
//!
//! ```
//! use notabene::{Changelog, lint, parse};
//!
//! # let s = r#"
//! #
//! # # Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! let parsed = parse(&s);
//! let diagnostics = lint(&parsed);
//! let changelog: Changelog = parsed.into();
//!
//! assert_eq!(changelog.title, Some("Changelog".into()));
//! let unreleased_changes = changelog.unreleased.unwrap().changes;
//! assert_eq!(unreleased_changes[0].kind, "Added".to_string());
//! assert_eq!(unreleased_changes[0].changes, vec!["Add foo".to_string()]);
//! ```
//!
//! ## Use a custom set of rules
//!
//! ```
//! use notabene::{Changelog, Linter, Rule, RuleSet, parse};
//!
//! # let s = r#"
//! #
//! # # Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! let parsed = parse(&s);
//! // A new RuleSet with one rule.
//! let ruleset = RuleSet::from([Rule::MissingTitle]);
//! // Configure a custom linter.
//! // `with_filename()` will set the filename reported in diagnostics.
//! let linter = Linter::new(&ruleset);
//! let diagnostics = linter.lint(&parsed);
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
pub mod location;
pub(crate) mod parser;
pub(crate) mod rule;
pub(crate) mod ruleset;
pub(crate) mod span;

pub mod changelog;
#[cfg(feature = "cli")]
pub mod cli;

pub use changelog::Changelog;
pub use diagnostic::Diagnostic;
pub use linter::{Linter, lint};
pub use location::Locator;
pub use parser::parse;
pub use rule::Rule;
pub use ruleset::RuleSet;
pub use span::Span;
