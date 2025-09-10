//! Represent changelogs in Keep a Changelog format.
use serde::{Deserialize, Serialize};

use crate::ir;

/// A changelog in Keep a Changelog format.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Changelog {
    pub title: Option<String>,
    pub unreleased: Option<Unreleased>,
    pub releases: Vec<Release>,
}

/// The unreleased section.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Unreleased {
    pub url: Option<String>,
    pub changes: Vec<Changes>,
}

/// A release section.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Release {
    pub version: String,
    pub url: Option<String>,
    pub date: Option<String>,
    pub yanked: bool,
    pub changes: Vec<Changes>,
}

/// Changes associated with a release.
#[derive(Debug, Deserialize, Serialize)]
pub struct Changes {
    pub kind: String,
    pub changes: Vec<String>,
}

/// Create a `Changelog` from an [`ir::Changelog`].
/// Use the first title and the first unreleased section as `title` and `unreleased`, respectively.
impl From<ir::Changelog> for Changelog {
    fn from(changelog: ir::Changelog) -> Self {
        let mut title: Option<String> = None;
        let mut unreleased: Option<Unreleased> = None;
        let mut releases: Vec<Release> = Vec::new();
        for section in changelog.sections.into_iter() {
            match section {
                ir::Section::Title(t) => {
                    if title.is_none() {
                        title = Some(t.into_inner())
                    }
                }
                ir::Section::Unreleased(u) => {
                    if unreleased.is_none() {
                        unreleased = Some(u.into())
                    }
                }
                ir::Section::Release(r) => releases.push(r.into()),
            }
        }
        Self {
            title,
            unreleased,
            releases,
        }
    }
}

impl From<ir::Unreleased> for Unreleased {
    fn from(unreleased: ir::Unreleased) -> Self {
        Self {
            url: unreleased.url,
            changes: unreleased.changes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ir::Release> for Release {
    fn from(release: ir::Release) -> Self {
        Self {
            version: release.version.into_inner(),
            url: release.url,
            date: release.date.map(|d| d.into_inner()),
            yanked: release.yanked.is_some(),
            changes: release.changes.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ir::Changes> for Changes {
    fn from(changes: ir::Changes) -> Self {
        Self {
            kind: changes.kind.into_inner(),
            changes: changes
                .changes
                .into_iter()
                .map(|c| c.into_inner())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_yaml_snapshot;

    use crate::span::Span;

    // Spans are not useful here, so create dummy spans.
    macro_rules! spanned {
        ($s:literal) => {
            ir::Spanned::new(Span::new(0, 0), $s.to_string())
        };
    }

    #[test]
    fn test_from_ir() {
        let ir_changelog = ir::Changelog {
            sections: vec![
                ir::Section::Title(spanned!("Title 1")),
                ir::Section::Title(spanned!("Title 2")),
                ir::Section::Title(spanned!("Title 3")),
                ir::Section::Unreleased(ir::Unreleased {
                    url: Some("https://example.org/unreleased/1".to_string()),
                    changes: vec![ir::Changes {
                        kind: spanned!("Added"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Add foo"), spanned!("Add bar")],
                    }],
                    ..Default::default()
                }),
                ir::Section::Unreleased(ir::Unreleased {
                    url: Some("https://example.org/unreleased/2".to_string()),
                    changes: vec![ir::Changes {
                        kind: spanned!("Added"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Add baz"), spanned!("Add quux")],
                    }],
                    ..Default::default()
                }),
                ir::Section::Release(ir::Release {
                    version: spanned!("1.0.0"),
                    url: Some("https://example.org/release/1.0.0".to_string()),
                    date: Some(spanned!("2025-01-01")),
                    changes: vec![ir::Changes {
                        kind: spanned!("Changed"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Change foo"), spanned!("Change bar")],
                    }],
                    ..Default::default()
                }),
                ir::Section::Release(ir::Release {
                    version: spanned!("0.1.0"),
                    url: Some("https://example.org/release/0.1.0".to_string()),
                    date: Some(spanned!("2024-01-01")),
                    yanked: Some(spanned!("[YANKED]")),
                    changes: vec![ir::Changes {
                        kind: spanned!("Changed"),
                        heading_span: Span::default(),
                        changes: vec![spanned!("Change baz"), spanned!("Change quux")],
                    }],
                    ..Default::default()
                }),
            ],
        };
        assert_yaml_snapshot!(Changelog::from(ir_changelog));
    }
}
