//! Represent changelogs in Keep a Changelog format.
pub mod owned;
pub mod parsed;
pub(crate) mod traits;

pub use owned::OwnedChangelog;
pub use parsed::ParsedChangelog;
pub use traits::{Changelog, Changes, Release, Unreleased};
