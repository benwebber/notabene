//! Linter implementation.
use std::collections::HashSet;

use time::Date;
use time::macros::format_description;

use crate::diagnostic::Diagnostic;
use crate::ir::Changelog;
use crate::profile::Profile;
use crate::rule::Rule;

macro_rules! check {
    ($name:ident, $linter:ident, $changelog: ident, $f:block) => {
        struct $name;
        impl Check for $name {
            fn rule(&self) -> Rule {
                Rule::$name
            }

            fn check(&$linter, $changelog: &Changelog) -> Vec<Diagnostic> $f
        }
    }
}

pub(crate) trait Check {
    fn rule(&self) -> Rule;
    fn check(&self, changelog: &Changelog) -> Vec<Diagnostic>;
}

pub fn lint(changelog: &Changelog, profile: &Profile) -> Vec<Diagnostic> {
    checks()
        .into_iter()
        .filter(|check| profile.is_enabled(check.rule()))
        .flat_map(|check| check.check(changelog))
        .collect()
}

fn checks() -> Vec<Box<dyn Check>> {
    vec![
        // E000 Document
        Box::new(MissingTitle),
        Box::new(DuplicateTitle),
        // E100 Unreleased
        Box::new(MissingUnreleased),
        Box::new(DuplicateUnreleased),
        // E200 Release
        Box::new(InvalidDate),
        Box::new(InvalidYanked),
        // E300 Changes
        Box::new(InvalidChangeType),
        Box::new(DuplicateChangeType),
        // E400 Content
        Box::new(EmptySection),
    ]
}

// E000 Document
// =============

check!(MissingTitle, self, changelog, {
    if changelog.titles.is_empty() {
        vec![Diagnostic::new(self.rule(), None)]
    } else {
        vec![]
    }
});

check!(DuplicateTitle, self, changelog, {
    changelog
        .titles
        .iter()
        .skip(1)
        .map(|s| Diagnostic::new(self.rule(), Some(s.span)))
        .collect()
});

// E100 Unreleased
// ===============

check!(MissingUnreleased, self, changelog, {
    match changelog.unreleased().next() {
        Some(_) => vec![],
        None => vec![Diagnostic::new(self.rule(), None)],
    }
});

check!(DuplicateUnreleased, self, changelog, {
    changelog
        .unreleased()
        .skip(1)
        .map(|u| Diagnostic::new(self.rule(), Some(u.heading_span)))
        .collect()
});

// E200 Release
// ============

check!(InvalidDate, self, changelog, {
    let format = format_description!("[year]-[month]-[day]");
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    for release in changelog.releases() {
        match &release.date {
            Some(spanned) => {
                if Date::parse(&spanned.value, &format).is_err() {
                    diagnostics.push(Diagnostic::new(self.rule(), Some(spanned.span)));
                }
            }
            None => continue,
        }
    }
    diagnostics
});

check!(InvalidYanked, self, changelog, {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    for release in changelog.releases() {
        match &release.yanked {
            Some(spanned) => {
                if spanned.value == "[YANKED]" {
                    continue;
                } else {
                    diagnostics.push(Diagnostic::new(self.rule(), Some(spanned.span)));
                }
            }
            None => continue,
        }
    }
    diagnostics
});

// E300 Changes
// ============

check!(InvalidChangeType, self, changelog, {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    for section in changelog.sections() {
        for changes in section.changes().iter() {
            if !matches!(
                changes.kind.value.as_str(),
                "Added" | "Changed" | "Deprecated" | "Fixed" | "Removed" | "Security"
            ) {
                diagnostics.push(Diagnostic::new(self.rule(), Some(changes.kind.span)));
            }
        }
    }
    diagnostics
});

check!(DuplicateChangeType, self, changelog, {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut seen = HashSet::new();
    for section in changelog.sections() {
        seen.clear();
        for changes in section.changes().iter() {
            if !seen.insert(changes.kind.value.as_str()) {
                diagnostics.push(Diagnostic::new(self.rule(), Some(changes.kind.span)));
            }
        }
    }
    diagnostics
});

// E400 Content
// ============

check!(EmptySection, self, changelog, {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    for unreleased in changelog.unreleased() {
        for changes in unreleased.changes.iter() {
            if changes.changes.is_empty() {
                diagnostics.push(Diagnostic::new(self.rule(), Some(changes.kind.span)));
            }
        }
    }
    for release in changelog.releases() {
        if release.changes.is_empty() {
            diagnostics.push(Diagnostic::new(self.rule(), Some(release.heading_span)));
        } else {
            for changes in release.changes.iter() {
                if changes.changes.is_empty() {
                    diagnostics.push(Diagnostic::new(self.rule(), Some(changes.kind.span)));
                }
            }
        }
    }
    diagnostics
});

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::*;
    use crate::span::Span;

    #[test]
    fn test_missing_title() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(MissingTitle.check(&changelog));

        let changelog = Changelog {
            titles: vec![Spanned::<String>::default()],
            ..Default::default()
        };
        assert_yaml_snapshot!(MissingTitle.check(&changelog));
    }

    #[test]
    fn test_duplicate_title() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(DuplicateTitle.check(&changelog));

        let changelog = Changelog {
            titles: vec![
                Spanned::<String>::default(),
                Spanned::new(Span::new(2, 11), "Changelog".to_string()),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(DuplicateTitle.check(&changelog));
    }

    #[test]
    fn test_missing_unreleased() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(MissingUnreleased.check(&changelog));

        let changelog = Changelog {
            sections: vec![Section::Unreleased(Unreleased::default())],
            ..Default::default()
        };
        assert_yaml_snapshot!(MissingUnreleased.check(&changelog));
    }

    #[test]
    fn test_duplicate_unreleased() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(DuplicateUnreleased.check(&changelog));

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
        assert_yaml_snapshot!(DuplicateUnreleased.check(&changelog));
    }

    #[test]
    fn test_invalid_date() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(InvalidDate.check(&changelog));

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
        assert_yaml_snapshot!(InvalidDate.check(&changelog));
    }

    #[test]
    fn test_invalid_yanked() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(InvalidYanked.check(&changelog));

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
        assert_yaml_snapshot!(InvalidYanked.check(&changelog));
    }

    #[test]
    fn test_invalid_change_type() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(InvalidChangeType.check(&changelog));

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
        assert_yaml_snapshot!(InvalidChangeType.check(&changelog));
    }

    #[test]
    fn test_duplicate_change_type() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(DuplicateChangeType.check(&changelog));

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
        assert_yaml_snapshot!(DuplicateChangeType.check(&changelog));
    }

    #[test]
    fn test_empty_section() {
        let changelog = Changelog::default();
        assert_yaml_snapshot!(EmptySection.check(&changelog));

        let changelog = Changelog {
            sections: vec![
                Section::Unreleased(Unreleased {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            changes: vec![Spanned::new(Span::new(0, 0), "Add foo".to_string())],
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
                    heading_span: Span::new(2, usize::MAX),
                    ..Default::default()
                }),
                Section::Release(Release {
                    changes: vec![
                        Changes {
                            kind: Spanned::new(Span::new(0, 0), "Added".to_string()),
                            changes: vec![Spanned::new(Span::new(0, 0), "Add foo".to_string())],
                            ..Default::default()
                        },
                        Changes {
                            kind: Spanned::new(Span::new(3, usize::MAX), "Added".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(EmptySection.check(&changelog));
    }
}
