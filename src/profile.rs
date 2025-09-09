use std::collections::HashSet;

use crate::rule::Rule;

pub struct Profile {
    rules: HashSet<Rule>,
}

impl Profile {
    pub fn new(rules: &[Rule]) -> Self {
        Self {
            rules: rules.iter().copied().collect(),
        }
    }

    pub fn is_enabled(&self, rule: Rule) -> bool {
        self.rules.contains(&rule)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new(&Rule::ALL)
    }
}
