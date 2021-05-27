//use serde::Deserialize;
use egg_mode::tweet::Tweet;
use chrono::{Local, Timelike};

use serde::Deserialize;

use regex::Regex;


#[derive(Clone, Deserialize)]
pub struct Range {
    start: u32,
    end: u32,
    #[serde(with = "serde_regex")]
    excludes: Option<Regex>,
}

#[derive(Deserialize, Clone)]
pub struct Rule {
    name: String,
    #[serde(with = "serde_regex")]
    includes: Regex, 
    #[serde(with = "serde_regex")]
    excludes: Option<Regex>,
    active_hours: Option<Vec<Range>>,
    active_days: Option<String>,
}

const ALL_DAY_RANGE:Range = Range { start: 0, end: 23, excludes: None };

impl Rule {
    pub fn matches(&self, tweet: &Tweet, followed_users:&Vec<u64>) -> bool {
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

            //No retweets of users we follow
            if tweet.retweeted.unwrap_or(false) && followed_users.contains(&tweet.user.as_ref().unwrap().id) {
                return false;
            }

            if !active_range.excludes_present(&tweet.text) {
                if self.includes.is_match(&tweet.text.to_ascii_lowercase()) {
                    return true;
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
            Some(excludes) => excludes.is_match(&test_text),
            None => false,
        }
    }
}