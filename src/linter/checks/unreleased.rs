//! `E100` Unreleased
use super::preamble::*;

#[derive(Default)]
pub struct MissingUnreleased {
    found: bool,
}

impl Check for MissingUnreleased {
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
pub struct DuplicateUnreleased {
    spans: Vec<Span>,
    found: bool,
}

impl Check for DuplicateUnreleased {
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

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::ir::*;
    use crate::linter::lint;
    use crate::ruleset::RuleSet;
    use crate::span::Span;

    #[test]
    fn test_missing_unreleased() {
        let ruleset = RuleSet::from([Rule::MissingUnreleased]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

        let changelog = Changelog {
            sections: vec![Section::Unreleased(Unreleased::default())],
            ..Default::default()
        };
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }

    #[test]
    fn test_duplicate_unreleased() {
        let ruleset = RuleSet::from([Rule::DuplicateUnreleased]);

        let changelog = Changelog::default();
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));

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
        assert_yaml_snapshot!(lint(&changelog, &ruleset, None));
    }
}
