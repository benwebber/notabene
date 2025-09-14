//! Parse a Markdown document as an iterator of block elements.
use pulldown_cmark as md;

use super::*;
use crate::span::Span;

type EventSpanIterator<'a> = dyn Iterator<Item = (md::Event<'a>, std::ops::Range<usize>)> + 'a;

pub struct Parser<'a> {
    inner: Box<EventSpanIterator<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(inner: Box<EventSpanIterator<'a>>) -> Self {
        Self { inner }
    }
}

impl Literal {
    fn new<S: Into<Span>>(span: S) -> Self {
        Self { span: span.into() }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((event, range)) = self.inner.next() {
            match event {
                md::Event::Start(md::Tag::Heading { level, .. }) => {
                    let inlines = parse_inlines(&mut self.inner, md::TagEnd::Heading(level));
                    let level = match level {
                        md::HeadingLevel::H1 => 1,
                        md::HeadingLevel::H2 => 2,
                        md::HeadingLevel::H3 => 3,
                        md::HeadingLevel::H4 => 4,
                        md::HeadingLevel::H5 => 5,
                        md::HeadingLevel::H6 => 6,
                    };
                    // Remove trailing newline.
                    let span = Span::new(range.start, range.end - 1);
                    let heading = Heading {
                        span,
                        level,
                        inlines,
                    };
                    return Some(Block::Heading(heading));
                }
                md::Event::Start(md::Tag::Paragraph) => {
                    let lit = Literal::new(range);
                    return Some(Block::Paragraph(lit));
                }
                md::Event::Start(md::Tag::List(None)) => {
                    let items = parse_items(&mut self.inner);
                    let list = List {
                        span: range.into(),
                        items,
                    };
                    return Some(Block::List(list));
                }
                _ => {}
            }
        }
        None
    }
}

fn parse_inlines<'a>(parser: &mut EventSpanIterator<'a>, until: md::TagEnd) -> Vec<Inline> {
    let mut inlines = vec![];
    while let Some((event, span)) = parser.next() {
        match event {
            md::Event::Start(md::Tag::Link { dest_url, .. }) => {
                let content_span = read_span_until(parser, md::TagEnd::Link);
                let content = Literal::new(content_span);
                let target: String = dest_url.into();
                let link = Link {
                    span: span.into(),
                    content,
                    target,
                };
                inlines.push(Inline::Link(link))
            }
            md::Event::Text(_) => {
                let span = Span::from(span);
                inlines.push(Inline::Literal(Literal::new(span)));
            }
            md::Event::End(tag) if tag == until => break,
            _ => {}
        }
    }
    inlines
}

fn parse_items<'a>(parser: &mut EventSpanIterator<'a>) -> Vec<Literal> {
    let mut items = vec![];
    while let Some((event, _)) = parser.next() {
        match event {
            md::Event::Start(md::Tag::Item) => {
                let item_span = read_span_until(parser, md::TagEnd::Item);
                items.push(Literal::new(item_span));
            }
            md::Event::End(md::TagEnd::List(false)) => break,
            _ => {}
        }
    }
    items
}

fn read_span_until<'a>(parser: &mut EventSpanIterator<'a>, until: md::TagEnd) -> Span {
    let mut span = Span::default();
    for (event, span_) in &mut *parser {
        match event {
            md::Event::End(tag) if tag == until => {
                break;
            }
            _ => {
                if span.start == 0 {
                    span.start = span_.start
                }
                span.end = span_.end
            }
        }
    }
    span
}

#[cfg(test)]
mod tests {

    use super::*;

    fn parser(s: &str) -> Parser {
        let parser = md::Parser::new(s).into_offset_iter();
        let inner = md::utils::TextMergeWithOffset::new(parser);
        Parser::new(Box::new(inner))
    }

    macro_rules! snapshot {
        ($s:literal) => {
            let parser = parser(&$s.trim());
            let blocks: Vec<Block> = parser.collect();
            insta::assert_yaml_snapshot!(blocks);
        };
    }

    #[test]
    fn test_parse_h1() {
        snapshot!("# Changelog");
        snapshot!(
            "
Changelog
=========
        "
        );
    }

    #[test]
    fn test_parse_h2() {
        snapshot!(
            "
## [Unreleased]

[Unreleased]: https://example.org/
        "
        );
        snapshot!(
            "
[Unreleased]
------------

[Unreleased]: https://example.org/
        "
        );
        snapshot!("## [Unreleased](https://example.org/)");
        snapshot!(
            "
[Unreleased](https://example.org/)
----------------------------------
"
        );
        snapshot!(
            "
## [1.6.8] - 2038-01-19 [YANKED]

[1.6.8]: https://example.org/
"
        );
        snapshot!(
            "
[1.6.8] - 2038-01-19 [YANKED]
-----------------------------

[1.6.8]: https://example.org/
"
        );
        snapshot!("## [1.6.8](https://example.org/) - 2038-01-19 [YANKED]");
        snapshot!(
            "
[1.6.8](https://example.org/) - 2038-01-19 [YANKED]
---------------------------------------------------
"
        );
    }

    #[test]
    fn test_parse_h3() {
        snapshot!("### Added");
    }

    #[test]
    fn test_parse_paragraph() {
        snapshot!(
            "
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        ");
    }

    #[test]
    fn test_parse_list() {
        snapshot!(
            "
* Lorem ipsum dolor sit amet
* Consectetur adipiscing elit
* Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
        "
        );
    }
}
