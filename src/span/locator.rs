use std::cmp::Ordering;

use super::{Point, Position, Ranged, Span};

/// Locates spans in the source document.
pub struct Locator<'a> {
    source: &'a str,
    line_spans: Vec<Span>,
}

impl<'a> Locator<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut line_spans = Vec::new();
        let mut start = 0;
        for (end, _) in source.match_indices('\n') {
            line_spans.push(Span::new(start, end));
            start = end + 1;
        }
        if start <= source.len() {
            // No trailing newline.
            line_spans.push(Span::new(start, source.len()))
        }
        Self { source, line_spans }
    }

    /// Return the unist Point for the given offset.
    fn point(&self, offset: usize) -> Point {
        if self.line_spans.is_empty() {
            return Point::default();
        }
        let cmp = |span: &Span| {
            if offset < span.start {
                Ordering::Greater
            } else if offset > span.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        };
        let i = match self.line_spans.binary_search_by(cmp) {
            Ok(i) => i,
            Err(i) => i
                .saturating_sub(1)
                .min(self.line_spans.len().saturating_sub(1)),
        };
        Point::new(
            i + 1,
            offset.saturating_sub(self.line_spans[i].start) + 1,
            offset,
        )
    }

    /// Return the unist Position for the given span.
    pub fn position<R: Ranged<usize>>(&self, ranged: &R) -> Position {
        let range = ranged.range();
        Position::new(self.point(range.start), self.point(range.end))
    }

    /// Return the contents of line number `line`.
    pub(crate) fn line(&self, line: usize) -> &'a str {
        &self.source[self.line_spans[line.saturating_sub(1)].range()]
    }

    /// Return the number of lines in the source.
    pub(crate) fn lines(&self) -> usize {
        self.line_spans.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::span::SpanIterator;

    #[test]
    fn test_position() {
        let s = "foo

bar

quux";
        let locator = Locator::new(s);
        let spans: Vec<Span> = SpanIterator::new(s).collect();
        assert_eq!(
            locator.position(&spans[0]),
            Position::new(Point::new(1, 1, 0), Point::new(1, 4, 3)),
        );
        assert_eq!(
            locator.position(&spans[1]),
            Position::new(Point::new(3, 1, 5), Point::new(3, 4, 8)),
        );
        assert_eq!(
            locator.position(&spans[2]),
            Position::new(Point::new(5, 1, 10), Point::new(5, 5, 14)),
        );
    }

    #[test]
    fn test_line() {
        let s = "foo bar baz\nfoobar foobaz";
        let locator = Locator::new(s);
        assert_eq!(locator.line(2), "foobar foobaz");
    }
}
