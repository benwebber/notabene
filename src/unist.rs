//! [Universal Syntax Tree](https://github.com/syntax-tree/unist) (unist) structs.
//!
//! These structs are used when reporting diagnostics instead of this crate's custom
//! [`Span`][crate::span::Span] type.
use serde::{Deserialize, Serialize};

/// A [Position](https://github.com/syntax-tree/unist/tree/3.0.0?tab=readme-ov-file#position), broadly equivalent to a [`Span`](crate::span::Span).
#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Position {
    pub start: Point,
    pub end: Point,
}

/// A [Point](https://github.com/syntax-tree/unist/tree/3.0.0?tab=readme-ov-file#point).
#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
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

impl Point {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}
