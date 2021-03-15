#![warn(clippy::all, clippy::pedantic)]
//extern crate clap;
//extern crate serde;
//extern crate rmp_serde as rmps;

mod cli;
mod client;
mod server;

use cli::generate_cli;
use client::start_client;
use server::start_server;
use server::error::TaskmasterError;


type Result<T> = std::result::Result<T, TaskmasterError>;

fn main() -> Result<()> {
    let cli = generate_cli();

    if let Some(matches) = cli.subcommand_matches("server") {
        let config = matches.value_of("config").unwrap();

        println!("Starting server");
        start_server(config);
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
