use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt;
use std::fs;

use super::watcher::Watcher;
use super::{default, error};

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub task: Vec<ReadTask>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct ReadTask {
    pub name: String,
    pub cmd: String,
    pub autostart: Option<bool>,
    pub numprocess: Option<u32>,
    pub umask: Option<u16>,
    pub stopsignal: Option<String>,
    pub workingdir: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl fmt::Display for ReadTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}\nCommand: {}\nNumber of processes: {}\nAutostart: {}\nUmask: {}\nWorking Directory: {}\nStdour: {}, Stderr: {}",
            self.name,
            self.cmd,
            self.numprocess.unwrap_or(default::NUMPROCESS),
            self.autostart.unwrap_or(default::AUTOSTART),
            self.umask.unwrap_or(default::UMASK),
            self.workingdir.as_ref().unwrap_or(&String::from(default::WORKDIR)),
            self.stdout.as_ref().unwrap_or(&String::from(default::STDOUT)),
            self.stderr.as_ref().unwrap_or(&String::from(default::STDERR))
        )
    }
}

impl TryFrom<&Watcher> for ConfigFile {
    type Error = error::Taskmaster;

    fn try_from(watcher: &Watcher) -> Result<Self, error::Taskmaster> {
        let content: String = match fs::read_to_string(&watcher.path) {
            Ok(c) => c,
            Err(e) => return Err(error::Taskmaster::ReadFile(e)),
        };
        match toml::from_str(&content) {
            Ok(c) => Ok(c),
            Err(e) => Err(error::Taskmaster::Parse(e)),
        }
    }
}
