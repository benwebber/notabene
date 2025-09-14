//! Parse a changelog as its [intermediate representation](crate::ir::Changelog).
use crate::ast::{self, Block, Heading, Inline, Literal};
use crate::ir::{
    Changelog, Changes, InvalidSection, InvalidTitle, Release, Section, Spanned, Unreleased,
};
use crate::span::{Span, SpanIterator};
use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;

use pulldown_cmark as md;

/// Parse a changelog.
///
/// Parsing never fails.
pub fn parse<'a>(s: &'a str) -> Changelog<'a> {
    let mut changelog = Changelog::default();
    let broken_links = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let broken_links = Rc::clone(&broken_links);
        move |link: md::BrokenLink| {
            broken_links.borrow_mut().push(link.span.into());
            None
        }
    };
    let parser = md::Parser::new_with_broken_link_callback(s, md::Options::empty(), Some(callback));
    let parser = md::utils::TextMergeWithOffset::new(parser.into_offset_iter());
    let mut blocks = ast::Parser::new(Box::new(parser)).peekable();
    while let Some(block) = blocks.next() {
        match block {
            Block::Heading(heading @ Heading { level: 1, .. }) => {
                let section = match get_heading_span(&heading) {
                    Some(span) => {
                        let spanned = Spanned::new(span, &s[span.range()]);
                        Section::Title(spanned)
                    }
                    None => Section::InvalidTitle(InvalidTitle {
                        heading_span: heading.span,
                    }),
                };
                changelog.sections.push(section);
            }
            Block::Heading(heading @ Heading { level: 2, .. }) => {
                let section = parse_section(s, &heading, &mut blocks);
                if let Some(sec) = section {
                    changelog.sections.push(sec);
                }
            }
            _ => {}
        }
    }
    // `blocks` still holds a reference to `callback` through the parser.
    drop(blocks);
    if let Ok(cell) = Rc::try_unwrap(broken_links) {
        changelog.broken_links = cell.into_inner();
    }
    changelog
}

fn parse_section<'a>(
    s: &'a str,
    heading: &Heading,
    blocks: &mut Peekable<ast::Parser<'a>>,
) -> Option<Section<'a>> {
    match heading.inlines.as_slice() {
        // Unreleased
        [Inline::Link(l)] if &s[l.content.span.range()] == "Unreleased" => {
            let changes = parse_changes(s, blocks);
            Some(Section::Unreleased(Unreleased {
                heading_span: heading.span,
                url: Some(l.target.to_string()),
                changes,
            }))
        }
        // Release
        [Inline::Link(l), Inline::Literal(t)] => {
            let mut release = Release {
                heading_span: heading.span,
                version: Spanned::new(l.content.span, &s[l.content.span.range()]),
                url: Some(l.target.to_string()),
                ..Default::default()
            };
            let mut spans = SpanIterator::new(&s[t.span.range()]);
            // Skip hyphen.
            spans.next();
            if let Some(span) = spans.next().map(|s| s.offset(t.span.start)) {
                let date = &s[span.range()];
                release.date = Some(Spanned::new(span, date))
            }
            if let Some(span) = spans.next().map(|s| s.offset(t.span.start)) {
                let yanked = &s[span.range()];
                release.yanked = Some(Spanned::new(span, yanked));
            }
            let changes = parse_changes(s, blocks);
            release.changes = changes;
            Some(Section::Release(release))
        }
        _ => Some(Section::Invalid(InvalidSection {
            heading_span: heading.span,
        })),
    }
}

fn parse_changes<'a>(s: &'a str, blocks: &mut Peekable<ast::Parser<'a>>) -> Vec<Changes<'a>> {
    let mut sections: Vec<(Span, Spanned<&'a str>, Vec<Spanned<&'a str>>)> = Vec::new();
    let mut current_kind: Option<&'a str> = None;
    let mut current_changes: Vec<Spanned<&'a str>> = Vec::new();
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
                current_kind = get_heading_text(s, heading);
                current_heading_span = heading.span;
                blocks.next();
            }
            Block::List(l) => {
                if current_kind.is_some() {
                    current_changes.extend(
                        l.items
                            .iter()
                            .map(|i| Spanned::new(i.span, &s[i.span.range()])),
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

    sections
        .into_iter()
        .map(|(heading_span, kind, spanned)| Changes {
            heading_span,
            kind,
            changes: spanned,
        })
        .collect()
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
        let changelog = parse(source);
        assert_yaml_snapshot!(changelog);
    }
}
