use std::cmp::Ordering;
use std::collections::HashSet;

use time::Date;
use time::macros::format_description;
use version_compare::{Cmp, Version};

use super::preamble::*;

#[derive(Default)]
pub struct InvalidDate {
    spans: Vec<Span>,
}

impl Check for InvalidDate {
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
                if Date::parse(spanned.value, &format).is_err() {
                    self.spans.push(spanned.span);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct InvalidYanked {
    spans: Vec<Span>,
}

impl Check for InvalidYanked {
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

#[derive(Default)]
pub struct MissingDate {
    spans: Vec<Span>,
}

impl Check for MissingDate {
    fn rule(&self) -> Rule {
        Rule::MissingDate
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Release(release) = section {
            if release.date.is_none() {
                self.spans.push(release.heading_span);
            }
        }
    }
}

#[derive(Default)]
pub struct ReleaseOutOfOrder {
    info: Vec<ReleaseInfo>,
}

struct ReleaseInfo {
    span: Span,
    version: String,
    date: Option<String>,
}

impl Check for ReleaseOutOfOrder {
    fn rule(&self) -> Rule {
        Rule::ReleaseOutOfOrder
    }

    fn spans(&self) -> &[Span] {
        unimplemented!()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Release(release) = section {
            self.info.push(ReleaseInfo {
                span: release.heading_span,
                version: release.version.value.to_string(),
                date: release.date.as_ref().map(|s| s.value.to_string()),
            })
        }
    }

    fn diagnostics(&self) -> Vec<Diagnostic> {
        self.info
            .as_slice()
            .windows(2)
            .filter_map(|window| {
                let prev = &window[0];
                let curr = &window[1];
                let prev_version = Version::from(&prev.version);
                let curr_version = Version::from(&curr.version);
                let (Some(prev_version), Some(curr_version)) = (prev_version, curr_version) else {
                    // Skip if either version is invalid.
                    return None;
                };
                // Sort by date in reverse chronological order. If the date is None, sort it last.
                let date_cmp = match (&curr.date, &prev.date) {
                    (Some(curr_date), Some(prev_date)) => curr_date.cmp(prev_date),
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (None, None) => Ordering::Equal,
                };
                // Then sort by version in reverse order.
                let out_of_order = match date_cmp {
                    Ordering::Less => false,
                    Ordering::Equal => matches!(curr_version.compare(&prev_version), Cmp::Gt),
                    Ordering::Greater => true,
                };
                if out_of_order { Some(curr.span) } else { None }
            })
            .map(|span| Diagnostic::new(self.rule(), Some(span)))
            .collect()
    }
}

#[derive(Default)]
pub struct DuplicateVersion {
    spans: Vec<Span>,
    versions: HashSet<String>,
}

impl Check for DuplicateVersion {
    fn rule(&self) -> Rule {
        Rule::DuplicateVersion
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Release(release) = section {
            if !self.versions.insert(release.version.value.to_string()) {
                self.spans.push(release.version.span);
            }
        }
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
    fn test_invalid_date() {
        let ruleset = RuleSet::from([Rule::InvalidDate]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(0, 9), "2038-01-19")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(1, 10), "2001-01-00")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(2, 5), "foo")),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset));
    }

    #[test]
    fn test_invalid_yanked() {
        let ruleset = RuleSet::from([Rule::InvalidYanked]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    ..Default::default()
                }),
                Section::Release(Release {
                    yanked: Some(Spanned::new(Span::new(0, 9), "[YANKED]")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    yanked: Some(Spanned::new(Span::new(1, 10), "[ZANKED]")),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset));
    }

    #[test]
    fn test_missing_date() {
        let ruleset = RuleSet::from([Rule::MissingDate]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(0, 11), "2025-01-01")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    heading_span: Span::new(1, usize::MAX),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset));
    }

    #[test]
    fn test_release_out_of_order() {
        let ruleset = RuleSet::from([Rule::ReleaseOutOfOrder]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    date: Some(Spanned::new(Span::default(), "2025-12-31")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::default(), "2025-01-01")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    heading_span: Span::new(1, usize::MAX),
                    date: Some(Spanned::new(Span::default(), "2025-06-01")),
                    ..Default::default()
                }),
                Section::Release(Release {
                    date: Some(Spanned::new(Span::default(), "2025-01-01")),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset));
    }

    #[test]
    fn test_duplicate_version() {
        let ruleset = RuleSet::from([Rule::DuplicateVersion]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    version: Spanned::new(Span::default(), "1.0.0"),
                    ..Default::default()
                }),
                Section::Release(Release {
                    version: Spanned::new(Span::new(1, usize::MAX), "1.0.0"),
                    ..Default::default()
                }),
                Section::Release(Release {
                    version: Spanned::new(Span::default(), "0.1.0"),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset));
    }
}
