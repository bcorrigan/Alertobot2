#![allow(unused_imports)]
extern crate clap;
extern crate tbot;

use clap::{App, Arg};
use tbot::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Twat ")
        .version("0.1")
        .author("Barry Corrigan <b.j.corrigan@gmail.com>")
        .about("Twitter and Telegram bot for notification use")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("The config and rules file which governs which accounts twat uses and what rules it has for following and alerting")
            .required(true)
            .takes_value(true))
        .get_matches();

    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    Ok(())
}


