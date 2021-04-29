use std::error;

#[derive(Debug)]
pub enum Taskmaster {
    ReadFile(std::io::Error),
    Io(std::io::Error),
    Parse(toml::de::Error),
    Signal,
    Cli,
}

impl Taskmaster {
    fn __description(&self) -> &str {
        match *self {
            Taskmaster::ReadFile(_) => "Unable to read file",
            Taskmaster::Io(_) => "IO failure",
            Taskmaster::Parse(_) => "Unable to parse config file",
            Taskmaster::Signal => "Signal not handled",
            Taskmaster::Cli => "Error in the cli",
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
            Taskmaster::Parse(ref e) => Some(e),
            Taskmaster::Signal | Taskmaster::Cli => None,
        }
    }
}

impl From<std::io::Error> for Taskmaster {
    fn from(err: std::io::Error) -> Taskmaster {
        Taskmaster::Io(err)
    }
}