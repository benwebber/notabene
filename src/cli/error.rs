use std::fmt;
use std::io::Error as IoError;

use toml::de::Error as TomlDeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Toml(TomlDeError),
    Check,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Toml(e) => write!(f, "TOML parse error: {}", e),
            Self::Check => write!(f, "checks failed"),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::Io(err)
    }
}

impl From<TomlDeError> for Error {
    fn from(err: TomlDeError) -> Self {
        Self::Toml(err)
    }
}
