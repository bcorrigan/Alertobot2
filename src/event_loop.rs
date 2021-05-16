use std::future::Future;

use crate::{
    config::TelegramConfig,
};

use tbot::{methods::SendMessage, types::parameters::Text};

const MAX_MESSAGE_LENGTH: usize = 4096;
const CROPPED_MESSAGE_LENGTH: usize = 3900;

fn check_twitter( telegram: TelegramConfig) -> impl Future<Item = (), Error = ()> {
    let message = "Woohoo this is a test.";

    SendMessage::new(telegram.bot_token, telegram.chat, Text::with_markdown(&message))
        .into_future()
        .map(|_| ())
        .map_err(|error| {
            dbg!(error);
        })
}

pub fn start(telegram: TelegramConfig) -> ! {
    tokio::spawn(check_twitter(telegram));
    std::thread::sleep(1000);
}
