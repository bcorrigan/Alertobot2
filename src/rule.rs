//use serde::Deserialize;

use chrono::{Local, Timelike, Datelike};

use serde::Deserialize;

use regex::Regex;
use crate::twitter;


#[derive(Clone, Deserialize)]
pub struct Range {
    pub start: u32,
    pub end: u32,
    #[serde(with = "serde_regex", default)]
    pub excludes: Option<Regex>,
}
#[derive(Clone, Deserialize)]
pub struct Chat {
    pub chat: i64,
}

#[derive(Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub chats: Vec<Chat>,
    #[serde(with = "serde_regex")]
    pub includes: Regex, 
    #[serde(with = "serde_regex", default)]
    pub excludes: Option<Regex>,
    #[serde(default)]
    pub active_hours: Option<Vec<Range>>,
    #[serde(with = "serde_regex", default)]
    pub active_days: Option<Regex>,
}

pub struct TweetInfo<'a> {
    pub text: String,
    pub hour: u32,
    pub day: String,
    pub retweeted: bool,
    pub user: u64,
    pub rtuser: u64,
    pub screen_name: &'a String,
    pub followed_users: &'a Vec<u64>
}

const ALL_DAY_RANGE:Range = Range { start: 0, end: 23, excludes: None };

impl Rule {
    pub fn matches(&self, twinfo:&TweetInfo) -> bool {
        let text = twinfo.text.to_ascii_lowercase();

        if *twinfo.screen_name == self.name {
            let active_range = match &self.active_hours {
                Some(ranges) => {
                    let active_ranges:Vec<&Range> = ranges.into_iter().filter(|range| range.in_range(twinfo.hour)).collect();
                    match active_ranges.get(0) {
                        Some(range) => *range,
                        None => return false,
                    }                    
                },    
                None => &ALL_DAY_RANGE
            };

            let active_today = match &self.active_days {
                Some(regex) => regex.is_match(&twinfo.day),
                None => true,
            };

            //No retweets of users we follow
            if twinfo.retweeted && twinfo.followed_users.contains(&twinfo.rtuser) { //TODO get ultimate user ID!
                println!("5");
                return false;
            }
            if active_today {
                if !active_range.excludes_present(&text) {
                    if self.includes.is_match(&text) {
                        return match &self.excludes {
                            Some(regex) => !regex.is_match(&text),
                            None => true,
                        };
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
        match &self.excludes {
            Some(excludes) => excludes.is_match(&text),
            None => false,
        }
    }
}