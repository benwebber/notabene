use crate::changelog::v2::parsed;
use crate::ir::Changelog;
use crate::linter::Check;
use crate::rule::Rule;
use crate::span::Span;

#[derive(Default)]
pub struct LinkReferenceDoesNotExist {
    spans: Vec<Span>,
}

impl Check for LinkReferenceDoesNotExist {
    fn rule(&self) -> Rule {
        Rule::LinkReferenceDoesNotExist
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

    use crate::changelog::v2::parsed::{Changelog, InvalidSpan};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_link_reference_does_not_exist() {
        let ruleset = RuleSet::from([Rule::LinkReferenceDoesNotExist]);
        let linter = Linter::new(&ruleset);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = Changelog {
            invalid_spans: vec![InvalidSpan::InvalidLinkReference(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
