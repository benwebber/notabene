use std::collections::HashSet;

use super::preamble::*;

use crate::changelog::parsed;

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

    fn visit_changes(&mut self, changes: &parsed::Changes) {
        if !matches!(
            changes.kind.value,
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

    fn visit_unreleased(&mut self, _unreleased: &parsed::Unreleased) {
        self.seen.clear();
    }

    fn visit_release(&mut self, _unreleased: &parsed::Release) {
        self.seen.clear();
    }

    fn visit_changes(&mut self, changes: &parsed::Changes) {
        if !self.seen.insert(changes.kind.value.to_string()) {
            self.spans.push(changes.kind.span);
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
    fn test_invalid_change_type() {
        let ruleset = RuleSet::from([Rule::InvalidChangeType]);
        let linter = Linter::new(&ruleset);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = Changelog {
            unreleased: Some(Unreleased {
                changes: vec![
                    Changes {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    Changes {
                        kind: Spanned::new(Span::new(1, usize::MAX), "Foo"),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            releases: vec![Release {
                changes: vec![
                    Changes {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    Changes {
                        kind: Spanned::new(Span::new(2, usize::MAX), "Foo"),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_duplicate_change_type() {
        let ruleset = RuleSet::from([Rule::DuplicateChangeType]);
        let linter = Linter::new(&ruleset);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = Changelog {
            unreleased: Some(Unreleased {
                changes: vec![
                    Changes {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    Changes {
                        kind: Spanned::new(Span::new(1, usize::MAX), "Added"),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            releases: vec![Release {
                changes: vec![
                    Changes {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    Changes {
                        kind: Spanned::new(Span::new(2, usize::MAX), "Added"),
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
