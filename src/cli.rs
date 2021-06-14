use clap::{clap_app, ArgMatches};

fn file_exist(path: String) -> Result<(), String> {
    if std::fs::metadata(path).is_ok() {
        Ok(())
    } else {
        Err(String::from("file doesn't exist"))
    }
}

pub fn generate() -> ArgMatches<'static> {
    clap_app!(app =>
        (version: "0.1")
        (about: "Unix process manager")
        (@arg logfile: --("log-file") [FILE] +takes_value "set ouput logging file")
        (@subcommand server =>
            (about: "Launch server daemon")
            (@arg config: <FILE> +takes_value {file_exist} "config file to use")
            (@arg format: -f --format possible_value[human yaml json] default_value[human] "set the message format")
        )
        (@subcommand client =>
            (about: "Launch client")
        )
    )
    .get_matches()
}
