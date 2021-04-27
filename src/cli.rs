use clap::{App, Arg, ArgMatches, SubCommand};

pub fn generate() -> ArgMatches<'static> {
    App::new("Taskmaster")
        .version("0.1")
        .about("Unix process manager")
        .subcommand(
            SubCommand::with_name("server")
                .about("Launch server daemon")
                .arg(
                    Arg::with_name("config")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("client").about("Launch client"))
        .get_matches()
}
