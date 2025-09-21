use std::collections::HashSet;

use crate::changelog::parsed;
use crate::linter::Check;
use crate::rule::Rule;
use crate::span::Span;

invalid_span!(InvalidTitle);

invalid_span!(InvalidSectionHeading);

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

#[derive(Default)]
pub struct UnknownChangeType {
    spans: Vec<Span>,
}

impl Check for UnknownChangeType {
    fn rule(&self) -> Rule {
        Rule::UnknownChangeType
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_changes(&mut self, changes: &parsed::ParsedChanges) {
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

    fn visit_unreleased(&mut self, _unreleased: &parsed::ParsedUnreleased) {
        self.seen.clear();
    }

    fn visit_release(&mut self, _unreleased: &parsed::ParsedRelease) {
        self.seen.clear();
    }

    fn visit_changes(&mut self, changes: &parsed::ParsedChanges) {
        if !self.seen.insert(changes.kind.value.to_string()) {
            self.spans.push(changes.kind.span);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{
        InvalidSpan, ParsedChangelog, ParsedChanges, ParsedRelease, ParsedUnreleased,
    };
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::{Span, Spanned};

    #[test]
    fn test_invalid_title() {
        let ruleset = RuleSet::from([Rule::InvalidTitle]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::InvalidTitle(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_invalid_section() {
        let ruleset = RuleSet::from([Rule::InvalidSectionHeading]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            invalid_spans: vec![InvalidSpan::InvalidSectionHeading(Span::new(1, usize::MAX))],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

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

    #[test]
    fn test_unknown_change_type() {
        let ruleset = RuleSet::from([Rule::UnknownChangeType]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            unreleased: Some(ParsedUnreleased {
                changes: vec![
                    ParsedChanges {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    ParsedChanges {
                        kind: Spanned::new(Span::new(1, usize::MAX), "Foo"),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            releases: vec![ParsedRelease {
                changes: vec![
                    ParsedChanges {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    ParsedChanges {
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

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            unreleased: Some(ParsedUnreleased {
                changes: vec![
                    ParsedChanges {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    ParsedChanges {
                        kind: Spanned::new(Span::new(1, usize::MAX), "Added"),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            releases: vec![ParsedRelease {
                changes: vec![
                    ParsedChanges {
                        kind: Spanned::new(Span::new(0, 0), "Added"),
                        ..Default::default()
                    },
                    ParsedChanges {
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
