use serde::Deserialize;

mod telegram;

pub use telegram::*;

//mostly derived from https://gitlab.com/SnejUgal/vk-to-telegram-bot/-/blob/master/src/config.rs

pub struct Config {
    pub telegram: TelegramConfig,
}

#[derive(Deserialize)]
struct OwnedConfig {
    pub telegram: OwnedTelegramConfig,
}

impl OwnedConfig {
    fn into_static(self) -> Config {
        Config {
            telegram: self.telegram.into_static(),
        }
    }
}

pub fn parse(path: &'static str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read(path)?;
    let config: OwnedConfig = ron::de::from_bytes(&contents[..])?;

    Ok(config.into_static())
}


