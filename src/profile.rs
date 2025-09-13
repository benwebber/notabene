use std::collections::{HashMap, HashSet};

use crate::rule::Rule;

pub struct Profile {
    pub rules: HashSet<Rule>,
}

impl Profile {
    pub fn new(rules: HashSet<Rule>) -> Self {
        Self { rules }
    }

    pub fn is_enabled(&self, rule: Rule) -> bool {
        self.rules.contains(&rule)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::from(Rule::ALL)
    }
}

impl<const N: usize> From<[Rule; N]> for Profile {
    fn from(rules: [Rule; N]) -> Self {
        Self::new(rules.iter().copied().collect())
    }
}
