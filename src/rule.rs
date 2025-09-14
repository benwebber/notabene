//! Linter rules.
use std::collections::HashMap;
use std::sync::LazyLock;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

pub static RULES_BY_CODE: LazyLock<HashMap<String, Rule>> = LazyLock::new(|| {
    Rule::ALL
        .iter()
        .map(|rule| (rule.code().to_string(), *rule))
        .collect()
});

macro_rules! rules {
    ($($rule:ident = ($doc:literal, $code:literal, $message:literal $(,)?)),* $(,)?) => {
        /// A linter rule.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
        #[non_exhaustive]
        pub enum Rule {
            $(#[doc = concat!("`", $code, "`. ")] #[doc = $doc] $rule),*
        }

        impl Rule {
            /// All rules.
            pub const ALL: [Self; [$(stringify!($rule)),*].len()] = [
                $(Self::$rule),*
            ];

            /// All codes.
            pub(crate) const CODES: [&str; [$(stringify!($code)),*].len()] = [
                $($code),*
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
    UnreleasedOutOfOrder = (
        "The unreleased section is not the first section in the document.",
        "E005",
        "Unreleased section must come before releases.",
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
    MissingDate = (
        "The release is missing a date",
        "E202",
        "Release missing date",
    ),
    ReleaseOutOfOrder = (
        "The release is not in reverse chronological order.",
        "E203",
        "Release out of order `{}`",
    ),
    DuplicateVersion = (
        "There is more than one release for this version in the document.",
        "E204",
        "Duplicate version `{}`",
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
    // E500 Links
    LinkReferenceDoesNotExist = (
        "The target reference does not exist.",
        "E500",
        "Link reference does not exist: `{}`",
    ),
}

impl TryFrom<String> for Rule {
    type Error = String;

    fn try_from(code: String) -> Result<Self, Self::Error> {
        RULES_BY_CODE
            .get(&code.to_uppercase())
            .copied()
            .ok_or_else(|| format!("invalid rule code '{}'", code))
    }
}

impl TryFrom<&str> for Rule {
    type Error = String;

    fn try_from(code: &str) -> Result<Self, Self::Error> {
        Rule::try_from(code.to_string())
    }
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleVisitor;

        impl<'de> Visitor<'de> for RuleVisitor {
            type Value = Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a rule code")
            }

            fn visit_str<E>(self, value: &str) -> Result<Rule, E>
            where
                E: de::Error,
            {
                RULES_BY_CODE
                    .get(value)
                    .ok_or(de::Error::unknown_variant(value, &Rule::CODES))
                    .copied()
            }
        }

        deserializer.deserialize_str(RuleVisitor)
    }
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
