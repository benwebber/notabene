//! A fast linter for changelogs in the Keep a Changelog format.
//!
//! ```rust
//! use notabene::parse;
//! let s = "# Changelog ...";
//! let changelog = parse(&s);
//! let diagnostics = changelog.lint();
//! ```
//!
//! # Parse, lint, locate
//!
//! 1. **Parse** a changelog document into a structured type.
//! 2. **Lint** the structured changelog to produce **diagnostics**.
//! 3. Optional. **Locate** the diagnostics to map them to line and column numbers.
//!
//! The examples below use this minimal, invalid changelog:
//!
//! ```markdown
//! # Changelog
//!
//! ## [Unreleased]
//!
//! ### Added
//!
//! * Add foo ([#12345])
//!
//! [Unreleased]: https://example.org/
//! ```
//!
//! This changelog is invalid because the `[#12345]` link label has no corresponding reference
//! definition.
//!
//! ## Parse
//!
//! Parse a changelog with [`parse`][].
//!
//! ```rust
//! use notabene::parse;
//! let s = r#"# Changelog
//!
//! ### [Unreleased]
//!
//! #### Added
//!
//! * Add foo ([#12345])
//!
//! [Unreleased]: https://example.org/
//! "#;
//! let changelog = parse(&s);
//! ```
//!
//! There are two ways to represent a changelog:
//!
//! * [`ParsedChangelog`](crate::changelog::ParsedChanglog), a borrowed version of the data
//! * [`OwnedChangelog`](crate::changelog::ParsedChanglog), an owned version of the data
//!
//! Parsing is nearly a zero-copy operation and returns a `ParsedChangelog`. This version also
//! includes location information about elements in the changelog for use in the linter. Use
//! [`ParsedChangelog::to_owned`](ParsedChangelog::to_owned) if you need owned data.
//!
//! This crate uses a trait-based API to access the changelog data of either type. The most
//! convenient way to import the traits is to import the prelude:
//!
//! ```rust
//! # use notabene::parse;
//! use notabene::prelude::*;
//! # let s = r#"# Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo ([#12345])
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! # let changelog = parse(&s);
//!
//! assert_eq!(changelog.title(), Some("Changelog"));
//! let unreleased = changelog.unreleased().unwrap();
//! assert_eq!(unreleased.changes()[0].kind(), "Added");
//! assert_eq!(unreleased.changes()[0].items().collect::<Vec<_>>()[0], "Add foo ([#12345])");
//! ```
//!
//! ## Lint
//!
//! Lint a `ParsedChangelog` with
//! [`ParsedChangelog::lint`](crate::changelog::ParsedChangelog::lint).
//!
//! ```rust
//! # use notabene::parse;
//! # use notabene::prelude::*;
//! # let s = r#"# Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo ([#12345])
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! # let changelog = parse(&s);
//! let diagnostics = changelog.lint();
//! ```
//!
//! The linter reports the invalid link as a [`Diagnostic`].
//!
//! ```
//! use notabene::Rule;
//! # use notabene::parse;
//! # use notabene::prelude::*;
//! # let s = r#"# Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo ([#12345])
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! # let changelog = parse(&s);
//! # let diagnostics = changelog.lint();
//! assert_eq!(diagnostics[0].rule, Rule::LinkReferenceDoesNotExist);
//! assert_eq!(diagnostics[0].rule.code(), "E500");
//! ```
//!
//! Use [`Linter`] to create a linter with a custom set of rules.
//!
//! ```rust
//! use notabene::{Linter, RuleSet};
//! # use notabene::Rule;
//! # use notabene::parse;
//! # use notabene::prelude::*;
//! # let s = r#"# Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo ([#12345])
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! # let changelog = parse(&s);
//! // This ruleset excludes the rule above.
//! let ruleset = RuleSet::new([Rule::MissingTitle, Rule::MissingUnreleased]);
//! let linter = Linter::new(&ruleset);
//! let diagnostics = linter.lint(&changelog);
//! assert_eq!(diagnostics, vec![]);
//! ```
//!
//! ## Locate
//!
//! By default, diagnostics report the *spans* in the source document that matched a [`Rule`].
//! A [`Span`](crate::span::Span) represents a range of byte offsets in the source document.
//! The [`Position`](crate::span::Position) type includes the line and column corresponding to the
//! offset.
//!
//! Use [`Diagnostic::locate`] to convert a `Diagnostic<Span>` to a `Diagnostic<Position>`.
//! Or use [`Locator::locate_all`] to do the same for a collection of diagnostics.
//!
//! ```
//! # use notabene::{parse, prelude::*};
//! # let s = r#"# Changelog
//! #
//! # ## [Unreleased]
//! #
//! # ### Added
//! #
//! # * Add foo ([#12345])
//! #
//! # [Unreleased]: https://example.org/
//! # "#;
//! # let changelog = parse(&s);
//! # let diagnostics = changelog.lint();
//! assert_eq!(diagnostics[0].range(), Some(52..60));
//!
//! use notabene::Locator;
//! let locator = Locator::new(&s);
//! let diagnostics = locator.locate_all(&diagnostics);
//! assert_eq!((diagnostics[0].range()), (Some(52..60)));
//! assert_eq!((diagnostics[0].line(), diagnostics[0].column()), (Some(7), Some(12)));
//! ```
pub(crate) mod ast;
pub(crate) mod diagnostic;
pub(crate) mod linter;
pub(crate) mod parser;
pub mod prelude;
pub(crate) mod rule;
pub(crate) mod ruleset;
pub mod span;

pub mod changelog;
#[cfg(feature = "cli")]
pub mod cli;

pub use diagnostic::Diagnostic;
pub use linter::Linter;
pub use parser::parse;
pub use rule::Rule;
pub use ruleset::RuleSet;
pub use span::Locator;
