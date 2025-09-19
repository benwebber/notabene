//! Work with spans within the source document.
use std::ops::{Deref, Range};

use serde::{Deserialize, Serialize};

mod iter;
mod locator;
mod ranged;
mod unist;

pub(crate) use iter::SpanIterator;
pub use locator::Locator;
pub(crate) use ranged::Ranged;
pub use unist::{Point, Position};

/// A span within the source document.
///
/// Use [`Locator`] to convert a `Span` to a [`Position`].
#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct Span {
    /// The start offset of the span.
    pub start: usize,
    /// The end offset of the span.
    pub end: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize)]
pub(crate) struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

impl Ranged<usize> for Span {
    fn range(&self) -> Range<usize> {
        (*self).into()
    }
}

impl<'a> Deref for Spanned<&'a str> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value
    }
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
