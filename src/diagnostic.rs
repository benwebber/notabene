//! Linter diagnostics.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::rule::Rule;
use crate::span::Index;
use crate::span::Span;
use crate::unist::Position;

/// A rule violation.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Diagnostic {
    /// The rule that was violated.
    pub rule: Rule,
    /// Where the violation occurred in the source document.
    pub span: Option<Span>,
    /// The source path, used in reporting.
    pub path: Option<PathBuf>,
}

impl Diagnostic {
    pub fn new(rule: Rule, span: Option<Span>) -> Self {
        Self {
            rule,
            span,
            path: None,
        }
    }

    pub fn code(&self) -> &str {
        self.rule.code()
    }

    pub fn message(&self, source: &str) -> String {
        match self.span {
            Some(span) => {
                let snippet = &source[span.range()];
                self.rule.message().replace("{}", snippet)
            }
            None => self.rule.message().to_string(),
        }
    }

    pub fn position(&self, index: &Index) -> Option<Position> {
        self.span.map(|span| index.position(&span))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::unist::Point;

    #[test]
    fn test_new() {
        assert_eq!(
            Diagnostic::new(Rule::MissingTitle, None),
            Diagnostic {
                rule: Rule::MissingTitle,
                span: None,
                path: None
            }
        );
        assert_eq!(
            Diagnostic::new(Rule::MissingTitle, Some(Span::default())),
            Diagnostic {
                rule: Rule::MissingTitle,
                span: Some(Span::default()),
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

    #[test]
    fn test_position() {
        let index = Index::new("# Changelog");
        let diagnostic = Diagnostic::new(Rule::DuplicateTitle, Some(Span::new(2, 11)));
        assert_eq!(
            diagnostic.position(&index),
            Some(Position::new(Point::new(1, 3, 2), Point::new(1, 12, 11)))
        );
    }
}
