//! Intermediate representation (IR) of a changelog.
//!
//! Parsing a changelog returns an [`ir::Changelog`](crate::ir::Changelog).
//! This model differs from the [`Changelog`](crate::Changelog) in two significant ways:
//!
//!   * Elements include span information.
//!   * It allows multiple titles and unreleased sections.
//!
//! Preserving span information allows lint checks to include the context around diagnostics.
use crate::span::Span;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub(crate) struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Changelog {
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Section {
    Title(Spanned<String>),
    Unreleased(Unreleased),
    Release(Release),
    Invalid(InvalidSection),
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub(crate) struct Unreleased {
    pub heading_span: Span,
    // TODO: Get span.
    pub url: Option<String>,
    pub changes: Vec<Changes>,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub(crate) struct Release {
    pub heading_span: Span,
    pub version: Spanned<String>,
    // TODO: Get span.
    pub url: Option<String>,
    pub date: Option<Spanned<String>>,
    pub yanked: Option<Spanned<String>>,
    pub changes: Vec<Changes>,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub(crate) struct Changes {
    pub heading_span: Span,
    pub kind: Spanned<String>,
    pub changes: Vec<Spanned<String>>,
}

impl<T> Spanned<T> {
    pub(crate) fn new(span: Span, value: T) -> Self {
        Self { value, span }
    }

    pub(crate) fn into_inner(self) -> T {
        self.value
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub(crate) struct InvalidSection {
    pub heading_span: Span,
}
