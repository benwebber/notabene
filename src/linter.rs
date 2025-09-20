//! Linter implementation.
use std::path::PathBuf;

use crate::changelog::parsed;
use crate::changelog::traits::*;
use crate::diagnostic::Diagnostic;
use crate::ruleset::RuleSet;

mod check;
mod checks;

use check::Check;

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
    pub fn lint(&self, changelog: &parsed::Changelog) -> Vec<Diagnostic> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let mut checks: Vec<_> = checks()
            .into_iter()
            .filter(|check| self.ruleset.is_enabled(check.rule()))
            .collect();
        for check in checks.iter_mut() {
            check.visit_changelog(&changelog);
            if let Some(unreleased) = changelog.unreleased() {
                check.visit_unreleased(&unreleased);
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

/// Lint a changelog with the default ruleset.
pub fn lint(changelog: &parsed::Changelog) -> Vec<Diagnostic> {
    let ruleset = RuleSet::default();
    let linter = Linter::new(&ruleset);
    linter.lint(changelog)
}

fn checks() -> Vec<Box<dyn Check>> {
    vec![
        // E000 Document
        Box::new(checks::MissingTitle::default()),
        Box::new(checks::InvalidTitle::default()),
        Box::new(checks::DuplicateTitle::default()),
        Box::new(checks::InvalidSectionHeading::default()),
        Box::new(checks::UnreleasedOutOfOrder::default()),
        // E100 Unreleased
        Box::new(checks::MissingUnreleased::default()),
        Box::new(checks::DuplicateUnreleased::default()),
        // E200 Release
        Box::new(checks::InvalidDate::default()),
        Box::new(checks::InvalidYanked::default()),
        Box::new(checks::MissingDate::default()),
        Box::new(checks::ReleaseOutOfOrder::default()),
        Box::new(checks::DuplicateVersion::default()),
        // E300 Changes
        Box::new(checks::InvalidChangeType::default()),
        Box::new(checks::DuplicateChangeType::default()),
        // E400 Content
        Box::new(checks::EmptySection::default()),
        // E500 Links
        Box::new(checks::LinkReferenceDoesNotExist::default()),
    ]
}
