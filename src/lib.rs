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
//! use notabene::{lint, parse};
//! use notabene::changelog::traits::{Changelog, Changes, Unreleased};
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
//!
//! assert_eq!(parsed.title(), Some("Changelog".into()));
//! let unreleased_changes = parsed.unreleased().unwrap().changes();
//! assert_eq!(unreleased_changes[0].kind(), "Added");
//! assert_eq!(unreleased_changes[0].items().collect::<Vec<_>>(), vec!["Add foo"]);
//! ```
//!
//! ## Use a custom set of rules
//!
//! ```
//! use notabene::{Linter, Rule, RuleSet, parse};
//! use notabene::changelog::traits::{Changelog, Changes, Unreleased};
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
//!
//! assert_eq!(parsed.title(), Some("Changelog".into()));
//! let unreleased_changes = parsed.unreleased().unwrap().changes();
//! assert_eq!(unreleased_changes[0].kind(), "Added");
//! assert_eq!(unreleased_changes[0].items().collect::<Vec<_>>(), vec!["Add foo"]);
//! ```

pub(crate) mod ast;
pub(crate) mod diagnostic;
pub(crate) mod linter;
pub(crate) mod parser;
pub(crate) mod rule;
pub(crate) mod ruleset;
pub mod span;

pub mod changelog;
#[cfg(feature = "cli")]
pub mod cli;

pub use diagnostic::Diagnostic;
pub use linter::{Linter, lint};
pub use parser::parse;
pub use rule::Rule;
pub use ruleset::RuleSet;
pub use span::Locator;
