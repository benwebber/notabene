//! Linter diagnostics.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::location::{Locator, Position, Ranged};
use crate::rule::Rule;
use crate::span::Span;

/// A rule violation.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Diagnostic<L = Span> {
    /// The rule that was violated.
    pub rule: Rule,
    /// Where the violation occurred in the source document.
    pub location: Option<L>,
    /// The source path, used in reporting.
    pub path: Option<PathBuf>,
}

impl Diagnostic {
    /// Create a new diagnostic from a `Span`.
    pub fn new(rule: Rule, location: Option<Span>) -> Self {
        Self {
            rule,
            location,
            path: None,
        }
    }
}

impl<L> Diagnostic<L> {
    /// Return the diagnostic rule code.
    pub fn code(&self) -> &str {
        self.rule.code()
    }
}

impl<Span: Ranged<usize>> Diagnostic<Span> {
    /// Locate a diagnostic in the source document.
    pub fn locate(self, locator: &Locator) -> Diagnostic<Position> {
        Diagnostic {
            rule: self.rule,
            location: self.location.map(|s| locator.position(&s.range())),
            path: self.path,
        }
    }
}

impl<L: Ranged<usize>> Diagnostic<L> {
    /// Return a formatted message.
    pub fn message(&self, source: &str) -> String {
        let range = self.location.as_ref().map(|l| l.range());
        match range {
            Some(range) => {
                let snippet = &source[range];
                self.rule.message().replace("{}", snippet)
            }
            None => self.rule.message().to_string(),
        }
    }

    /// Return the unist Position of the diagnostic.
    pub fn position(&self, locator: &Locator) -> Option<Position> {
        self.location.as_ref().map(|l| locator.position(&l.range()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Diagnostic::new(Rule::MissingTitle, None),
            Diagnostic {
                rule: Rule::MissingTitle,
                location: None,
                path: None
            }
        );
        assert_eq!(
            Diagnostic::new(Rule::MissingTitle, Some(Span::default())),
            Diagnostic {
                rule: Rule::MissingTitle,
                location: Some(Span::default()),
                path: None
            }
        );
    }

    #[test]
    fn test_code() {
        assert_eq!(
            Diagnostic::new(Rule::MissingTitle, None).code(),
            Rule::MissingTitle.code()
        );
    }

    #[test]
    fn test_message() {
        let source = "";
        let diagnostic = Diagnostic::new(Rule::MissingTitle, None);
        assert_eq!(diagnostic.message(source), Rule::MissingTitle.message());

        let source = "# Changelog";
        let diagnostic = Diagnostic::new(Rule::DuplicateTitle, Some(Span::new(2, 11)));
        assert_eq!(
            diagnostic.message(source),
            Rule::DuplicateTitle.message().replace("{}", "Changelog")
        );
    }
}
