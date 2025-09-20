use serde::Serialize;

use crate::span::{Span, Spanned};

use super::traits::Release as _;
use super::{owned, traits};

type SpannedStr<'a> = Spanned<&'a str>;

#[derive(Default, Serialize)]
pub struct Changelog<'a> {
    pub(crate) title: Option<SpannedStr<'a>>,
    pub(crate) unreleased: Option<Unreleased<'a>>,
    pub(crate) releases: Vec<Release<'a>>,
    pub(crate) invalid_spans: Vec<InvalidSpan>,
}

#[derive(Default, Serialize)]
pub struct Unreleased<'a> {
    pub(crate) heading_span: Span,
    pub(crate) url: Option<SpannedStr<'a>>,
    pub(crate) changes: Vec<Changes<'a>>,
}

#[derive(Default, Serialize)]
pub struct Release<'a> {
    pub(crate) heading_span: Span,
    pub(crate) version: SpannedStr<'a>,
    pub(crate) url: Option<SpannedStr<'a>>,
    pub(crate) date: Option<SpannedStr<'a>>,
    pub(crate) yanked: Option<SpannedStr<'a>>,
    pub(crate) changes: Vec<Changes<'a>>,
}

#[derive(Default, Serialize)]
pub struct Changes<'a> {
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

impl<'a> traits::Changelog for Changelog<'a> {
    type Unreleased = Unreleased<'a>;
    type Release = Release<'a>;

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

impl<'a> traits::Unreleased for Unreleased<'a> {
    type Changes = Changes<'a>;

    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn changes(&self) -> &[Self::Changes] {
        &self.changes
    }
}

impl<'a> traits::Release for Release<'a> {
    type Changes = Changes<'a>;

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

impl<'a> traits::Changes for Changes<'a> {
    fn kind(&self) -> &str {
        &self.kind.value
    }

    fn items(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(|i| &**i)
    }
}

impl<'a> Changelog<'a> {
    fn to_owned(&self) -> owned::Changelog {
        owned::Changelog {
            title: self.title.map(|s| s.value.to_owned()),
            unreleased: self.unreleased.as_ref().map(|u| u.to_owned()),
            releases: self.releases.iter().map(|r| r.to_owned()).collect(),
        }
    }
}

impl<'a> Unreleased<'a> {
    fn to_owned(&self) -> owned::Unreleased {
        owned::Unreleased {
            url: self.url.map(|s| s.value.to_owned()),
            changes: self.changes.iter().map(|c| c.to_owned()).collect(),
        }
    }
}

impl<'a> Release<'a> {
    fn to_owned(&self) -> owned::Release {
        owned::Release {
            version: self.version.value.to_owned(),
            url: self.url.map(|s| s.value.to_owned()),
            date: self.date.map(|s| s.value.to_owned()),
            yanked: self.yanked(),
            changes: self.changes.iter().map(|c| c.to_owned()).collect(),
        }
    }
}

impl<'a> Changes<'a> {
    fn to_owned(&self) -> owned::Changes {
        owned::Changes {
            kind: self.kind.value.to_owned(),
            items: self.items.iter().map(|i| i.value.to_owned()).collect(),
        }
    }
}
