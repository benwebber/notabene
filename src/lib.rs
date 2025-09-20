//! A fast linter for changelogs in the Keep a Changelog format.
//! ```

pub(crate) mod ast;
pub(crate) mod diagnostic;
pub mod ir;
pub(crate) mod linter;
pub(crate) mod parser;
pub(crate) mod rule;
pub(crate) mod ruleset;
pub mod span;

pub mod changelog;
#[cfg(feature = "cli")]
pub mod cli;

pub use changelog::Changelog;
pub use diagnostic::Diagnostic;
pub use linter::{Linter, lint};
pub use parser::parse;
pub use rule::Rule;
pub use ruleset::RuleSet;
pub use span::Locator;
