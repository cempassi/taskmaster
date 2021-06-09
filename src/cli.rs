use clap::{clap_app, ArgMatches};

pub fn generate() -> ArgMatches<'static> {
    clap_app!(app =>
        (version: "0.1")
        (about: "Unix process manager")
        (@arg logfile: --("log-file") [FILE] +takes_value "set ouput logging file")
        (@subcommand server =>
            (about: "Launch server daemon")
            (@arg config: <FILE> +takes_value "config file to use")
        )
        (@subcommand client =>
            (about: "Launch client")
        )
    )
    .get_matches()
}
