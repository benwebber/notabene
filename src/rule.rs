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

            /// Return the documentation for this rule.
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
    // E000 Structure
    MissingTitle = (
        "The title is missing.",
        "E001",
        "Missing title",
    ),
    DuplicateTitle = (
        "There is a duplicate `h1` in the document.",
        "E002",
        "Duplicate title `{}`",
    ),
    MissingUnreleased = (
        "The document does not have an unreleased section.",
        "E003",
        "Missing unreleased heading",
    ),
    DuplicateUnreleased = (
        "There is more than one unreleased section heading in the document.",
        "E004",
        "Duplicate unreleased section `{}`",
    ),
    InvalidUnreleasedPosition = (
        "The unreleased section is not the first section in the document.",
        "E005",
        "Unreleased section must come before releases.",
    ),
    // E100 Content
    InvalidTitle = (
        "The title is not plain text.",
        "E100",
        "Invalid title `{}`",
    ),
    InvalidSectionHeading = (
        "The `h2` is not a valid unreleased or release section heading.",
        "E101",
        "Invalid heading `{}`",
    ),
    EmptySection = (
        "A section is unexpectedly empty (e.g. a release with no changes).",
        "E102",
        "Empty section",
    ),
    UnknownChangeType = (
        "The change section heading is not a known change type.",
        "E103",
        "Invalid change type `{}`",
    ),
    DuplicateChangeType = (
        "There is more than one change section with the same change type.",
        "E104",
        "Duplicate change type `{}`",
    ),
    // E200 Release
    InvalidReleaseOrder = (
        "The release is not in reverse chronological order.",
        "E200",
        "Release out of order `{}`",
    ),
    DuplicateVersion = (
        "There is more than one release for this version in the document.",
        "E201",
        "Duplicate version `{}`",
    ),
    MissingDate = (
        "The release is missing a date",
        "E202",
        "Release missing date",
    ),
    InvalidDate = (
        "The date is not in ISO 8601 format.",
        "E203",
        "Invalid date `{}`",
    ),
    InvalidYanked = (
        "The yanked token does not match `[YANKED]`.",
        "E204",
        "Invalid [YANKED] format `{}`",
    ),
    // E500 Links
    UndefinedLinkReference = (
        "The target reference does not exist.",
        "E300",
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
