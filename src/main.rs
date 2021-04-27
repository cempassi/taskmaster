#![warn(clippy::all, clippy::pedantic)]
extern crate clap;
extern crate serde;

mod cli;
mod client;
mod server;

use server::error::TaskmasterError;

type Result<T> = std::result::Result<T, TaskmasterError>;

fn main() -> Result<()> {
    let cli = cli::generate();

    if let Some(matches) = cli.subcommand_matches("server") {
        let config = matches.value_of("config").unwrap();

        println!("Starting server");
        server::start(config);
    } else {
        client::start();
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
