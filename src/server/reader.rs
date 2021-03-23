use serde::Deserialize;
use std::fs;
use std::fmt;
use std::convert::TryFrom;

use super::error::TaskmasterError;
use super::watcher::Watcher;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub task: Vec<ReadTask>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct ReadTask {
    pub name: String,
    pub cmd: String,
    pub autostart: bool,
    pub numprocess: i32,
    pub umask: i16,
    pub stopsignal: String,
    pub workingdir: String,
    pub stdout: String,
    pub stderr: String,
}

impl fmt::Display for ReadTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}\nCommand: {:?}\nNumber of processes: {}\nAutostart: {}\nUmask: {}\nWorking Directory: {:?}\nStdour: {:?}, Stderr: {:?}",
            self.name,
            self.cmd,
            self.numprocess,
            self.autostart,
            self.umask,
            self.workingdir,
            self.stdout,
            self.stderr
        )
    }
}

impl TryFrom<&Watcher> for ConfigFile {
    type Error = TaskmasterError;

    fn try_from(watcher: &Watcher) -> Result<Self, TaskmasterError> {
        let content: String = match fs::read_to_string(&watcher.path) {
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
