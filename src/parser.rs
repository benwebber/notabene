//! Parse a changelog as its [intermediate representation](crate::ir::Changelog).
use crate::ast::*;
use crate::diagnostic::Diagnostic;
use crate::ir::*;
use crate::rule::Rule;
use crate::span::{Span, SpanIterator};
use std::iter::Peekable;

use pulldown_cmark as md;

/// Parse a changelog.
///
/// Parsing never fails, although the parser may return diagnostics indicating that it cannot parse
/// some element of the document correctly.
pub fn parse(s: &str) -> (Changelog, Vec<Diagnostic>) {
    let mut changelog = Changelog::default();
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let mut blocks = Parser::new(s).peekable();
    while let Some(block) = blocks.next() {
        match block {
            Block::Heading(heading @ Heading { level: 1, .. }) => {
                let section = match get_heading_span(&heading) {
                    Some(span) => {
                        let spanned = Spanned::new(span, s[span.range()].to_string());
                        Section::Title(spanned)
                    }
                    None => Section::InvalidTitle(InvalidTitle {
                        heading_span: heading.span,
                    }),
                };
                changelog.sections.push(section);
            }
            Block::Heading(heading @ Heading { level: 2, .. }) => {
                let (section, mut section_diagnostics) = parse_section(s, &heading, &mut blocks);
                diagnostics.append(&mut section_diagnostics);
                if let Some(sec) = section {
                    changelog.sections.push(sec);
                }
            }
            _ => {}
        }
    }
    // Detect broken links.
    let callback = |link: md::BrokenLink| {
        changelog.broken_links.push(link.span.into());
        None
    };
    let parser = md::Parser::new_with_broken_link_callback(s, md::Options::empty(), Some(callback));
    for _event in parser {}
    (changelog, diagnostics)
}

fn parse_section(
    s: &str,
    heading: &Heading,
    blocks: &mut Peekable<Parser>,
) -> (Option<Section>, Vec<Diagnostic>) {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();
    let section = match heading.inlines.as_slice() {
        // Unreleased
        [Inline::Link(l)] if &s[l.content.span.range()] == "Unreleased" => {
            let (changes, mut change_diagnostics) = parse_changes(s, blocks);
            diagnostics.append(&mut change_diagnostics);
            Some(Section::Unreleased(Unreleased {
                heading_span: heading.span,
                url: Some(l.target.clone()),
                changes,
            }))
        }
        // Release
        [Inline::Link(l), Inline::Literal(t)] => {
            let mut release = Release {
                heading_span: heading.span,
                version: Spanned::new(l.content.span, s[l.content.span.range()].to_string()),
                url: Some(l.target.clone()),
                ..Default::default()
            };
            let mut spans = SpanIterator::new(&s[t.span.range()]);
            // Skip hyphen.
            spans.next();
            if let Some(span) = spans.next().map(|s| s.offset(t.span.start)) {
                let date = &s[span.range()];
                release.date = Some(Spanned::new(span, date.to_string()))
            }
            if let Some(span) = spans.next().map(|s| s.offset(t.span.start)) {
                let yanked = &s[span.range()];
                release.yanked = Some(Spanned::new(span, yanked.to_string()));
            }
            let (changes, mut change_diagnostics) = parse_changes(s, blocks);
            release.changes = changes;
            diagnostics.append(&mut change_diagnostics);
            Some(Section::Release(release))
        }
        _ => Some(Section::Invalid(InvalidSection {
            heading_span: heading.span,
        })),
    };
    (section, diagnostics)
}

fn parse_changes(s: &str, blocks: &mut Peekable<Parser>) -> (Vec<Changes>, Vec<Diagnostic>) {
    let mut sections: Vec<(Span, Spanned<String>, Vec<Spanned<String>>)> = Vec::new();
    let diagnostics: Vec<Diagnostic> = Vec::new();
    let mut current_kind: Option<String> = None;
    let mut current_changes: Vec<Spanned<String>> = Vec::new();
    let mut current_heading_span: Span = Span::default();

    while let Some(block) = blocks.peek() {
        match block {
            Block::Heading(heading @ Heading { level: 3, .. }) => {
                if let Some(kind) = current_kind.take() {
                    // TODO: more accurate span for kind
                    sections.push((
                        current_heading_span,
                        Spanned::new(current_heading_span, kind),
                        std::mem::take(&mut current_changes),
                    ));
                }
                current_kind = get_heading_text(s, heading).map(|s| s.to_string());
                current_heading_span = heading.span;
                blocks.next();
            }
            Block::List(l) => {
                if current_kind.is_some() {
                    current_changes.extend(
                        l.items
                            .iter()
                            .map(|i| Spanned::new(i.span, s[i.span.range()].to_string())),
                    );
                }
                blocks.next();
            }
            _ => break,
        }
    }

    if let Some(kind) = current_kind.take() {
        sections.push((
            current_heading_span,
            Spanned::new(current_heading_span, kind),
            current_changes,
        ));
    }

    let changes = sections
        .into_iter()
        .map(|(heading_span, kind, spanned)| Changes {
            heading_span,
            kind,
            changes: spanned,
        })
        .collect();

    (changes, diagnostics)
}

fn get_heading_span(heading: &Heading) -> Option<Span> {
    match heading.inlines.as_slice() {
        [Inline::Literal(Literal { span, .. })] => Some(*span),
        _ => None,
    }
}

fn get_heading_text<'a>(s: &'a str, heading: &Heading) -> Option<&'a str> {
    match heading.inlines.as_slice() {
        [Inline::Literal(Literal { span, .. })] => Some(&s[span.range()]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    #[test]
    fn test_parse() {
        let source = "
# Title 1
# Title 2

## [Unreleased][]

### Removed

* Remove foo

## [1.0.0] - 2025-01-01

### Added

* Add foo
* Add bar

## [0.1.0] - 2024-01-01

### Added

* Add baz
* Add quux

## [broken] - 2024-01-01

[Unreleased]: https://example.org/unreleased
[1.0.0]: https://example.org/release/1.0.0
[0.1.0]: https://example.org/release/0.1.0
        ";
        let (changelog, diagnostics) = parse(source);
        assert_yaml_snapshot!((changelog, diagnostics));
    }
}
