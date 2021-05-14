#![allow(unused_imports)]
extern crate clap;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}


