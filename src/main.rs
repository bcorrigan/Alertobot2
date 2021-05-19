#![allow(unused_imports)]
extern crate clap;
extern crate tbot;
extern crate egg_mode;

mod config;

use clap::{App, Arg};
use clap::value_t;
use tbot::prelude::*;
use tbot::Bot;
use std::io;

#[tokio::main]
async fn main() {
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
        .arg(Arg::with_name("twitter-auth")
            .long("twitter-auth")
            .required(false)
            .takes_value(false)
            .help("Perform authentication with twitter to get token"))
        .get_matches();

    let cfg_file = Box::leak(value_t!(matches, "config", String).unwrap_or_else(|_| "./config.ron".to_string()).into_boxed_str() );

    let config = config::parse(cfg_file).unwrap_or_else(|error| {
        eprintln!("Couldn't parse the config: {:#?}", error);
        std::process::exit(1);
    });

    //TODO reimplement to follow this process: https://github.com/egg-mode-rs/egg-mode/blob/master/examples/common/mod.rs
    if matches.is_present("twitter-auth") {
        async {
            let con_token = egg_mode::KeyPair::new(config.twitter.key, config.twitter.secret);
            let request_token = egg_mode::auth::request_token(&con_token, "oob").await.unwrap();
            let auth_url = egg_mode::auth::authorize_url(&request_token);

            println!("Please visit this auth url: {}", auth_url);
            println!("input pin: ");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            let pin = user_input.trim();

            let (token, user_id, screen_name) = egg_mode::auth::access_token(con_token, &request_token, verifier).await.unwrap();

            println!("Token {} obtained for user {}", token, user_id);

        }.await;
    }



    let bot = Bot::new(config.telegram.bot_token.to_string()).event_loop();

    bot.polling().start().await.unwrap();

}


