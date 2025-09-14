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

    fn visit_changelog(&mut self, changelog: &Changelog) {
        self.spans.extend_from_slice(&changelog.broken_links);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::*;
    use crate::linter::lint;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_link_reference_does_not_exist() {
        let ruleset = RuleSet::from([Rule::LinkReferenceDoesNotExist]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset));

        let changelog = Changelog {
            broken_links: vec![Span::new(1, usize::MAX)],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset));
    }
}
