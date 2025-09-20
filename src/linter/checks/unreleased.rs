//! `E100` Unreleased
use super::preamble::*;

#[derive(Default)]
pub struct MissingUnreleased {
    found: bool,
}

impl Check for MissingUnreleased {
    fn rule(&self) -> Rule {
        Rule::MissingUnreleased
    }

    fn visit_changelog(&mut self, changelog: &parsed::ParsedChangelog) {
        self.found = changelog.unreleased.is_some();
    }

    fn diagnostics(&self) -> Vec<Diagnostic> {
        if self.found {
            vec![]
        } else {
            vec![Diagnostic::new(self.rule(), None)]
        }
    }
}

#[derive(Default)]
pub struct DuplicateUnreleased {
    spans: Vec<Span>,
    found: bool,
}

impl Check for DuplicateUnreleased {
    fn rule(&self) -> Rule {
        Rule::DuplicateUnreleased
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
        if let parsed::InvalidSpan::DuplicateUnreleased(s) = span {
            self.spans.push(*s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{InvalidSpan, ParsedChangelog, ParsedUnreleased};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_missing_unreleased() {
        let ruleset = RuleSet::from([Rule::MissingUnreleased]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            unreleased: Some(ParsedUnreleased::default()),
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_duplicate_unreleased() {
        let ruleset = RuleSet::from([Rule::DuplicateUnreleased]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::DuplicateUnreleased(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
