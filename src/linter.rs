//! Linter implementation.
use std::collections::HashSet;

use time::Date;
use time::macros::format_description;

use crate::diagnostic::Diagnostic;
use crate::ir::{Changelog, Changes, Section};
use crate::profile::Profile;
use crate::rule::Rule;
use crate::span::Span;

mod check;

use check::Check;

pub fn lint(changelog: &Changelog, profile: &Profile) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let mut checks: Vec<_> = checks()
        .into_iter()
        .filter(|check| profile.is_enabled(check.rule()))
        .collect();

    for section in &changelog.sections {
        for check in checks.iter_mut() {
            check.visit_section(section);
            match section {
                Section::Unreleased(unreleased) => {
                    for changes in &unreleased.changes {
                        check.visit_changes(changes);
                    }
                }
                Section::Release(release) => {
                    for changes in &release.changes {
                        check.visit_changes(changes);
                    }
                }
                _ => {}
            }
        }
    }

    for check in checks.iter_mut() {
        diagnostics.append(&mut check.diagnostics());
    }

    diagnostics
}

fn checks() -> Vec<Box<dyn Check>> {
    vec![
        // E000 Document
        Box::new(MissingTitleCheck::default()),
        Box::new(DuplicateTitleCheck::default()),
        Box::new(UnreleasedNotFirstCheck::default()),
        //// E100 Unreleased
        Box::new(MissingUnreleasedCheck::default()),
        Box::new(DuplicateUnreleasedCheck::default()),
        //// E200 Release
        Box::new(InvalidDateCheck::default()),
        Box::new(InvalidYankedCheck::default()),
        //// E300 Changes
        Box::new(InvalidChangeTypeCheck::default()),
        Box::new(DuplicateChangeTypeCheck::default()),
        //// E400 Content
        Box::new(EmptySectionCheck::default()),
    ]
}

// E000 Document
// =============

#[derive(Default)]
struct MissingTitleCheck {
    found: bool,
}

impl Check for MissingTitleCheck {
    fn rule(&self) -> Rule {
        Rule::MissingTitle
    }

    fn visit_section(&mut self, section: &Section) {
        if self.found {
            return;
        }
        if matches!(section, Section::Title(_)) {
            self.found = true;
        }
    }

    fn diagnostics(&self) -> Vec<Diagnostic> {
        if self.found {
            vec![]
        } else {
            vec![Diagnostic::new(self.rule(), None)]
        }
    }
}

#[derive(Default)]
struct DuplicateTitleCheck {
    spans: Vec<Span>,
    found: bool,
}

impl Check for DuplicateTitleCheck {
    fn rule(&self) -> Rule {
        Rule::DuplicateTitle
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Title(spanned) = section {
            if self.found {
                self.spans.push(spanned.span);
            } else {
                self.found = true;
            }
        }
    }
}

// E100 Unreleased
// ===============

#[derive(Default)]
struct MissingUnreleasedCheck {
    found: bool,
}

impl Check for MissingUnreleasedCheck {
    fn rule(&self) -> Rule {
        Rule::MissingUnreleased
    }

    fn visit_section(&mut self, section: &Section) {
        if self.found {
            return;
        }
        if matches!(section, Section::Unreleased(_)) {
            self.found = true;
        }
    }

    fn diagnostics(&self) -> Vec<Diagnostic> {
        if self.found {
            vec![]
        } else {
            vec![Diagnostic::new(self.rule(), None)]
        }
    }
}

#[derive(Default)]
struct DuplicateUnreleasedCheck {
    spans: Vec<Span>,
    found: bool,
}

impl Check for DuplicateUnreleasedCheck {
    fn rule(&self) -> Rule {
        Rule::DuplicateUnreleased
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Unreleased(unreleased) = section {
            if self.found {
                self.spans.push(unreleased.heading_span);
            } else {
                self.found = true;
            }
        }
    }
}

#[derive(Default)]
struct UnreleasedNotFirstCheck {
    spans: Vec<Span>,
    found: bool,
}

impl Check for UnreleasedNotFirstCheck {
    fn rule(&self) -> Rule {
        Rule::UnreleasedNotFirst
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if self.found {
            return;
        }
        match section {
            Section::Unreleased(_) => self.found = true,
            Section::Release(release) => self.spans.push(release.heading_span),
            _ => {}
        }
    }
}

// E200 Release
// ============

#[derive(Default)]
struct InvalidDateCheck {
    spans: Vec<Span>,
}

impl Check for InvalidDateCheck {
    fn rule(&self) -> Rule {
        Rule::InvalidDate
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        let format = format_description!("[year]-[month]-[day]");
        if let Section::Release(release) = section {
            if let Some(spanned) = &release.date {
                if Date::parse(&spanned.value, &format).is_err() {
                    self.spans.push(spanned.span);
                }
            }
        }
    }
}

#[derive(Default)]
struct InvalidYankedCheck {
    spans: Vec<Span>,
}

impl Check for InvalidYankedCheck {
    fn rule(&self) -> Rule {
        Rule::InvalidYanked
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Release(release) = section {
            if let Some(spanned) = &release.yanked {
                if spanned.value != "[YANKED]" {
                    self.spans.push(spanned.span);
                }
            }
        }
    }
}

// E300 Changes
// ============

#[derive(Default)]
struct InvalidChangeTypeCheck {
    spans: Vec<Span>,
}

impl Check for InvalidChangeTypeCheck {
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
struct DuplicateChangeTypeCheck {
    spans: Vec<Span>,
    seen: HashSet<String>,
}

impl Check for DuplicateChangeTypeCheck {
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

// E400 Content
// ============

#[derive(Default)]
struct EmptySectionCheck {
    spans: Vec<Span>,
}

impl Check for EmptySectionCheck {
    fn rule(&self) -> Rule {
        Rule::EmptySection
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        match section {
            Section::Unreleased(unreleased) => {
                if unreleased.changes.is_empty() {
                    self.spans.push(unreleased.heading_span);
                }
            }
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
    use crate::profile::Profile;
    use crate::span::Span;

    #[test]
    fn test_missing_title() {
        let profile = Profile::new(&[Rule::MissingTitle]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![Section::Title(Spanned::<String>::default())],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

    #[test]
    fn test_duplicate_title() {
        let profile = Profile::new(&[Rule::DuplicateTitle]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Title(Spanned::<String>::default()),
                Section::Title(Spanned::new(Span::new(2, 11), "Changelog".to_string())),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

    #[test]
    fn test_missing_unreleased() {
        let profile = Profile::new(&[Rule::MissingUnreleased]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![Section::Unreleased(Unreleased::default())],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

    #[test]
    fn test_unreleased_not_first() {
        let profile = Profile::new(&[Rule::UnreleasedNotFirst]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    heading_span: Span::new(1, usize::MAX),
                    ..Default::default()
                }),
                Section::Unreleased(Unreleased::default()),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

    #[test]
    fn test_duplicate_unreleased() {
        let profile = Profile::new(&[Rule::DuplicateUnreleased]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Unreleased(Unreleased::default()),
                Section::Unreleased(Unreleased {
                    heading_span: Span::new(0, 17),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

    #[test]
    fn test_invalid_date() {
        let profile = Profile::new(&[Rule::InvalidDate]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(0, 9), "2038-01-19".to_string())),
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(1, 10), "2001-01-00".to_string())),
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(2, 5), "foo".to_string())),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

    #[test]
    fn test_invalid_yanked() {
        let profile = Profile::new(&[Rule::InvalidYanked]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    ..Default::default()
                }),
                Section::Release(Release {
                    yanked: Some(Spanned::new(Span::new(0, 9), "[YANKED]".to_string())),
                    ..Default::default()
                }),
                Section::Release(Release {
                    yanked: Some(Spanned::new(Span::new(1, 10), "[ZANKED]".to_string())),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }

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

    #[test]
    fn test_empty_section() {
        let profile = Profile::new(&[Rule::EmptySection]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                // No changes.
                Section::Unreleased(Unreleased {
                    heading_span: Span::new(1, usize::MAX),
                    ..Default::default()
                }),
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
