//! `E100` Unreleased
use super::preamble::*;

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
