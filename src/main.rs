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
use std::{fs::File, time};

type Result<T> = std::result::Result<T, error::Taskmaster>;

fn init(cli: &clap::ArgMatches<'static>) -> std::result::Result<(), SetLoggerError> {
    let config = Config::new(Some(time::Instant::now()));

    cli.value_of("log-file").map_or_else(
        || logger::simple::Logger::init(LevelFilter::Debug, config),
        |file| logger::file::Logger::init(LevelFilter::Debug, config, File::create(file).unwrap()),
    )
}

fn main() -> Result<()> {
    let cli = cli::generate();
    init(&cli).unwrap();

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
