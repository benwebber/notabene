//! Linter implementation.
use crate::diagnostic::Diagnostic;
use crate::ir::{Changelog, Changes, Section};
use crate::profile::Profile;
use crate::rule::Rule;
use crate::span::Span;

mod check;
mod checks;

use check::Check;

pub fn lint(changelog: &Changelog, profile: &Profile) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let mut checks: Vec<_> = checks()
        .into_iter()
        .filter(|check| profile.is_enabled(check.rule()))
        .collect();

    for section in &changelog.sections {
        for check in checks.iter_mut() {
            check.visit_section(section);
            match section {
                Section::Unreleased(unreleased) => {
                    for changes in &unreleased.changes {
                        check.visit_changes(changes);
                    }
                }
                Section::Release(release) => {
                    for changes in &release.changes {
                        check.visit_changes(changes);
                    }
                }
                _ => {}
            }
        }
    }

    for check in checks.iter_mut() {
        diagnostics.append(&mut check.diagnostics());
    }

    diagnostics
}

fn checks() -> Vec<Box<dyn Check>> {
    vec![
        // E000 Document
        Box::new(checks::MissingTitle::default()),
        Box::new(checks::DuplicateTitle::default()),
        Box::new(checks::UnreleasedNotFirst::default()),
        //// E100 Unreleased
        Box::new(checks::MissingUnreleased::default()),
        Box::new(checks::DuplicateUnreleased::default()),
        //// E200 Release
        Box::new(checks::InvalidDate::default()),
        Box::new(checks::InvalidYanked::default()),
        Box::new(checks::MissingDate::default()),
        Box::new(checks::ReleaseOutOfOrder::default()),
        Box::new(checks::DuplicateVersion::default()),
        //// E300 Changes
        Box::new(checks::InvalidChangeType::default()),
        Box::new(checks::DuplicateChangeType::default()),
        //// E400 Content
        Box::new(checks::EmptySection::default()),
    ]
}
