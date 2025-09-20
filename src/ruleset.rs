use std::collections::HashSet;
use std::sync::LazyLock;

use crate::rule::Rule;

static DEFAULT: LazyLock<RuleSet> = LazyLock::new(|| RuleSet::from(Rule::ALL));

/// A set of linter rules.
#[derive(Debug)]
pub struct RuleSet {
    rules: HashSet<Rule>,
}

impl RuleSet {
    pub fn new<I: Into<HashSet<Rule>>>(rules: I) -> Self {
        Self {
            rules: rules.into(),
        }
    }

    pub fn is_enabled(&self, rule: Rule) -> bool {
        self.rules.contains(&rule)
    }

    pub(crate) fn default_static() -> &'static Self {
        &DEFAULT
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::from(Rule::ALL)
    }
}

impl<const N: usize> From<[Rule; N]> for RuleSet {
    fn from(rules: [Rule; N]) -> Self {
        Self::new(rules.iter().copied().collect::<HashSet<Rule>>())
    }
}
