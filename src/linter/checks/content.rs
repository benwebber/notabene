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

    fn visit_release(&mut self, release: &parsed::Release) {
        if release.changes.is_empty() {
            self.spans.push(release.heading_span);
        }
    }

    fn visit_changes(&mut self, changes: &parsed::Changes) {
        if changes.items.is_empty() {
            self.spans.push(changes.heading_span)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{Changelog, Changes, Release, Unreleased};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::{Span, Spanned};

    #[test]
    fn test_empty_section() {
        let ruleset = RuleSet::from([Rule::EmptySection]);
        let linter = Linter::new(&ruleset);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        // Unreleased with no changes.
        let changelog = Changelog {
            unreleased: Some(Unreleased::default()),
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));

        // Unreleased with empty change section.
        let changelog = Changelog {
            unreleased: Some(Unreleased {
                changes: vec![
                    Changes {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        items: vec![Spanned::new(Span::new(0, 0), "Add foo")],
                        ..Default::default()
                    },
                    // Empty changes.
                    Changes {
                        heading_span: Span::new(1, usize::MAX),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));

        // Release with no changes.
        let changelog = Changelog {
            releases: vec![Release {
                heading_span: Span::new(1, usize::MAX),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));

        // Release with empty change section.
        let changelog = Changelog {
            releases: vec![Release {
                changes: vec![
                    Changes {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        items: vec![Spanned::new(Span::new(0, 0), "Add foo")],
                        ..Default::default()
                    },
                    // Empty changes.
                    Changes {
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
