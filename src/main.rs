#![warn(clippy::all, clippy::pedantic)]
extern crate clap;

mod cli;
mod client;
mod error;
mod reader;
mod server;
mod signal;
mod state;
mod task;
mod watcher;
mod worker;
mod history;
mod editor;

use crate::error::TaskmasterError;
use cli::generate_cli;
use client::start_client;

type Result<T> = std::result::Result<T, TaskmasterError>;

fn main() -> Result<()> {
    let cli = generate_cli();

    if let Some(matches) = cli.subcommand_matches("server") {
        let _config = matches.value_of("config").unwrap();
        println!("Starting server");
    } else {
        println!("Starting client");
        start_client();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
