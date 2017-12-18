#![recursion_limit = "1024"]

extern crate discord;
#[macro_use] extern crate error_chain;
extern crate ron;
#[macro_use] extern crate serde;
#[macro_use] extern crate slog;

mod command;
mod config;
mod errors { error_chain!{} }

use std::fs::OpenOptions;
use std::path::Path;

use discord::Discord;
use discord::model::Event;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, PlainDecorator};

use command::Command;
use config::Config;
pub use errors::*;


fn main() {
    let log_file_path = "./botto.log";
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(log_path)
        .expect(format!("Could not open log file: {}", log_file_path))
    ;

    let logger = {
        let decorator = PlainDecorator::new(log_file);
        let drain = FullFormat::new(decorator).build().fuse();
        let drain = Async::new(drain).build().fuse();

        Logger::root(drain, o!())
    };

    if let Err(ref error) = run(&logger) {
        crit!(logger, "Botto-chan is an airhead! She fell over her own feet >.<");
        crit!(logger, "{}", error);

        if let Some(backtrace) = error.backtrace() {
            crit!(logger, "Backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run(logger: &Logger) -> Result<()> {
    let config = Config::new(Path::new("./config.ron"))?;
    let discord = Discord::from_bot_token(config.token)?;
    let (mut connection, _) = discord.connect()?;

    println!("Ready.");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                if Err(e) = Command::interpret(message) {
                    break Err(e)
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => break Err(format!("Gateway closed on us with code {:?}: {}", code, body))
            Err(err) => error!(logger, "Received error: {:?}", err)
        }
    }
}
