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
pub struct InvalidTitle {
    spans: Vec<Span>,
}

impl Check for InvalidTitle {
    fn rule(&self) -> Rule {
        Rule::InvalidTitle
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::InvalidTitle(invalid) = section {
            self.spans.push(invalid.heading_span);
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
pub struct InvalidSectionHeading {
    spans: Vec<Span>,
}

impl Check for InvalidSectionHeading {
    fn rule(&self) -> Rule {
        Rule::InvalidSectionHeading
    }

    fn spans(&self) -> &[Span] {
        self.spans.as_slice()
    }

    fn visit_section(&mut self, section: &Section) {
        if let Section::Invalid(invalid) = section {
            self.spans.push(invalid.heading_span);
        }
    }
}

#[derive(Default)]
pub struct UnreleasedOutOfOrder {
    spans: Vec<Span>,
    found: bool,
}

impl Check for UnreleasedOutOfOrder {
    fn rule(&self) -> Rule {
        Rule::UnreleasedOutOfOrder
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

    use crate::ir::{self, *};
    use crate::linter::lint;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_missing_title() {
        let ruleset = RuleSet::from([Rule::MissingTitle]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

        let changelog = Changelog {
            sections: vec![Section::Title(Spanned::<&str>::default())],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }

    #[test]
    fn test_invalid_title() {
        let ruleset = RuleSet::from([Rule::InvalidTitle]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

        let changelog = Changelog {
            sections: vec![Section::InvalidTitle(ir::InvalidTitle {
                heading_span: Span::new(1, usize::MAX),
            })],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }

    #[test]
    fn test_duplicate_title() {
        let ruleset = RuleSet::from([Rule::DuplicateTitle]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

        let changelog = Changelog {
            sections: vec![
                Section::Title(Spanned::<&str>::default()),
                Section::Title(Spanned::new(Span::new(2, 11), "Changelog")),
            ],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }

    #[test]
    fn test_invalid_section() {
        let ruleset = RuleSet::from([Rule::InvalidSectionHeading]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

        let changelog = Changelog {
            sections: vec![Section::Invalid(InvalidSection {
                heading_span: Span::new(1, usize::MAX),
            })],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }

    #[test]
    fn test_unreleased_not_first() {
        let ruleset = RuleSet::from([Rule::UnreleasedOutOfOrder]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

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
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }
}
