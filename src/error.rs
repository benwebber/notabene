//! Errors returned by this crate.
use std::fmt::Display;

/// A fatal parsing error.
///
/// This enum is currently a placeholder for future fatal errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error")
    }
}
