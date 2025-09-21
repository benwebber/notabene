//! `E000` Structure
use super::preamble::*;

#[derive(Default)]
pub struct MissingTitle;

impl Check for MissingTitle {
    fn rule(&self) -> Rule {
        Rule::MissingTitle
    }

    fn visit_changelog(&mut self, context: &mut Context, changelog: &parsed::ParsedChangelog) {
        if changelog.title.is_none() {
            context.report(self.rule(), None);
        }
    }
}

invalid_span!(DuplicateTitle);

#[derive(Default)]
pub struct MissingUnreleased;

impl Check for MissingUnreleased {
    fn rule(&self) -> Rule {
        Rule::MissingUnreleased
    }

    fn visit_changelog(&mut self, context: &mut Context, changelog: &parsed::ParsedChangelog) {
        if changelog.unreleased.is_none() {
            context.report(self.rule(), None);
        }
    }
}

invalid_span!(DuplicateUnreleased);

invalid_span!(InvalidUnreleasedPosition);

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
    fn test_invalid_unreleased_position() {
        let ruleset = RuleSet::from([Rule::InvalidUnreleasedPosition]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::InvalidUnreleasedPosition(Span::new(
                1,
                usize::MAX,
            ))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
