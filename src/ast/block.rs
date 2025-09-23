//! AST block elements.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::span::Span;

/// A block element.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Block {
    Heading(Heading),
    Paragraph(Literal),
    List(List),
}

/// An inline element.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Inline {
    Link(Link),
    Literal(Literal),
}

/// A section heading.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Heading {
    pub span: Span,
    pub level: usize,
    pub inlines: Vec<Inline>,
}

/// A link.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Link {
    pub span: Span,
    pub content: Literal,
    // TODO: Figure out how to obtain span for this.
    pub target: String,
}

/// Literal markup.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Literal {
    pub span: Span,
}

/// An unordered list.
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct List {
    pub span: Span,
    pub items: Vec<Literal>,
}
