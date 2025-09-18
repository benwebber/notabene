pub trait Changelog {
    type Unreleased: Unreleased;
    type Release: Release;

    fn title(&self) -> Option<&str>;
    fn unreleased(&self) -> Option<&Self::Unreleased>;
    fn releases(&self) -> &[Self::Release];
}

pub trait Unreleased {
    type Changes: Changes;

    fn url(&self) -> Option<&str>;
    fn changes(&self) -> &[Self::Changes];
}

pub trait Release {
    type Changes: Changes;

    fn version(&self) -> &str;
    fn url(&self) -> Option<&str>;
    fn date(&self) -> Option<&str>;
    fn yanked(&self) -> bool;
    fn changes(&self) -> &[Self::Changes];
}

pub trait Changes {
    fn kind(&self) -> &str;
    fn items(&self) -> impl Iterator<Item = &str>;
}
