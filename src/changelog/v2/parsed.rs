use crate::ir;
use crate::span::{Span, Spanned};

use super::traits::Release as _;
use super::{owned, traits};

type SpannedStr<'a> = Spanned<&'a str>;

pub struct Changelog<'a> {
    pub(crate) title: Option<SpannedStr<'a>>,
    pub(crate) unreleased: Option<Unreleased<'a>>,
    pub(crate) releases: Vec<Release<'a>>,
    pub(crate) invalid_spans: Vec<InvalidSpan>,
}

pub struct Unreleased<'a> {
    pub(crate) heading_span: Span,
    pub(crate) url: Option<SpannedStr<'a>>,
    pub(crate) changes: Vec<Changes<'a>>,
}

pub struct Release<'a> {
    pub(crate) heading_span: Span,
    pub(crate) version: SpannedStr<'a>,
    pub(crate) url: Option<SpannedStr<'a>>,
    pub(crate) date: Option<SpannedStr<'a>>,
    pub(crate) yanked: Option<SpannedStr<'a>>,
    pub(crate) changes: Vec<Changes<'a>>,
}

pub struct Changes<'a> {
    pub(crate) heading_span: Span,
    pub(crate) kind: SpannedStr<'a>,
    pub(crate) items: Vec<SpannedStr<'a>>,
}

pub enum InvalidSpan {
    InvalidTitle(Span),
    InvalidHeading(Span),
    InvalidLinkReference(Span),
    DuplicateUnreleased(Span),
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

impl<'a> From<&'a ir::Changelog<'a>> for Changelog<'a> {
    fn from(ir: &'a ir::Changelog<'a>) -> Self {
        let mut title: Option<SpannedStr<'a>> = None;
        let mut unreleased: Option<Unreleased<'a>> = None;
        let mut releases: Vec<Release<'a>> = Vec::new();
        let mut invalid_spans: Vec<InvalidSpan> = Vec::new();
        for section in ir.sections.iter() {
            match section {
                ir::Section::Title(t) => {
                    if title.is_none() {
                        title = Some(SpannedStr {
                            span: t.span,
                            value: t.value,
                        })
                    }
                }
                ir::Section::Unreleased(u) => match unreleased {
                    Some(_) => invalid_spans.push(InvalidSpan::DuplicateUnreleased(u.heading_span)),
                    None => unreleased = Some(u.into()),
                },
                ir::Section::Release(r) => releases.push(r.into()),
                ir::Section::Invalid(i) => {
                    invalid_spans.push(InvalidSpan::InvalidHeading(i.heading_span))
                }
                ir::Section::InvalidTitle(i) => {
                    invalid_spans.push(InvalidSpan::InvalidTitle(i.heading_span))
                }
            }
        }
        let mut broken_links = ir
            .broken_links
            .iter()
            .map(|s| InvalidSpan::InvalidLinkReference(*s))
            .collect();
        invalid_spans.append(&mut broken_links);
        Self {
            title,
            unreleased,
            releases,
            invalid_spans,
        }
    }
}

impl<'a> From<&'a ir::Unreleased<'a>> for Unreleased<'a> {
    fn from(ir: &'a ir::Unreleased<'a>) -> Self {
        // TODO: Get proper span for URL.
        let url = match &ir.url {
            Some(s) => Some(SpannedStr {
                span: Span::default(),
                value: s.as_str(),
            }),
            None => None,
        };
        let changes = ir.changes.iter().map(|c| Changes::from(c)).collect();
        Self {
            heading_span: ir.heading_span,
            url,
            changes,
        }
    }
}

impl<'a> From<&'a ir::Release<'a>> for Release<'a> {
    fn from(ir: &'a ir::Release<'a>) -> Self {
        let version = SpannedStr {
            span: ir.version.span,
            value: ir.version.value,
        };
        // TODO: Get proper span for URL.
        let url = match &ir.url {
            Some(s) => Some(SpannedStr {
                span: Span::default(),
                value: s.as_str(),
            }),
            None => None,
        };
        let date = match &ir.date {
            Some(s) => Some(SpannedStr {
                span: s.span,
                value: s.value,
            }),
            None => None,
        };
        let yanked = match &ir.yanked {
            Some(s) => Some(SpannedStr {
                span: s.span,
                value: s.value,
            }),
            None => None,
        };
        let changes = ir.changes.iter().map(|c| Changes::from(c)).collect();
        Self {
            heading_span: ir.heading_span,
            version,
            url,
            date,
            yanked,
            changes,
        }
    }
}

impl<'a> From<&'a ir::Changes<'a>> for Changes<'a> {
    fn from(ir: &'a ir::Changes<'a>) -> Self {
        let kind = SpannedStr {
            span: ir.kind.span,
            value: ir.kind.value,
        };
        let items = ir
            .changes
            .iter()
            .map(|s| SpannedStr {
                span: s.span,
                value: s.value,
            })
            .collect();
        Self {
            heading_span: ir.heading_span,
            kind,
            items,
        }
    }
}
