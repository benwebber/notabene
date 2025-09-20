/// A changelog.
pub trait Changelog {
    type Unreleased: Unreleased;
    type Release: Release;

    /// The title.
    fn title(&self) -> Option<&str>;

    /// The unreleased section.
    fn unreleased(&self) -> Option<&Self::Unreleased>;

    /// A list of releases.
    fn releases(&self) -> &[Self::Release];
}

/// An unreleased section.
pub trait Unreleased {
    type Changes: Changes;

    /// A link to unleleased changes..
    fn url(&self) -> Option<&str>;

    /// A list of unreleased changes.
    fn changes(&self) -> &[Self::Changes];
}

/// A release section.
pub trait Release {
    type Changes: Changes;

    /// The release version.
    fn version(&self) -> &str;

    /// A link to the release.
    fn url(&self) -> Option<&str>;

    /// The release date.

    fn date(&self) -> Option<&str>;

    /// Whether the release was yanked.
    fn yanked(&self) -> bool;

    /// A list of changes.
    fn changes(&self) -> &[Self::Changes];
}

/// A list of changes.
pub trait Changes {
    /// The kind or type of change, such as "Added" or "Changed".
    fn kind(&self) -> &str;

    /// The individual change items.
    fn items(&self) -> impl Iterator<Item = &str>;
}
