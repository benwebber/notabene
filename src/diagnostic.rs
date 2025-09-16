//! Linter diagnostics.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::location::{Location, Locator, Position};
use crate::rule::Rule;
use crate::span::Span;

/// A rule violation.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Diagnostic {
    /// The rule that was violated.
    pub rule: Rule,
    /// Where the violation occurred in the source document.
    pub location: Option<Location>,
    /// The source path, used in reporting.
    pub path: Option<PathBuf>,
}

impl Diagnostic {
    pub fn new(rule: Rule, location: Option<Location>) -> Self {
        Self {
            rule,
            location,
            path: None,
        }
    }

    pub fn code(&self) -> &str {
        self.rule.code()
    }

    pub fn message(&self, source: &str) -> String {
        let range = match self.location {
            Some(Location::Span(span)) => Some(span.range()),
            Some(Location::Position(pos)) => Some(pos.start.offset..pos.end.offset),
            _ => None,
        };
        match range {
            Some(range) => {
                let snippet = &source[range];
                self.rule.message().replace("{}", snippet)
            }
            None => self.rule.message().to_string(),
        }
    }

    pub fn position(&self, locator: &Locator) -> Option<Position> {
        match self.location {
            Some(Location::Position(p)) => Some(p),
            Some(Location::Span(s)) => Some(locator.position(&s)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::location::Point;

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
            Diagnostic::new(Rule::MissingTitle, Some(Location::default())),
            Diagnostic {
                rule: Rule::MissingTitle,
                location: Some(Location::default()),
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
        let diagnostic =
            Diagnostic::new(Rule::DuplicateTitle, Some(Location::Span(Span::new(2, 11))));
        assert_eq!(
            diagnostic.message(source),
            Rule::DuplicateTitle.message().replace("{}", "Changelog")
        );
    }
}
