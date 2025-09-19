use super::traits;

pub struct Changelog {
    pub(crate) title: Option<String>,
    pub(crate) unreleased: Option<Unreleased>,
    pub(crate) releases: Vec<Release>,
}

pub struct Unreleased {
    pub(crate) url: Option<String>,
    pub(crate) changes: Vec<Changes>,
}

pub struct Release {
    pub(crate) version: String,
    pub(crate) url: Option<String>,
    pub(crate) date: Option<String>,
    pub(crate) yanked: bool,
    pub(crate) changes: Vec<Changes>,
}

pub struct Changes {
    pub(crate) kind: String,
    pub(crate) items: Vec<String>,
}

impl traits::Changelog for Changelog {
    type Unreleased = Unreleased;
    type Release = Release;

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

impl traits::Unreleased for Unreleased {
    type Changes = Changes;

    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn changes(&self) -> &[Self::Changes] {
        &self.changes
    }
}

impl traits::Release for Release {
    type Changes = Changes;

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

impl traits::Changes for Changes {
    fn kind(&self) -> &str {
        &self.kind
    }

    fn items(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(|s| s.as_str())
    }
}
