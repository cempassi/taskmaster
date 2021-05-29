use clap::{App, Arg, ArgMatches, SubCommand};

pub fn generate() -> ArgMatches<'static> {
    App::new("Taskmaster")
        .version("0.1")
        .about("Unix process manager")
        .subcommand(
            SubCommand::with_name("server")
                .about("Launch server daemon")
                .args(&[Arg::with_name("config")
                    .short("c")
                    .help("use config file")
                    .long("config")
                    .value_name("FILE")
                    .takes_value(true)
                    .required(true)]),
        )
        .subcommand(SubCommand::with_name("client").about("Launch client"))
        .args(&[Arg::with_name("log-file")
            .default_value("/dev/stderr")
            .help("set output logging file")
            .long("log-file")
            .value_name("FILE")
            .takes_value(true)])
        .get_matches()
}
