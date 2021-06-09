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

type TaskmasterResult<T> = Result<T, error::Taskmaster>;

fn init(cli: &clap::ArgMatches<'static>) -> Result<(), SetLoggerError> {
    let config = Config::new(Some(time::Instant::now()));

    cli.value_of("logfile").map_or_else(
        || logger::simple::Logger::init(LevelFilter::Debug, config),
        |file| logger::file::Logger::init(LevelFilter::Debug, config, File::create(file).unwrap()),
    )
}

fn main() -> TaskmasterResult<()> {
    let cli = cli::generate();
    init(&cli).unwrap();

    match cli.subcommand() {
        ("server", Some(matches)) => {
            let config = matches.value_of("config").unwrap();
            let format = matches.value_of("format").unwrap();
            server::start(config, format)
        }
        ("client", Some(_matches)) => {
            log::info!("starting client");
            client::start();
            Ok(())
        }
        _ => {
            log::error!("unknown subcommand");
            Err(error::Taskmaster::Cli)
        }
    }
}
