use serde::Deserialize;

use crate::rule::*;

//mostly derived from https://gitlab.com/SnejUgal/vk-to-telegram-bot/-/blob/master/src/config.rs

#[derive(Deserialize, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub twitter: TwitterConfig,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Clone)]
pub struct TelegramConfig {
    pub bot_token: String
}


#[derive(Deserialize, Clone)]
pub struct TwitterConfig {
    pub key: String,
    pub secret: String,
}


pub fn parse(path: &'static str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read(path)?;
    let config: Config = ron::de::from_bytes(&contents[..])?;

    Ok(config)
}


