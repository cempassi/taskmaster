#![warn(clippy::all, clippy::pedantic)]
extern crate clap;
extern crate serde;

mod cli;
mod client;
mod server;
mod shared;

use log::{LevelFilter, SetLoggerError};
use server::error;
use shared::logger::simple::LOGGER;

type Result<T> = std::result::Result<T, error::Taskmaster>;

/// # Errors
///
/// Will return `Err` when failing to initialise `LOGGER`
unsafe fn init() -> std::result::Result<(), SetLoggerError> {
    LOGGER.set_instant(std::time::Instant::now());
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

fn main() -> Result<()> {
    unsafe {
        init().unwrap();
    }
    let cli = cli::generate();

    // LOGGER.

    if let Some(matches) = cli.subcommand_matches("server") {
        let config = matches.value_of("config").unwrap();

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
