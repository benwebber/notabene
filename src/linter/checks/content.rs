use crate::changelog::parsed;
use crate::linter::Check;
use crate::rule::Rule;
use crate::span::Span;

#[derive(Default)]
pub struct EmptySection {
    spans: Vec<Span>,
}

// TODO: Store better spans for these headings.
impl Check for EmptySection {
    fn rule(&self) -> Rule {
        Rule::EmptySection
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_release(&mut self, release: &parsed::ParsedRelease) {
        if release.changes.is_empty() {
            self.spans.push(release.heading_span);
        }
    }

    fn visit_changes(&mut self, changes: &parsed::ParsedChanges) {
        if changes.items.is_empty() {
            self.spans.push(changes.heading_span)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{
        ParsedChangelog, ParsedChanges, ParsedRelease, ParsedUnreleased,
    };
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::{Span, Spanned};

    #[test]
    fn test_empty_section() {
        let ruleset = RuleSet::from([Rule::EmptySection]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        // ParsedUnreleased with no changes.
        let changelog = ParsedChangelog {
            unreleased: Some(ParsedUnreleased::default()),
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));

        // ParsedUnreleased with empty change section.
        let changelog = ParsedChangelog {
            unreleased: Some(ParsedUnreleased {
                changes: vec![
                    ParsedChanges {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        items: vec![Spanned::new(Span::new(0, 0), "Add foo")],
                        ..Default::default()
                    },
                    // Empty changes.
                    ParsedChanges {
                        heading_span: Span::new(1, usize::MAX),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));

        // ParsedRelease with no changes.
        let changelog = ParsedChangelog {
            releases: vec![ParsedRelease {
                heading_span: Span::new(1, usize::MAX),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));

        // ParsedRelease with empty change section.
        let changelog = ParsedChangelog {
            releases: vec![ParsedRelease {
                changes: vec![
                    ParsedChanges {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        items: vec![Spanned::new(Span::new(0, 0), "Add foo")],
                        ..Default::default()
                    },
                    // Empty changes.
                    ParsedChanges {
                        heading_span: Span::new(1, usize::MAX),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
