use serde::Deserialize;
use std::fs;
use std::str::FromStr;
use std::path::Path;
use std::io::Error;

use crate::error::TaskmasterError;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub config: ReadConfig,
    pub tasks: Vec<ReadTask>,
}

#[derive(Debug, Deserialize)]
pub struct ReadConfig {
    pub numprocess: i32,
    pub umask: i32,
    pub stopsignal: String,
    pub workingdir: String,
}

#[derive(Debug, Deserialize)]
pub struct ReadTask {
    pub name: String,
    pub cmd: String,
    pub numprocess: i32,
    pub umask: i16,
    pub workingdir: String,
    pub stdout: String,
    pub stderr: String,
}

impl FromStr for ConfigFile {
    type Err = TaskmasterError;

    fn from_str(path: &str) -> Result<Self, TaskmasterError> {
        if !Path::new(path).exists() {
            return Err(TaskmasterError::FileNotFound(Error::last_os_error()));
        };

        let content: String = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(TaskmasterError::ReadFile(e)),
        };

        let parsed = match toml::from_str(&content) {
            Ok(c) => Ok(c),
            Err(e) => Err(TaskmasterError::Parse(e)),
        };
        parsed
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_from_str_ok () -> Result <(), TaskmasterError> {
        let path = "./config.toml";
        let configfile: ConfigFile = ConfigFile::from_str(path)?;
        assert!(!configfile.config.workingdir.is_empty(), "configFile empty so panic");
        Ok(())
    }

}