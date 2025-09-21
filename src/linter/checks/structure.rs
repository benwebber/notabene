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

invalid_span!(
    DuplicateTitle,
    Rule::DuplicateTitle,
    parsed::InvalidSpan::DuplicateTitle
);

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

invalid_span!(
    DuplicateUnreleased,
    Rule::DuplicateUnreleased,
    parsed::InvalidSpan::DuplicateUnreleased
);

invalid_span!(
    UnreleasedOutOfOrder,
    Rule::InvalidUnreleasedPosition,
    parsed::InvalidSpan::UnreleasedOutOfOrder
);

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{InvalidSpan, ParsedChangelog, ParsedUnreleased};
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
    fn test_unreleased_not_first() {
        let ruleset = RuleSet::from([Rule::InvalidUnreleasedPosition]);
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
