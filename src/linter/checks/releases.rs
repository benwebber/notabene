use time::Date;
use time::macros::format_description;

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
                if Date::parse(&spanned.value, &format).is_err() {
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

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::*;
    use crate::linter::lint;
    use crate::profile::Profile;
    use crate::span::Span;

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
    fn test_missing_date() {
        let profile = Profile::new(&[Rule::MissingDate]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &profile));

        let changelog = Changelog {
            sections: vec![
                Section::Release(Release {
                    date: Some(Spanned::new(Span::new(0, 11), "2025-01-01".to_string())),
                    ..Default::default()
                }),
                Section::Release(Release {
                    heading_span: Span::new(1, usize::MAX),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &profile));
    }
}
