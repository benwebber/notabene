use std::collections::HashMap;

use crate::rule::Rule;

pub struct Profile {
    pub rules: HashMap<String, Rule>,
}

impl Profile {
    pub fn new(rules: &[Rule]) -> Self {
        let mut rules_by_code = HashMap::new();
        for rule in rules {
            rules_by_code.insert(rule.code().to_string(), *rule);
        }
        Self {
            rules: rules_by_code,
        }
    }

    pub fn is_enabled(&self, rule: Rule) -> bool {
        self.rules.contains_key(rule.code())
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new(&Rule::ALL)
    }
}

impl TryFrom<Vec<String>> for Profile {
    type Error = String;

    fn try_from(codes: Vec<String>) -> Result<Self, Self::Error> {
        let default = Profile::default();
        let mut rules = Vec::new();
        for code in codes.iter() {
            if let Some(rule) = default.rules.get(code) {
                rules.push(*rule)
            } else {
                return Err(format!("Invalid rule code '{}'", &code));
            }
        }
        Ok(Self::new(rules.as_slice()))
    }
}
