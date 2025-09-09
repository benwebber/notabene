//! Errors returned by this crate.
use std::fmt::Display;

/// The main container for errors returned by this crate.
#[derive(Debug)]
pub enum Error {
    /// An I/O error.
    Io(std::io::Error),
    /// A fatal parsing error.
    Parse(ParseError),
}

/// A fatal parsing error.
///
/// This enum is currently a placeholder for future fatal errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Parse(_) => write!(f, "parse error"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}
