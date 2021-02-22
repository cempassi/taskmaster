use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use crate::error::TaskmasterError;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub task: Vec<ReadTask>,
}

#[derive(Debug, Deserialize)]
pub struct ReadTask {
    pub name: String,
    pub cmd: String,
    pub numprocess: i32,
    pub umask: i16,
    pub stopsignal: String,
    pub workingdir: String,
    pub stdout: String,
    pub stderr: String,
}

impl FromStr for ConfigFile {
    type Err = TaskmasterError;

    fn from_str(path: &str) -> Result<Self, TaskmasterError> {
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
