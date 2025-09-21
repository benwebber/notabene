use crate::changelog::parsed;
use crate::rule::Rule;

use super::Context;

pub(crate) trait Check {
    /// Return the `Rule` this check evaluates.
    fn rule(&self) -> Rule;

    fn visit_changelog(&mut self, _context: &mut Context, _changelog: &parsed::ParsedChangelog) {}
    fn visit_unreleased(&mut self, _context: &mut Context, _unreleased: &parsed::ParsedUnreleased) {
    }
    fn visit_release(&mut self, _context: &mut Context, _release: &parsed::ParsedRelease) {}
    fn visit_changes(&mut self, _context: &mut Context, _changes: &parsed::ParsedChanges) {}
    fn visit_invalid_span(&mut self, _context: &mut Context, _span: &parsed::InvalidSpan) {}

    fn finalize(&mut self, _context: &mut Context) {}
}
