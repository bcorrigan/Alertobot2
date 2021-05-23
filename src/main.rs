#![allow(unused_imports)]
extern crate clap;
extern crate tbot;
extern crate egg_mode;
extern crate yansi;
extern crate tokio_stream;

mod config;
mod twitter;

use clap::{App, Arg};
use clap::value_t;
use tbot::prelude::*;
use tbot::Bot;
use std::io;

use egg_mode::user::TwitterUser;
use egg_mode::error::Result;
use egg_mode::cursor::CursorIter;

use tokio_stream::StreamExt;


use crate::twitter::Auth;

//parse cfg/rules
//log onto twitter
//create handler for twitter events
//create handler for telegram events
//the end!

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
        .get_matches();

    let cfg_file = Box::leak(value_t!(matches, "config", String).unwrap_or_else(|_| "./config.ron".to_string()).into_boxed_str() );

    let config = config::parse(cfg_file).unwrap_or_else(|error| {
        eprintln!("Couldn't parse the config: {:#?}", error);
        std::process::exit(1);
    });

    //twitter log in
    let twauth = Auth::load(&config).await;

    let t:Result<Vec<TwitterUser>> = egg_mode::user::friends_ids(twauth.user_id, &twauth.token)
            .take(10)
            .map_ok(|r| r.response)
            .try_collect::<Vec<_>>()
            .await;



    let stream = egg_mode::stream::filter()
        //.follow()
        .language(&["en"])
        .start(&twauth.token);
        /*.try_for_each(|m| {
            if let StreamMessage::Tweet(tweet) = m {
                common::print_tweet(&tweet);
                println!("──────────────────────────────────────");
            } else {
                println!("{:?}", m);
            }
            futures::future::ok(())
        });*/


    //set up following
    //https://github.com/egg-mode-rs/egg-mode/blob/master/examples/stream_filter.rs



    let bot = Bot::new(config.telegram.bot_token.to_string()).event_loop();

    bot.polling().start().await.unwrap();

}


