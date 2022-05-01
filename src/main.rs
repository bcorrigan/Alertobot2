#![allow(unused_imports)]
extern crate clap;
extern crate tbot;
extern crate egg_mode;
extern crate yansi;
extern crate tokio_stream;
extern crate futures;
extern crate regex;
extern crate serde_regex;
extern crate sd_notify;

mod config;
mod twitter;
mod rule;
mod test;

use clap::{App, Arg};
use clap::value_t;
use regex::Regex;
use tbot::prelude::*;
use tbot::Bot;
use tbot::types::parameters::Text;
use tbot::methods::SendMessage;
use tbot::types::chat::Id;
//use tbot::types::chat::Chat;
use tbot::types::chat;
use tbot::types::input_file::{Photo,MediaGroup, PhotoOrVideo};
use std::io;
use std::time::Duration;
use crate::rule::TweetInfo;

use chrono::{Local, Timelike, Datelike};
use egg_mode::{stream::StreamMessage, user::TwitterUser};
//use egg_mode::error::Result;
use egg_mode::cursor::CursorIter;

use tokio_stream::StreamExt;
use tokio::time::{Timeout, timeout};
use futures::stream::TryStreamExt;
use futures::executor::block_on;

use std::{thread, time};

use sd_notify::{notify, NotifyState};


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
    let mut twauth = Auth::load(&config).await;

    let t:Vec<u64> = egg_mode::user::friends_ids(twauth.user_id, &twauth.token)
            .take(100)
            .map_ok(|r| r.response)
            .try_collect::<Vec<_>>()
            .await.unwrap();

    println!("Following {} accounts", t.len());

    let tbot = Bot::new(config.telegram.bot_token.to_string());
    let rules = &config.rules;
    let duration = Duration::new(900, 0);

    loop {
        let fut = egg_mode::stream::filter()
            .follow(&t)
            .language(&["en"])
            .start(&twauth.token)
            //.timeout(duration)
            .take_while(|m| m.is_ok())
            .try_for_each(|m| {
                /*let sm = match m {
                    Ok(sm) => sm,
                    Err(e) => {
                        println!("Timeout detected");
                        return futures::future::err(e);
                    },
                }; */

                if let StreamMessage::Tweet(tweet) = m {
                    twitter::print_tweet(&tweet);
                    for rule in rules {
                        //we construct this because mocking it is a complete pain
                        let tweetinfo = TweetInfo {
                            text: twitter::get_text(&tweet),
                            hour: Local::now().hour(),
                            day: Local::now().date().weekday().to_string(),
                            retweeted: tweet.retweeted.unwrap_or(false),
                            user: tweet.user.as_ref().unwrap().id,
                            rtuser : twitter::get_root_user(&tweet),
                            screen_name: &tweet.user.as_ref().unwrap().screen_name,
                            followed_users: &t,
                        };

                        if rule.matches(&tweetinfo) { 
                            //need to refetch the tweet here as it doesn't seem to have media entities populated when got from stream
                            if let Ok(fulltweet) = block_on(egg_mode::tweet::show(tweet.id, &twauth.token)) {
                                let has_media = fulltweet.entities.media.is_some();
                                let webpage_preview = rule.webpage_preview && !has_media;
                                /*
                                |  wp   |  !hm  |outcome|
                                | false | true  | false |
                                | false | false | false |
                                | true  | false | false |
                                | true  | true  | true  |
                                */
                                for chat in &rule.chats {
                                    //TODO I suppose should try not blocking here...
                                    println!("RULE: Sending body to {}", chat.chat);
                                    let _ = block_on(tbot.send_message(Id(chat.chat), Text::with_html(format!("<b>{}</b>: {}" , tweet.user.as_ref().unwrap().screen_name, twitter::get_text(&tweet)))).is_web_page_preview_disabled(!webpage_preview)
                                                                    .call()).map_err(|e| format!("There was a telegram error: {}", e));
                                }
                                if rule.include_images {
                                    thread::sleep(time::Duration::from_millis(1000));
                                    if let Some(entities) = &fulltweet.extended_entities {
                                        let mut photos = Vec::new();
                                        for entity in &entities.media {
                                            //send media
                                            let photo = PhotoOrVideo::Photo( Photo::with_url(entity.media_url_https.clone()) );
                                            photos.push(photo);
                                            //TODO videos and documents
                                            println!("RULE: Appending media: {}", entity.media_url_https);
                                        } 
                                        let media_group = MediaGroup::PhotosAndVideos(photos);
                                        for chat in &rule.chats {
                                            let _ = block_on(tbot.send_media_group(Id(chat.chat), media_group.clone()).is_notification_disabled(true).call()).map_err(|e| format!("There was a telegram error: {}", e));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    println!("──────────────────────────────────────");
                    //TODO check rules etc here and print to telegram
                } else {
                    println!("Unknown object:{:?}", m);
                }
                //notify systemd watchdog that we were active
                let _ = notify(true, &[NotifyState::Ready]);
                futures::future::ok(())
            }); //.await.map_err(|e| format!("There was a tweeter error: {}", e));

            if let Err(e) = timeout(duration, fut).await {
                println!("Timed out, restarting..")
            }

            thread::sleep(time::Duration::from_millis(10000));
            twauth = Auth::load(&config).await;
            println!("Restarting tweet stream..");
    }
}

#[test]
fn regexx() {
    let rgx = Regex::new("(?i)a76[\\D$]|irvine|kilmarnock|a77[\\D$]|m77[\\D$]|bellfield|galston").unwrap();
    let teststr = "UPDATE❗️⌚️07:50

    #M8 roadworks junction 15 to junction 18🚧
    
    Slow westbound from J13
    Eastbound from J22
    
    #M77 slow northbound J2 
    #M74 slow northbound J3A to J1A 
    
    @SWTrunkRoads
     @GlasgowCC";

    println!("Match? {}", rgx.is_match(&teststr));
    //assert!(false);
}