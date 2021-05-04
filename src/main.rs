#![warn(clippy::all, clippy::pedantic)]
extern crate clap;
extern crate serde;

mod cli;
mod client;
mod server;
mod shared;

use log::{Level, LevelFilter, SetLoggerError};
use server::error;

type Result<T> = std::result::Result<T, error::Taskmaster>;

static LOGGER: shared::logger::Simple = shared::logger::Simple {
    level: Level::Debug,
};

/// # Errors
///
/// Will return `Err` when failing to initialise `LOGGER`
pub fn init() -> std::result::Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

fn main() -> Result<()> {
    init().unwrap();
    let cli = cli::generate();

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
