use std::io::Write;
use std::path::Path;

use crate::diagnostic::Diagnostic;
use crate::locator::Locator;

pub(crate) mod context;
pub(crate) mod format;
pub(crate) mod formatter;

pub(crate) use context::Context;
pub(crate) use format::Format;
pub(crate) use formatter::Formatter;

pub fn report<W: Write>(
    w: &mut W,
    diagnostics: &[Diagnostic],
    source: &str,
    path: Option<&Path>,
    locator: &Locator,
    format: Format,
) -> std::io::Result<()> {
    let context = Context {
        source,
        path,
        locator,
    };
    match format {
        Format::Short => formatter::ShortFormatter.format(w, diagnostics, &context),
        Format::Full => formatter::FullFormatter.format(w, diagnostics, &context),
        Format::Json => formatter::JsonFormatter.format(w, diagnostics, &context),
        Format::JsonLines => formatter::JsonLinesFormatter.format(w, diagnostics, &context),
    }
}
