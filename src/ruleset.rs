use std::collections::HashSet;

use crate::rule::Rule;

/// A set of linter rules.
pub struct RuleSet {
    pub rules: HashSet<Rule>,
}

impl RuleSet {
    pub fn new(rules: HashSet<Rule>) -> Self {
        Self { rules }
    }

    pub fn is_enabled(&self, rule: Rule) -> bool {
        self.rules.contains(&rule)
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::from(Rule::ALL)
    }
}

impl<const N: usize> From<[Rule; N]> for RuleSet {
    fn from(rules: [Rule; N]) -> Self {
        Self::new(rules.iter().copied().collect())
    }
}
