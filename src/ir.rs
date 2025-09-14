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

use serde::Serialize;

#[derive(Debug, Default, Eq, Hash, PartialEq, Serialize)]
pub(crate) struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct Changelog<'a> {
    pub sections: Vec<Section<'a>>,
    pub broken_links: Vec<Span>,
}

#[derive(Debug, Serialize)]
pub enum Section<'a> {
    Title(Spanned<&'a str>),
    Unreleased(Unreleased<'a>),
    Release(Release<'a>),
    Invalid(InvalidSection),
    // TODO: If Title held something like Vec<Inline>, it would be possible
    // to get rid of InvalidTitle and validate Title in the linter.
    InvalidTitle(InvalidTitle),
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub(crate) struct Unreleased<'a> {
    pub heading_span: Span,
    // TODO: Get span.
    pub url: Option<String>,
    pub changes: Vec<Changes<'a>>,
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub(crate) struct Release<'a> {
    pub heading_span: Span,
    pub version: Spanned<&'a str>,
    // TODO: Get span.
    pub url: Option<String>,
    pub date: Option<Spanned<&'a str>>,
    pub yanked: Option<Spanned<&'a str>>,
    pub changes: Vec<Changes<'a>>,
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub(crate) struct Changes<'a> {
    pub heading_span: Span,
    pub kind: Spanned<&'a str>,
    pub changes: Vec<Spanned<&'a str>>,
}

impl<T> Spanned<T> {
    pub(crate) fn new(span: Span, value: T) -> Self {
        Self { value, span }
    }

    pub(crate) fn into_inner(self) -> T {
        self.value
    }
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub(crate) struct InvalidSection {
    pub heading_span: Span,
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub(crate) struct InvalidTitle {
    pub heading_span: Span,
}
