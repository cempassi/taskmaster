use std::error;

#[derive(Debug)]
pub enum Taskmaster {
    ReadFile(std::io::Error),
    Io(std::io::Error),
    ParseToml(toml::de::Error),
    ParseYaml(serde_yaml::Error),
    Signal,
    Cli,
    InvalidConf,
    InvalidCmd,
    ForkFailed
}

impl Taskmaster {
    fn __description(&self) -> &str {
        match *self {
            Taskmaster::ReadFile(_) => "Unable to read file",
            Taskmaster::Io(_) => "IO failure",
            Taskmaster::ParseToml(_) => "Unable to parse config file in TOML format",
            Taskmaster::ParseYaml(_) => "Unable to parse config file in YAML format",
            Taskmaster::Signal => "Signal not handled",
            Taskmaster::Cli => "Error in the cli",
            Taskmaster::InvalidConf => "Config file path is invald",
            Taskmaster::InvalidCmd => "Invalid Command",
            Taskmaster::ForkFailed => "Fork Failed"
        }
    }
}

impl std::fmt::Display for Taskmaster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.__description().fmt(f)
    }
}

impl error::Error for Taskmaster {
    fn description(&self) -> &str {
        self.__description()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Taskmaster::ReadFile(ref e) | Taskmaster::Io(ref e) => Some(e),
            Taskmaster::ParseToml(ref e) => Some(e),
            Taskmaster::ParseYaml(ref e) => Some(e),
            Taskmaster::Signal
            | Taskmaster::Cli
            | Taskmaster::ForkFailed
            | Taskmaster::InvalidConf
            | Taskmaster::InvalidCmd => None,
        }
    }
}

impl From<std::io::Error> for Taskmaster {
    fn from(err: std::io::Error) -> Taskmaster {
        Taskmaster::Io(err)
    }
}
