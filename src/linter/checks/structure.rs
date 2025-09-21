//! `E000` Structure
use super::preamble::*;

#[derive(Default)]
pub struct MissingTitle {
    found: bool,
}

impl Check for MissingTitle {
    fn rule(&self) -> Rule {
        Rule::MissingTitle
    }

    fn visit_changelog(&mut self, changelog: &parsed::ParsedChangelog) {
        if self.found {
            return;
        }
        if changelog.title().is_some() {
            self.found = true;
        }
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
pub struct InvalidTitle {
    spans: Vec<Span>,
}

impl Check for InvalidTitle {
    fn rule(&self) -> Rule {
        Rule::InvalidTitle
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
        if let parsed::InvalidSpan::InvalidTitle(s) = span {
            self.spans.push(*s);
        }
    }
}

#[derive(Default)]
pub struct DuplicateTitle {
    spans: Vec<Span>,
}

impl Check for DuplicateTitle {
    fn rule(&self) -> Rule {
        Rule::DuplicateTitle
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
        if let parsed::InvalidSpan::DuplicateTitle(s) = span {
            self.spans.push(*s);
        }
    }
}

#[derive(Default)]
pub struct InvalidSectionHeading {
    spans: Vec<Span>,
}

impl Check for InvalidSectionHeading {
    fn rule(&self) -> Rule {
        Rule::InvalidSectionHeading
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
        if let parsed::InvalidSpan::InvalidHeading(s) = span {
            self.spans.push(*s);
        }
    }
}

#[derive(Default)]
pub struct UnreleasedOutOfOrder {
    spans: Vec<Span>,
}

impl Check for UnreleasedOutOfOrder {
    fn rule(&self) -> Rule {
        Rule::UnreleasedOutOfOrder
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
        if let parsed::InvalidSpan::UnreleasedOutOfOrder(s) = span {
            self.spans.push(*s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{InvalidSpan, ParsedChangelog};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::{Span, Spanned};

    #[test]
    fn test_missing_title() {
        let ruleset = RuleSet::from([Rule::MissingTitle]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            title: Some(Spanned::<&str>::default()),
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_invalid_title() {
        let ruleset = RuleSet::from([Rule::InvalidTitle]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::InvalidTitle(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_duplicate_title() {
        let ruleset = RuleSet::from([Rule::DuplicateTitle]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::DuplicateTitle(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_invalid_section() {
        let ruleset = RuleSet::from([Rule::InvalidSectionHeading]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::InvalidHeading(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_unreleased_not_first() {
        let ruleset = RuleSet::from([Rule::UnreleasedOutOfOrder]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::UnreleasedOutOfOrder(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
