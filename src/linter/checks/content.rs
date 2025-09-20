use crate::changelog::v2::parsed;
use crate::ir::{Changes, Section};
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

    fn visit_changes_v2(&mut self, changes: &parsed::Changes) {
        if changes.items.is_empty() {
            self.spans.push(changes.heading_span)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::{self, *};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_empty_section() {
        let ruleset = RuleSet::from([Rule::EmptySection]);
        let linter = Linter::new(&ruleset);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = Changelog {
            sections: vec![
                // Unreleased can be empty.
                Section::Unreleased(Unreleased::default()),
                Section::Unreleased(ir::Unreleased {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added"),
                            changes: vec![Spanned::new(Span::new(0, 0), "Add foo")],
                            ..Default::default()
                        },
                        // Empty changes.
                        Changes {
                            heading_span: Span::new(2, usize::MAX),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                // No changes.
                Section::Release(ir::Release {
                    heading_span: Span::new(3, usize::MAX),
                    ..Default::default()
                }),
                Section::Release(ir::Release {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added"),
                            changes: vec![Spanned::new(Span::new(0, 0), "Add foo")],
                            ..Default::default()
                        },
                        // Empty changes.
                        Changes {
                            heading_span: Span::new(4, usize::MAX),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
