#![warn(clippy::all, clippy::pedantic)]
extern crate clap;
extern crate serde;

mod cli;
mod client;
mod server;
mod shared;

use log::{LevelFilter, SetLoggerError};
use server::error;
use shared::logger::{self, Config};
use std::time;

type Result<T> = std::result::Result<T, error::Taskmaster>;

/// # Errors
///
/// Will return `Err` when failing to initialise `LOGGER`
fn init() -> std::result::Result<(), SetLoggerError> {
    logger::simple::Logger::init(LevelFilter::Debug, Config::new(Some(time::Instant::now())))
}

fn main() -> Result<()> {
    let cli = cli::generate();
    init().unwrap();

    // LOGGER.

    if let Some(matches) = cli.subcommand_matches("server") {
        let config = matches.value_of("config").unwrap();
        if let Some(file) = matches.value_of("log-file") {};

        log::info!("starting server");
        server::start(config)?;
    } else if cli.subcommand_matches("client").is_some() {
        log::info!("starting client");
        client::start();
    } else {
        log::error!("unknown subcommand");
        return Err(error::Taskmaster::Cli);
    }
    Ok(())
}
