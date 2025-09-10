//! `E000` Structure
use super::preamble::*;

#[derive(Default)]
pub struct MissingTitle {
    found: bool,
}

impl Check for MissingTitle {
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
pub struct DuplicateTitle {
    spans: Vec<Span>,
    found: bool,
}

impl Check for DuplicateTitle {
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

#[derive(Default)]
pub struct UnreleasedNotFirst {
    spans: Vec<Span>,
    found: bool,
}

impl Check for UnreleasedNotFirst {
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

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::*;
    use crate::linter::lint;
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
}
