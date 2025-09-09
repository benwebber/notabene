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

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub(crate) struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Changelog {
    pub titles: Vec<Spanned<String>>,
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Section {
    Unreleased(Unreleased),
    Release(Release),
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

impl Changelog {
    /// Return the first unreleased section, or [`None`] if it does not exist.
    pub fn unreleased(&self) -> Option<&Unreleased> {
        self.sections.iter().find_map(|section| {
            if let Section::Unreleased(unreleased) = section {
                Some(unreleased)
            } else {
                None
            }
        })
    }

    /// Filter sections to releases.
    pub fn releases(&self) -> impl Iterator<Item = &Release> {
        self.sections.iter().filter_map(|s| {
            if let Section::Release(r) = s {
                Some(r)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    // Spans are not useful here, so create dummy spans.
    macro_rules! spanned {
        ($s:literal) => {
            Spanned::new(Span::new(0, 0), $s.to_string())
        };
    }

    // TODO: Figure out a better way to share this complete IR with Changelog tests.
    pub fn changelog() -> Changelog {
        Changelog {
            titles: vec![
                spanned!("Title 1"),
                spanned!("Title 2"),
                spanned!("Title 3"),
            ],
            sections: vec![
                Section::Unreleased(Unreleased {
                    url: Some("https://example.org/unreleased/1".to_string()),
                    changes: vec![Changes {
                        kind: spanned!("Added"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Add foo"), spanned!("Add bar")],
                    }],
                    ..Default::default()
                }),
                Section::Unreleased(Unreleased {
                    url: Some("https://example.org/unreleased/2".to_string()),
                    changes: vec![Changes {
                        kind: spanned!("Added"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Add baz"), spanned!("Add quux")],
                    }],
                    ..Default::default()
                }),
                Section::Release(Release {
                    version: spanned!("1.0.0"),
                    url: Some("https://example.org/release/1.0.0".to_string()),
                    date: Some(spanned!("2025-01-01")),
                    changes: vec![Changes {
                        kind: spanned!("Changed"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Change foo"), spanned!("Change bar")],
                    }],
                    ..Default::default()
                }),
                Section::Release(Release {
                    version: spanned!("0.1.0"),
                    url: Some("https://example.org/release/0.1.0".to_string()),
                    date: Some(spanned!("2024-01-01")),
                    yanked: Some(spanned!("[YANKED]")),
                    changes: vec![Changes {
                        kind: spanned!("Changed"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Change baz"), spanned!("Change quux")],
                    }],
                    ..Default::default()
                }),
            ],
        }
    }

    #[test]
    fn test_unreleased() {
        assert_eq!(Changelog::default().unreleased(), None);
        let mut changelog = changelog();
        // These the non-matching branch by inserting a fake Release before the Unreleased section.
        changelog
            .sections
            .insert(0, Section::Release(Release::default()));
        assert_eq!(
            changelog.unreleased(),
            Some(&Unreleased {
                url: Some("https://example.org/unreleased/1".to_string()),
                changes: vec![Changes {
                    kind: spanned!("Added"),
                    heading_span: Span::default(),
                    changes: vec![spanned!("Add foo"), spanned!("Add bar"),],
                }],
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_releases() {
        assert_eq!(
            Changelog::default().releases().collect::<Vec<&Release>>(),
            Vec::<&Release>::new()
        );
        assert_eq!(
            changelog().releases().collect::<Vec<_>>(),
            vec![
                &Release {
                    version: spanned!("1.0.0"),
                    url: Some("https://example.org/release/1.0.0".to_string()),
                    date: Some(spanned!("2025-01-01")),
                    changes: vec![Changes {
                        kind: spanned!("Changed"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Change foo"), spanned!("Change bar")],
                    }],
                    ..Default::default()
                },
                &Release {
                    version: spanned!("0.1.0"),
                    url: Some("https://example.org/release/0.1.0".to_string()),
                    date: Some(spanned!("2024-01-01")),
                    yanked: Some(spanned!("[YANKED]")),
                    changes: vec![Changes {
                        kind: spanned!("Changed"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Change baz"), spanned!("Change quux")],
                    }],
                    ..Default::default()
                },
            ]
        );
    }
}
