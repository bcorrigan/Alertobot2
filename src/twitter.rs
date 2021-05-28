#![allow(dead_code)]

use egg_mode;
use std;
use std::io::{Read, Write};


pub use yansi::Paint;

use crate::config::Config;

//Auth is all lifted from egg_mode examples, no point reinventing wheel 
pub struct Auth {
    pub token: egg_mode::Token,
    pub user_id: u64,
    pub screen_name: String,
}

impl Auth {
    pub async fn load(config:&Config) -> Self {
        let a1 = Auth::load_inner(config).await;
        if let Some(auth) = a1 {
            return auth;
        }

        Auth::load_inner(config).await.unwrap()
    }

    /// This needs to be a separate function so we can retry after creating the
    /// twitter_settings file. Idealy we would recurse, but that requires boxing
    /// the output which doesn't seem worthwhile
    async fn load_inner(config:&Config) -> Option<Self> {
        //IMPORTANT: make an app for yourself at apps.twitter.com and get your
        //key/secret into these files; these examples won't work without them
        let consumer_key = &config.twitter.key;
        let consumer_secret = &config.twitter.secret;

        let con_token = egg_mode::KeyPair::new(consumer_key.clone(), consumer_secret.clone());

        let mut authcfg = String::new();
        let user_id: u64;
        let username: String;
        let token: egg_mode::Token;

        //look at all this unwrapping! who told you it was my birthday?
        if let Ok(mut f) = std::fs::File::open("twitter_settings") {
            f.read_to_string(&mut authcfg).unwrap();

            let mut iter = authcfg.split('\n');

            username = iter.next().unwrap().to_string();
            user_id = u64::from_str_radix(&iter.next().unwrap(), 10).unwrap();
            let access_token = egg_mode::KeyPair::new(
                iter.next().unwrap().to_string(),
                iter.next().unwrap().to_string(),
            );
            token = egg_mode::Token::Access {
                consumer: con_token,
                access: access_token,
            };

            if let Err(err) = egg_mode::auth::verify_tokens(&token).await {
                println!("We've hit an error using your old tokens: {:?}", err);
                println!("We'll have to reauthenticate before continuing.");
                std::fs::remove_file("twitter_settings").unwrap();
            } else {
                println!("Welcome back, {}!\n", username);
            }
        } else {
            let request_token = egg_mode::auth::request_token(&con_token, "oob").await.unwrap();

            println!("Go to the following URL, sign in, and give me the PIN that comes back:");
            println!("{}", egg_mode::auth::authorize_url(&request_token));

            let mut pin = String::new();
            std::io::stdin().read_line(&mut pin).unwrap();
            println!("");

            let tok_result = egg_mode::auth::access_token(con_token, &request_token, pin)
                .await
                .unwrap();

            token = tok_result.0;
            user_id = tok_result.1;
            username = tok_result.2;

            match token {
                egg_mode::Token::Access {
                    access: ref access_token,
                    ..
                } => {
                    authcfg.push_str(&username);
                    authcfg.push('\n');
                    authcfg.push_str(&format!("{}", user_id));
                    authcfg.push('\n');
                    authcfg.push_str(&access_token.key);
                    authcfg.push('\n');
                    authcfg.push_str(&access_token.secret);
                }
                _ => unreachable!(),
            }

            let mut f = std::fs::File::create("twitter_settings").unwrap();
            f.write_all(authcfg.as_bytes()).unwrap();

            println!("Welcome, {}, let's get this show on the road!", username);
        }

        //TODO: Is there a better way to query whether a file exists?
        if std::fs::metadata("twitter_settings").is_ok() {
            Some(Auth {
                token: token,
                user_id: user_id,
                screen_name: username,
            })
        } else {
            None
        }
    }
}

//get full text if retweeted
pub fn get_text(tweet: &egg_mode::tweet::Tweet) -> String {
    if let Some(ref status) = tweet.retweeted_status {
        return format!("RT {}: {}", status.user.as_ref().unwrap().screen_name, get_text(status));
    } else {
        return tweet.text.clone();
    }
}

pub fn print_tweet(tweet: &egg_mode::tweet::Tweet) {
    if let Some(ref user) = tweet.user {
        println!(
            "{} (@{}) posted at {}",
            Paint::blue(&user.name),
            Paint::bold(Paint::blue(&user.screen_name)),
            tweet.created_at.with_timezone(&chrono::Local)
        );
    }

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        println!("➜ in reply to @{}", Paint::blue(screen_name));
    }

    if let Some(ref status) = tweet.retweeted_status {
        println!("{}", Paint::red("Retweet ➜"));
        print_tweet(status);
        return;
    } else {
        println!("{}", Paint::green(&tweet.text));
    }

    if let Some(source) = &tweet.source {
        println!("➜ via {} ({})", source.name, source.url);
    }

    if let Some(ref place) = tweet.place {
        println!("➜ from: {}", place.full_name);
    }

    if let Some(ref status) = tweet.quoted_status {
        println!("{}", Paint::red("➜ Quoting the following status:"));
        print_tweet(status);
    }

    if !tweet.entities.hashtags.is_empty() {
        println!("➜ Hashtags contained in the tweet:");
        for tag in &tweet.entities.hashtags {
            println!("  {}", tag.text);
        }
    }

    if !tweet.entities.symbols.is_empty() {
        println!("➜ Symbols contained in the tweet:");
        for tag in &tweet.entities.symbols {
            println!("  {}", tag.text);
        }
    }

    if !tweet.entities.urls.is_empty() {
        println!("➜ URLs contained in the tweet:");
        for url in &tweet.entities.urls {
            if let Some(expanded_url) = &url.expanded_url {
                println!("  {}", expanded_url);
            }
        }
    }

    if !tweet.entities.user_mentions.is_empty() {
        println!("➜ Users mentioned in the tweet:");
        for user in &tweet.entities.user_mentions {
            println!("  {}", Paint::bold(Paint::blue(&user.screen_name)));
        }
    }

    if let Some(ref media) = tweet.extended_entities {
        println!("➜ Media attached to the tweet:");
        for info in &media.media {
            println!("  A {:?}", info.media_type);
        }
    }
}
