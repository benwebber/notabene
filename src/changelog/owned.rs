//! Owned versions of changelog types.
use super::traits;

pub struct OwnedChangelog {
    pub(crate) title: Option<String>,
    pub(crate) unreleased: Option<OwnedUnreleased>,
    pub(crate) releases: Vec<OwnedRelease>,
}

pub struct OwnedUnreleased {
    pub(crate) url: Option<String>,
    pub(crate) changes: Vec<OwnedChanges>,
}

pub struct OwnedRelease {
    pub(crate) version: String,
    pub(crate) url: Option<String>,
    pub(crate) date: Option<String>,
    pub(crate) yanked: bool,
    pub(crate) changes: Vec<OwnedChanges>,
}

pub struct OwnedChanges {
    pub(crate) kind: String,
    pub(crate) items: Vec<String>,
}

impl traits::Changelog for OwnedChangelog {
    type Unreleased = OwnedUnreleased;
    type Release = OwnedRelease;

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

impl traits::Unreleased for OwnedUnreleased {
    type Changes = OwnedChanges;

    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn changes(&self) -> &[Self::Changes] {
        &self.changes
    }
}

impl traits::Release for OwnedRelease {
    type Changes = OwnedChanges;

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
        self.yanked
    }

    fn changes(&self) -> &[Self::Changes] {
        &self.changes
    }
}

impl traits::Changes for OwnedChanges {
    fn kind(&self) -> &str {
        &self.kind
    }

    fn items(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(|s| s.as_str())
    }
}
