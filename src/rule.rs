use serde::Deserialize;
use egg_mode::tweet::Tweet;

#[derive(Clone, Copy)]
pub struct Rule {
    pub name: &'static str,
    pub includes: &'static str,
    pub excludes: &'static str,
}

#[derive(Deserialize)]
pub struct OwnedRule {
    name: String,
    includes: String,
    excludes: String,
}

impl OwnedRule {
    pub fn into_static(self) -> Rule {
        Rule {
            name: Box::leak(Box::new(self.name)),
            includes: Box::leak(Box::new(self.includes)),
            excludes: Box::leak(Box::new(self.excludes)),
        }
    }
}

impl Rule {
    pub fn matches(&self, tweet: &Tweet) -> bool {
        if tweet.user.as_ref().unwrap().screen_name == self.name {
            for rule_str in self.includes.split(',').into_iter() {
                if tweet.text.to_ascii_lowercase().contains(rule_str) {
                    return true;
                }
            }
        }

        false
    }
}