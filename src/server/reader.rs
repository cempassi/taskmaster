use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt;
use std::fs;

use super::error;
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
    pub stopdelay: u32,
    pub workingdir: String,
    pub stdout: String,
    pub stderr: String,
}

impl fmt::Display for ReadTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}\nCommand: {:?}\nNumber of processes: {}\nAutostart: {}\nUmask: {}\nWorking Directory: {:?}\nStdour: {:?}, Stderr: {:?}, StopSignal: {}, StopDelay: {}",
            self.name,
            self.cmd,
            self.numprocess,
            self.autostart,
            self.umask,
            self.workingdir,
            self.stdout,
            self.stderr,
            self.stopsignal,
            self.stopdelay
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
