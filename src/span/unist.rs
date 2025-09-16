use std::ops::Range;

use serde::{Deserialize, Serialize};

use crate::span::Ranged;

/// A unist [Position](https://github.com/syntax-tree/unist/tree/3.0.0?tab=readme-ov-file#position).
///
/// Like [`Span`](crate::span::Span), `Position` represents a span within the source document.
/// `Position` also includes line and column information.
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Position {
    pub start: Point,
    pub end: Point,
}

/// A unist [Point](https://github.com/syntax-tree/unist/tree/3.0.0?tab=readme-ov-file#point).
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Point {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(start: Point, end: Point) -> Self {
        Position { start, end }
    }
}

impl Ranged<usize> for Position {
    fn range(&self) -> Range<usize> {
        self.start.offset..self.end.offset
    }
}

impl Point {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}
