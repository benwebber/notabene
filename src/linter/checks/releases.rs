use std::cmp::Ordering;
use std::collections::HashSet;

use time::Date;
use time::macros::format_description;
use version_compare::{Cmp, Version};

use super::preamble::*;

use crate::changelog::parsed;

#[derive(Default)]
pub struct InvalidDate;

impl Check for InvalidDate {
    fn rule(&self) -> Rule {
        Rule::InvalidDate
    }

    fn visit_release(&mut self, context: &mut Context, release: &parsed::ParsedRelease) {
        let format = format_description!("[year]-[month]-[day]");
        if let Some(spanned) = &release.date {
            if Date::parse(spanned.value, &format).is_err() {
                context.report(self.rule(), Some(spanned.span));
            }
        }
    }
}

#[derive(Default)]
pub struct InvalidYanked;

impl Check for InvalidYanked {
    fn rule(&self) -> Rule {
        Rule::InvalidYanked
    }

    fn visit_release(&mut self, context: &mut Context, release: &parsed::ParsedRelease) {
        if let Some(spanned) = &release.yanked {
            if spanned.value != "[YANKED]" {
                context.report(self.rule(), Some(spanned.span));
            }
        }
    }
}

#[derive(Default)]
pub struct MissingDate;

impl Check for MissingDate {
    fn rule(&self) -> Rule {
        Rule::MissingDate
    }

    fn visit_release(&mut self, context: &mut Context, release: &parsed::ParsedRelease) {
        if release.date.is_none() {
            context.report(self.rule(), Some(release.heading_span));
        }
    }
}

#[derive(Default)]
pub struct InvalidReleaseOrder {
    info: Vec<ReleaseInfo>,
}

struct ReleaseInfo {
    span: Span,
    version: String,
    date: Option<String>,
}

impl Check for InvalidReleaseOrder {
    fn rule(&self) -> Rule {
        Rule::InvalidReleaseOrder
    }

    fn visit_release(&mut self, _context: &mut Context, release: &parsed::ParsedRelease) {
        self.info.push(ReleaseInfo {
            span: release.heading_span,
            version: release.version.value.to_string(),
            date: release.date.as_ref().map(|s| s.value.to_string()),
        });
    }

    fn finalize(&mut self, context: &mut Context) {
        let spans = self.info.as_slice().windows(2).filter_map(|window| {
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
        });
        for span in spans {
            context.report(self.rule(), Some(span));
        }
    }
}

#[derive(Default)]
pub struct DuplicateVersion {
    versions: HashSet<String>,
}

impl Check for DuplicateVersion {
    fn rule(&self) -> Rule {
        Rule::DuplicateVersion
    }

    fn visit_release(&mut self, context: &mut Context, release: &parsed::ParsedRelease) {
        if !self.versions.insert(release.version.value.to_string()) {
            context.report(self.rule(), Some(release.version.span));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::changelog::parsed::{ParsedChangelog, ParsedRelease};
    use crate::linter::Linter;
    use crate::ruleset::RuleSet;
    use crate::span::{Span, Spanned};

    #[test]
    fn test_invalid_date() {
        let ruleset = RuleSet::from([Rule::InvalidDate]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            releases: vec![
                ParsedRelease {
                    ..Default::default()
                },
                ParsedRelease {
                    date: Some(Spanned::new(Span::new(0, 9), "2038-01-19")),
                    ..Default::default()
                },
                ParsedRelease {
                    date: Some(Spanned::new(Span::new(1, usize::MAX), "2001-01-00")),
                    ..Default::default()
                },
                ParsedRelease {
                    date: Some(Spanned::new(Span::new(2, usize::MAX), "foo")),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_invalid_yanked() {
        let ruleset = RuleSet::from([Rule::InvalidYanked]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            releases: vec![
                ParsedRelease::default(),
                ParsedRelease {
                    yanked: Some(Spanned::new(Span::new(0, 9), "[YANKED]")),
                    ..Default::default()
                },
                ParsedRelease {
                    yanked: Some(Spanned::new(Span::new(1, usize::MAX), "[ZANKED]")),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_missing_date() {
        let ruleset = RuleSet::from([Rule::MissingDate]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            releases: vec![
                ParsedRelease {
                    date: Some(Spanned::new(Span::new(0, 11), "2025-01-01")),
                    ..Default::default()
                },
                ParsedRelease {
                    heading_span: Span::new(1, usize::MAX),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_invalid_release_order() {
        let ruleset = RuleSet::from([Rule::InvalidReleaseOrder]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            releases: vec![
                ParsedRelease {
                    date: Some(Spanned::new(Span::default(), "2025-12-31")),
                    ..Default::default()
                },
                ParsedRelease {
                    date: Some(Spanned::new(Span::default(), "2025-01-01")),
                    ..Default::default()
                },
                ParsedRelease {
                    heading_span: Span::new(1, usize::MAX),
                    date: Some(Spanned::new(Span::default(), "2025-06-01")),
                    ..Default::default()
                },
                ParsedRelease {
                    date: Some(Spanned::new(Span::default(), "2025-01-01")),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }

    #[test]
    fn test_duplicate_version() {
        let ruleset = RuleSet::from([Rule::DuplicateVersion]);
        let linter = Linter::new(&ruleset);

        let changelog = ParsedChangelog::default();
        assert_yaml_snapshot!(linter.lint(&changelog));

        let changelog = ParsedChangelog {
            releases: vec![
                ParsedRelease {
                    version: Spanned::new(Span::default(), "1.0.0"),
                    ..Default::default()
                },
                ParsedRelease {
                    version: Spanned::new(Span::new(1, usize::MAX), "1.0.0"),
                    ..Default::default()
                },
                ParsedRelease {
                    version: Spanned::new(Span::default(), "0.1.0"),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(linter.lint(&changelog));
    }
}
