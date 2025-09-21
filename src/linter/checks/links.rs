use crate::changelog::parsed;
use crate::linter::Check;
use crate::rule::Rule;
use crate::span::Span;

#[derive(Default)]
pub struct LinkReferenceDoesNotExist {
    spans: Vec<Span>,
}

impl Check for LinkReferenceDoesNotExist {
    fn rule(&self) -> Rule {
        Rule::UndefinedLinkReference
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_invalid_span(&mut self, span: &parsed::InvalidSpan) {
        if let parsed::InvalidSpan::InvalidLinkReference(s) = span {
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
    use crate::span::Span;

    #[test]
    fn test_link_reference_does_not_exist() {
        let ruleset = RuleSet::from([Rule::UndefinedLinkReference]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::InvalidLinkReference(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
