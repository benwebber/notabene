use serde::Serialize;

use crate::span::{Span, Spanned};

use super::traits::Release as _;
use super::{owned, traits};

type SpannedStr<'a> = Spanned<&'a str>;

#[derive(Default, Serialize)]
pub struct ParsedChangelog<'a> {
    pub(crate) title: Option<SpannedStr<'a>>,
    pub(crate) unreleased: Option<ParsedUnreleased<'a>>,
    pub(crate) releases: Vec<ParsedRelease<'a>>,
    pub(crate) invalid_spans: Vec<InvalidSpan>,
}

#[derive(Default, Serialize)]
pub struct ParsedUnreleased<'a> {
    pub(crate) heading_span: Span,
    pub(crate) url: Option<SpannedStr<'a>>,
    pub(crate) changes: Vec<ParsedChanges<'a>>,
}

#[derive(Default, Serialize)]
pub struct ParsedRelease<'a> {
    pub(crate) heading_span: Span,
    pub(crate) version: SpannedStr<'a>,
    pub(crate) url: Option<SpannedStr<'a>>,
    pub(crate) date: Option<SpannedStr<'a>>,
    pub(crate) yanked: Option<SpannedStr<'a>>,
    pub(crate) changes: Vec<ParsedChanges<'a>>,
}

#[derive(Default, Serialize)]
pub struct ParsedChanges<'a> {
    pub(crate) heading_span: Span,
    pub(crate) kind: SpannedStr<'a>,
    pub(crate) items: Vec<SpannedStr<'a>>,
}

#[derive(Serialize)]
pub enum InvalidSpan {
    InvalidTitle(Span),
    InvalidHeading(Span),
    InvalidLinkReference(Span),
    DuplicateUnreleased(Span),
    DuplicateTitle(Span),
    UnreleasedOutOfOrder(Span),
}

impl<'a> traits::Changelog for ParsedChangelog<'a> {
    type Unreleased = ParsedUnreleased<'a>;
    type Release = ParsedRelease<'a>;

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn unreleased(&self) -> Option<&Self::Unreleased> {
        self.unreleased.as_ref()
    }

    fn releases(&self) -> &[Self::Release] {
        &self.releases
    }
}

impl<'a> traits::Unreleased for ParsedUnreleased<'a> {
    type Changes = ParsedChanges<'a>;

    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn changes(&self) -> &[Self::Changes] {
        &self.changes
    }
}

impl<'a> traits::Release for ParsedRelease<'a> {
    type Changes = ParsedChanges<'a>;

    fn version(&self) -> &str {
        &self.version
    }

    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn date(&self) -> Option<&str> {
        self.date.as_deref()
    }

    fn yanked(&self) -> bool {
        self.yanked.map_or(false, |s| s.value == "[YANKED]")
    }

    fn changes(&self) -> &[Self::Changes] {
        &self.changes
    }
}

impl<'a> traits::Changes for ParsedChanges<'a> {
    fn kind(&self) -> &str {
        &self.kind.value
    }

    fn items(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(|i| &**i)
    }
}

impl<'a> ParsedChangelog<'a> {
    fn to_owned(&self) -> owned::OwnedChangelog {
        owned::OwnedChangelog {
            title: self.title.map(|s| s.value.to_owned()),
            unreleased: self.unreleased.as_ref().map(|u| u.to_owned()),
            releases: self.releases.iter().map(|r| r.to_owned()).collect(),
        }
    }
}

impl<'a> ParsedUnreleased<'a> {
    fn to_owned(&self) -> owned::OwnedUnreleased {
        owned::OwnedUnreleased {
            url: self.url.map(|s| s.value.to_owned()),
            changes: self.changes.iter().map(|c| c.to_owned()).collect(),
        }
    }
}

impl<'a> ParsedRelease<'a> {
    fn to_owned(&self) -> owned::OwnedRelease {
        owned::OwnedRelease {
            version: self.version.value.to_owned(),
            url: self.url.map(|s| s.value.to_owned()),
            date: self.date.map(|s| s.value.to_owned()),
            yanked: self.yanked(),
            changes: self.changes.iter().map(|c| c.to_owned()).collect(),
        }
    }
}

impl<'a> ParsedChanges<'a> {
    fn to_owned(&self) -> owned::OwnedChanges {
        owned::OwnedChanges {
            kind: self.kind.value.to_owned(),
            items: self.items.iter().map(|i| i.value.to_owned()).collect(),
        }
    }
}
