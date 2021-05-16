use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct TelegramConfig {
    pub bot_token: &'static str,
    pub chat: i64,
}

#[derive(Deserialize)]
pub struct OwnedTelegramConfig {
    bot_token: String,
    chat: i64,
}

impl OwnedTelegramConfig {
    pub fn into_static(self) -> TelegramConfig {
        TelegramConfig {
            bot_token: Box::leak(Box::new(self.bot_token)),
            chat: self.chat,
        }
    }
}
