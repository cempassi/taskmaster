#![warn(clippy::all, clippy::pedantic)]
extern crate clap;
extern crate serde;

mod cli;
mod client;
mod server;
mod shared;

use log::{LevelFilter, SetLoggerError};
use nix::unistd::{close, execve, fork, setsid, ForkResult};
use shared::{
    error,
    logger::{self, Config},
};
use std::{env, ffi::CString, fs::File, time};

type TaskmasterResult<T> = Result<T, error::Taskmaster>;

fn init(cli: &clap::ArgMatches<'static>) -> Result<(), SetLoggerError> {
    let config = Config::new(Some(time::Instant::now()));

    cli.value_of("logfile").map_or_else(
        || logger::simple::Logger::init(LevelFilter::Debug, config),
        |file| logger::file::Logger::init(LevelFilter::Debug, config, File::create(file).unwrap()),
    )
}

fn detach(path: String, config: &str, logfile: Option<&str>) -> TaskmasterResult<()> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Child is {}", child);
            Ok(())
        }
        Ok(ForkResult::Child) => {
            println!("I'm in Child Process");
            let mut args = vec![
                CString::new("taskmaster").unwrap(),
                CString::new("--log-file").unwrap(),
            ];
            if let Some(file) = logfile {
                args.push(CString::new(file).unwrap());
            } else {
                args.push(CString::new("/tmp/taskmaster.log").unwrap());
            }
            args.push(CString::new("server").unwrap());
            args.push(CString::new(config).unwrap());
            close(0).unwrap();
            close(1).unwrap();
            //close(2).unwrap();
            setsid().unwrap();
            execve(
                CString::new(path).unwrap().as_c_str(),
                args.as_slice(),
                &[CString::new("").unwrap()],
            )
            .expect("Failed to launch process");
            Ok(())
        }
        Err(_) => Err(error::Taskmaster::ForkFailed),
    }
}

fn main() -> TaskmasterResult<()> {
    let cli = cli::generate();
    let mut args = env::args();
    init(&cli).unwrap();

    match cli.subcommand() {
        ("server", Some(matches)) => {
            let config = matches.value_of("config").unwrap();
            let format = matches.value_of("format").unwrap();
            if matches.is_present("detached") {
                log::info!("detached");
                let logfile = cli.value_of("logfile");
                detach(args.next().unwrap(), config, logfile)
            } else {
                server::start(config, format)
            }
        }
        ("client", Some(_matches)) => {
            log::info!("starting client");
            client::start()
        }
        _ => {
            log::error!("unknown subcommand");
            Err(error::Taskmaster::Cli)
        }
    }
}
