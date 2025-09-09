//! A simplified Markdown document AST.
//!
//! The AST is a sequence of [`Block`] elements, associated with spans in the original document.
mod block;
mod parser;

pub(crate) use block::*;
pub(crate) use parser::Parser;
