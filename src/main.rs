#![warn(clippy::all, clippy::pedantic)]
extern crate clap;
extern crate serde;

mod cli;
mod client;
mod server;
mod shared;

use log::{Level, LevelFilter, SetLoggerError};
use server::error;
use std::str::FromStr;

type Result<T> = std::result::Result<T, error::Taskmaster>;

static LOGGER: shared::logger::Simple = shared::logger::Simple {
    level: Level::Debug,
};

/// # Errors
///
/// Will return `Err` when failing to initialise `LOGGER`
/// # Panics
///
/// Will panic if `level` is bad formated
pub fn init(level: &str) -> std::result::Result<(), SetLoggerError> {
    let converted_level = LevelFilter::from_str(level).unwrap();
    let res = log::set_logger(&LOGGER).map(|()| log::set_max_level(converted_level));
    log::info!("log level set to {}", level);

    res
}

fn main() -> Result<()> {
    let cli = cli::generate();
    let level = cli.value_of("verbose").unwrap();
    init(level).unwrap();

    if let Some(matches) = cli.subcommand_matches("server") {
        let config = matches.value_of("config").unwrap();

        log::info!("starting server");
        server::start(config);
    } else if cli.subcommand_matches("client").is_some() {
        log::info!("starting client");
        client::start();
    } else {
        log::error!("unknown subcommand");
        return Err(error::Taskmaster::Cli);
    }
    Ok(())
}
