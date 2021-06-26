//use serde::Deserialize;

use chrono::{Local, Timelike, Datelike};

use serde::Deserialize;

use regex::Regex;
use crate::twitter;

fn default_true_value() -> bool {true}

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
    #[serde(default="default_true_value")]
    pub include_images: bool,
    #[serde(default="default_true_value")]
    pub webpage_preview: bool,
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
        if *twinfo.screen_name == self.name {
            println!("RULE: testing rule for {}", self.name);
            let active_range = match &self.active_hours {
                Some(ranges) => {
                    let active_ranges:Vec<&Range> = ranges.into_iter().filter(|range| range.in_range(twinfo.hour)).collect();
                    println!("RULE: Found {} active ranges for hour {}", active_ranges.len(), twinfo.hour);
                    match active_ranges.get(0) {
                        Some(range) => *range,
                        None => {println!("RULE: No active ranges, rule FALSE"); return false},
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
                println!("RULE: Not doing a retweet of followed user, rule FALSE");
                return false;
            }
            if active_today {
                println!("RULE: active today");
                if !active_range.excludes_matches(&twinfo.text) {
                    println!("RULE: no range excludes strings present");
                    if self.includes.is_match(&twinfo.text) {
                        println!("RULE: includes regex matches");
                        return match &self.excludes {
                            Some(regex) => {println!("RULE: range excludes match? rule {}", !regex.is_match(&twinfo.text)); !regex.is_match(&twinfo.text)},
                            None => {println!("RULE: No range excludes to worry about, rule TRUE");true},
                        };
                    } else { println!("RULE: includes regex not matching. rule FALSE")}
                } else { println!("RULE: range excludes regex matches. rule FALSE"); }
            } else { println!("RULE: Inactive today. rule FALSE"); }
        }    

        false
    }
}

impl Range {
    fn in_range(&self, hour: u32) -> bool {
        hour>=self.start && hour<=self.end
    }

    fn excludes_matches(&self, text: &String) -> bool {
        match &self.excludes {
            Some(excludes) => excludes.is_match(&text),
            None => false,
        }
    }
}