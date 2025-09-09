//! Linter rules.
use serde::{Deserialize, Serialize};

macro_rules! rules {
    ($($rule:ident = ($doc:literal, $code:literal, $message:literal $(,)?)),* $(,)?) => {
        /// A linter rule.
        #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
        #[non_exhaustive]
        pub enum Rule {
            $(#[doc = concat!("`", $code, "`. ")] #[doc = $doc] $rule),*
        }

        impl Rule {
            /// All rules.
            pub const ALL: [Self; [$(stringify!($rule)),*].len()] = [
                $(Self::$rule),*
            ];

            /// Return the error code for this rule.
            pub fn code(&self) -> &str {
                match self {
                    $(Rule::$rule => $code),*
                }
            }

            /// Return the documentation this rule.
            pub fn doc(&self) -> &str {
                match self {
                    $(Rule::$rule => $doc),*
                }
            }

            /// Return the message template for this rule.
            pub fn message(&self) -> &str {
                match self {
                    $(Rule::$rule => $message),*
                }
            }
        }
    };
}

rules! {
    // E000 Document
    MissingTitle = (
        "The title is missing.",
        "E001",
        "Missing title",
    ),
    InvalidTitle = (
        "The title is not plain text.",
        "E002",
        "Invalid title `{}`",
    ),
    DuplicateTitle = (
        "There is a duplicate `h1` in the document.",
        "E003",
        "Duplicate title `{}`",
    ),
    InvalidSectionHeading = (
        "The `h2` is not a valid unreleased or release section heading.",
        "E004",
        "Invalid heading `{}`",
    ),
    // E100 Unreleased
    MissingUnreleased = (
        "The document does not have an unreleased section.",
        "E100",
        "Missing unreleased heading",
    ),
    DuplicateUnreleased = (
        "There is more than one unreleased section heading in the document.",
        "E101",
        "Duplicate unreleased section `{}`",
    ),
    // E200 Release
    InvalidDate = (
        "The date is not in ISO 8601 format.",
        "E200",
        "Invalid date `{}`",
    ),
    InvalidYanked = (
        "The yanked token does not match `[YANKED]`.",
        "E201",
        "Invalid [YANKED] format `{}`",
    ),
    // E300 Changes
    InvalidChangeType = (
        "The change section heading is not a known change type.",
        "E300",
        "Invalid change type `{}`",
    ),
    DuplicateChangeType = (
        "There is more than one change section with the same change type.",
        "E301",
        "Duplicate change type `{}`",
    ),
    // E400 Content
    EmptySection = (
        "A section is unexpectedly empty (e.g. a release with no changes).",
        "E400",
        "Empty section",
    ),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_rule_codes_unique() {
        let mut codes = HashSet::new();
        let mut duplicates: Vec<&str> = vec![];
        for rule in Rule::ALL.iter() {
            if !codes.insert(rule.code()) {
                duplicates.push(rule.code());
            }
        }
        assert_eq!(duplicates, Vec::<&str>::new());
    }
}
