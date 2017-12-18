#![recursion_limit = "1024"]

extern crate discord;
#[macro_use] extern crate error_chain;
extern crate ron;
#[macro_use] extern crate serde;

mod config;
mod errors { error_chain!{} }

use std::fs::File;
use std::path::Path;

use discord::Discord;
use discord::model::Event;

use config::Config;
pub use errors::*;


fn main() {
    if let Err(ref error) = run() {
        eprintln!("Botto-chan is an airhead! She fell over her own feet >.<");
        eprintln!("{}", error);

        if let Some(backtrace) = error.backtrace() {
            eprintln!("Backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = Config::new(Path::new("./config.ron"))?;
    let discord = Discord::from_bot_token(config.token)?;
    let (mut connection, _) = discord.connect()?;

    println!("Ready.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("{} says: {}", message.author.name, message.content);
                if message.content == "!test" {
                    let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
                } else if message.content == "!quit" {
                    println!("Quitting.");
                    break
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}
