use crate::changelog::parsed;
use crate::linter::Check;
use crate::rule::Rule;

use super::preamble::*;

invalid_span!(UndefinedLinkReference);

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{InvalidSpan, ParsedChangelog};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_undefined_link_reference() {
        let ruleset = RuleSet::from([Rule::UndefinedLinkReference]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::UndefinedLinkReference(Span::new(
                1,
                usize::MAX,
            ))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
