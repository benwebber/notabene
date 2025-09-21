//! Linter implementation.
use std::path::PathBuf;

use crate::changelog::parsed;
use crate::changelog::traits::*;
use crate::diagnostic::Diagnostic;
use crate::ruleset::RuleSet;

#[macro_use]
mod macros;

mod check;
mod checks;

use check::Check;

macro_rules! checks {
    ($($check:ty),* $(,)?) => {
        vec![$(Box::new(<$check>::default())),*]
    }
}

/// A changelog linter.
#[derive(Debug)]
pub struct Linter<'a> {
    ruleset: &'a RuleSet,
    filename: Option<PathBuf>,
}

impl<'a> Linter<'a> {
    /// Create a new linter with the given ruleset.
    pub fn new(ruleset: &'a RuleSet) -> Self {
        Self {
            ruleset,
            filename: None,
        }
    }

    /// Set the filename reported in diagnostics.
    pub fn with_filename<P: Into<PathBuf>>(self, filename: Option<P>) -> Self {
        Self {
            filename: filename.map(|n| n.into()),
            ..self
        }
    }

    /// Lint a changelog.
    pub fn lint(&self, changelog: &parsed::ParsedChangelog) -> Vec<Diagnostic> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let mut checks: Vec<_> = checks()
            .into_iter()
            .filter(|check| self.ruleset.is_enabled(check.rule()))
            .collect();
        for check in checks.iter_mut() {
            check.visit_changelog(changelog);
            if let Some(unreleased) = changelog.unreleased() {
                check.visit_unreleased(unreleased);
                for changes in unreleased.changes() {
                    check.visit_changes(changes);
                }
            }
            for span in &changelog.invalid_spans {
                check.visit_invalid_span(span);
            }
        }
        for release in changelog.releases() {
            for check in checks.iter_mut() {
                check.visit_release(release);
                for changes in release.changes() {
                    check.visit_changes(changes);
                }
            }
        }
        for check in checks.iter_mut() {
            diagnostics.append(
                &mut check
                    .diagnostics()
                    .into_iter()
                    .map(|mut d| {
                        d.path = self.filename.clone();
                        d
                    })
                    .collect(),
            );
        }
        diagnostics
    }
}

impl<'a> Default for Linter<'a> {
    fn default() -> Linter<'a> {
        Linter::new(RuleSet::default_static())
    }
}

fn checks() -> Vec<Box<dyn Check>> {
    checks![
        // E000 Structure
        checks::MissingTitle,
        checks::DuplicateTitle,
        checks::MissingUnreleased,
        checks::DuplicateUnreleased,
        checks::InvalidUnreleasedPosition,
        // E100 Content
        checks::InvalidSectionHeading,
        checks::InvalidTitle,
        checks::EmptySection,
        checks::UnknownChangeType,
        checks::DuplicateChangeType,
        // E200 Release
        checks::InvalidReleaseOrder,
        checks::DuplicateVersion,
        checks::MissingDate,
        checks::InvalidDate,
        checks::InvalidYanked,
        // E300 Links
        checks::UndefinedLinkReference,
    ]
}
