use std::collections::HashSet;

use super::preamble::*;

#[derive(Default)]
pub struct InvalidChangeType {
    spans: Vec<Span>,
}

impl Check for InvalidChangeType {
    fn rule(&self) -> Rule {
        Rule::InvalidChangeType
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_changes(&mut self, changes: &Changes) {
        if !matches!(
            changes.kind.value.as_str(),
            "Added" | "Changed" | "Deprecated" | "Fixed" | "Removed" | "Security"
        ) {
            self.spans.push(changes.kind.span);
        }
    }
}

#[derive(Default)]
pub struct DuplicateChangeType {
    spans: Vec<Span>,
    seen: HashSet<String>,
}

impl Check for DuplicateChangeType {
    fn rule(&self) -> Rule {
        Rule::DuplicateChangeType
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, _section: &Section) {
        self.seen.clear();
    }

    fn visit_changes(&mut self, changes: &Changes) {
        if !self.seen.insert(changes.kind.value.clone()) {
            self.spans.push(changes.kind.span);
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
    fn test_invalid_change_type() {
        let profile = Profile::new(&[Rule::InvalidChangeType]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Unreleased(Unreleased {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            ..Default::default()
                        },
                        Changes {
                            kind: Spanned::new(Span::new(1, usize::MAX), "Foo".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                Section::Release(Release {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            ..Default::default()
                        },
                        Changes {
                            kind: Spanned::new(Span::new(2, usize::MAX), "Foo".to_string()),
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

    #[test]
    fn test_duplicate_change_type() {
        let profile = Profile::new(&[Rule::DuplicateChangeType]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Unreleased(Unreleased {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            ..Default::default()
                        },
                        Changes {
                            kind: Spanned::new(Span::new(1, usize::MAX), "Added".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                Section::Release(Release {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            ..Default::default()
                        },
                        Changes {
                            kind: Spanned::new(Span::new(2, usize::MAX), "Added".to_string()),
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
