//! Work with spans within the source document.
use std::ops::Range;

use serde::{Deserialize, Serialize};

mod index;
mod iter;

pub(crate) use index::Index;
pub(crate) use iter::SpanIterator;

/// A span within the source document.
#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct Span {
    /// The start offset of the span.
    pub start: usize,
    /// The end offset of the span.
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Offset a span by `offset`.
    pub fn offset(&self, offset: usize) -> Self {
        Self {
            start: offset + self.start,
            end: offset + self.end,
        }
    }

    /// Convert a span into a `Range`, suitable for indexing a `&str`.
    pub fn range(&self) -> Range<usize> {
        (*self).into()
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset() {
        let span = Span::new(0, 10);
        assert_eq!(span.offset(5), Span::new(5, 15));
    }

    #[test]
    fn test_range() {
        let span = Span::new(0, 10);
        assert_eq!(span.range(), span.into())
    }
}
