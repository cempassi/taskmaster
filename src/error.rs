use std::error;

#[derive(Debug)]
pub enum TaskmasterError {
    FileNotFound(std::io::Error),
    ReadFile(std::io::Error),
    Io(std::io::Error),
    Parse(toml::de::Error),
    Signal,
}

impl TaskmasterError {
    fn __description(&self) -> &str {
        match *self {
            TaskmasterError::FileNotFound(_) => "Unable to find file",
            TaskmasterError::ReadFile(_) => "Unable to read file",
            TaskmasterError::Io(_) => "IO failure",
            TaskmasterError::Parse(_) => "Unable to parse config file",
            TaskmasterError::Signal => "Signal not handled",
        }
    }
}

impl std::fmt::Display for TaskmasterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.__description().fmt(f)
    }
}

impl error::Error for TaskmasterError {
    fn description(&self) -> &str {
        self.__description()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            TaskmasterError::FileNotFound(ref e) => Some(e),
            TaskmasterError::ReadFile(ref e) => Some(e),
            TaskmasterError::Io(ref e) => Some(e),
            TaskmasterError::Parse(ref e) => Some(e),
            TaskmasterError::Signal => None,
        }
    }
}

impl From<std::io::Error> for TaskmasterError {
    fn from(err: std::io::Error) -> TaskmasterError{
        TaskmasterError::Io(err)
    }
}
