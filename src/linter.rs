//! Linter implementation.
use std::path::PathBuf;

use crate::changelog::parsed;
use crate::changelog::traits::*;
use crate::diagnostic::Diagnostic;
use crate::rule::Rule;
use crate::ruleset::RuleSet;
use crate::span::Span;

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

#[derive(Default)]
pub(crate) struct Context {
    diagnostics: Vec<Diagnostic>,
}

impl Context {
    fn report(&mut self, rule: Rule, span: Option<Span>) {
        self.diagnostics.push(Diagnostic::new(rule, span));
    }
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
        let mut checks: Vec<_> = checks()
            .into_iter()
            .filter(|check| self.ruleset.is_enabled(check.rule()))
            .collect();
        let mut context = Context::default();
        for check in checks.iter_mut() {
            check.visit_changelog(&mut context, changelog);
            if let Some(unreleased) = changelog.unreleased() {
                check.visit_unreleased(&mut context, unreleased);
                for changes in unreleased.changes() {
                    check.visit_changes(&mut context, changes);
                }
            }
            for span in &changelog.invalid_spans {
                check.visit_invalid_span(&mut context, span);
            }
        }
        for release in changelog.releases() {
            for check in checks.iter_mut() {
                check.visit_release(&mut context, release);
                for changes in release.changes() {
                    check.visit_changes(&mut context, changes);
                }
            }
        }
        for check in checks.iter_mut() {
            check.finalize(&mut context);
        }
        context
            .diagnostics
            .into_iter()
            .map(|mut diagnostic| {
                diagnostic.path = self.filename.clone();
                diagnostic
            })
            .collect()
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
