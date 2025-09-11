use crate::ir::{Changes, Section};
use crate::linter::Check;
use crate::rule::Rule;
use crate::span::Span;

#[derive(Default)]
pub struct EmptySection {
    spans: Vec<Span>,
}

impl Check for EmptySection {
    fn rule(&self) -> Rule {
        Rule::EmptySection
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        match section {
            Section::Release(release) => {
                if release.changes.is_empty() {
                    self.spans.push(release.heading_span);
                }
            }
            _ => {}
        }
    }

    fn visit_changes(&mut self, changes: &Changes) {
        if changes.changes.is_empty() {
            self.spans.push(changes.heading_span)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::*;
    use crate::linter::lint;
    use crate::profile::Profile;
    use crate::span::Span;

    #[test]
    fn test_empty_section() {
        let profile = Profile::new(&[Rule::EmptySection]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                // Unreleased can be empty.
                Section::Unreleased(Unreleased::default()),
                Section::Unreleased(Unreleased {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            changes: vec![Spanned::new(Span::new(0, 0), "Add foo".to_string())],
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
                Section::Release(Release {
                    heading_span: Span::new(3, usize::MAX),
                    ..Default::default()
                }),
                Section::Release(Release {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            changes: vec![Spanned::new(Span::new(0, 0), "Add foo".to_string())],
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
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }
}
