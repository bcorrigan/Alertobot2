use serde::Deserialize;
use egg_mode::tweet::Tweet;
use chrono::{Local, Timelike};

#[derive(Clone, Deserialize)]
pub struct Range {
    start: u32,
    end: u32,
    excludes: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Rule {
    name: String,
    includes: String,
    excludes: Option<String>,
    active_hours: Option<Vec<Range>>,
    active_days: Option<String>,
}

const ALL_DAY_RANGE:Range = Range { start: 0, end: 23, excludes: None };

impl Rule {
    pub fn matches(&self, tweet: &Tweet) -> bool {
        let hour = Local::now().hour();

        if tweet.user.as_ref().unwrap().screen_name == self.name {
            let active_range = match &self.active_hours {
                Some(ranges) => {
                    let active_ranges:Vec<&Range> = ranges.into_iter().filter(|range| range.in_range(hour)).collect();
                    match active_ranges.get(0) {
                        Some(range) => *range,
                        None => return false,
                    }                    
                },    
                None => &ALL_DAY_RANGE
            };


            if !active_range.excludes_present(&tweet.text) {
                for rule_str in self.includes.split(',').into_iter() {
                    if tweet.text.to_ascii_lowercase().contains(rule_str) {
                        return true;
                    }
                }
            }
        }    

        false
    }
}

impl Range {
    fn in_range(&self, hour: u32) -> bool {
        hour>=self.start && hour<=self.end
    }

    fn excludes_present(&self, text: &String) -> bool {
        let test_text = text.to_ascii_lowercase();
        match &self.excludes {
            Some(excludes) => {
                for exclude in excludes.split(",").into_iter() {
                    if test_text.contains(exclude) {
                        return true;
                    }
                }
                false
            },
            None => false,
        }
    }
}