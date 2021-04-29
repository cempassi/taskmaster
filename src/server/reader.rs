use super::watcher::Watcher;
use super::{default, error, relaunch::Relaunch};
use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt;
use std::fs;

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
    pub workingdir: Option<String>,

    pub stopsignal: Option<String>,
    pub stopdelay: Option<u32>,

    pub stdout: Option<String>,
    pub stderr: Option<String>,

    pub retry: Option<u32>,

    pub successdelay: Option<u32>,

    pub exitcodes: Option<Vec<i32>>,

    pub restart: Option<Relaunch>,

    pub env: Option<Vec<String>>,
}

impl fmt::Display for ReadTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}\nCommand: {}\nNumber of processes: {}\nAutostart: {}\nUmask: {:#05o}\nWorking Directory: {}\nStdout: {}\nStderr: {}\nStop signal: {}\nStop delay: {}\nretry: {}\nSuccess Delay: {}\nExit Codes: {:?}\nRestart: {}\nEnv: {:?}",
            self.name,
            self.cmd,
            self.numprocess.unwrap_or(default::NUMPROCESS),
            self.autostart.unwrap_or(default::AUTOSTART),
            self.umask.unwrap_or(default::UMASK),
            self.workingdir.as_ref().unwrap_or(&String::from(default::WORKDIR)),

            self.stdout.as_ref().unwrap_or(&String::from(default::STDOUT)),
            self.stderr.as_ref().unwrap_or(&String::from(default::STDERR)),

            self.stopsignal.as_ref().unwrap_or(&String::from(default::STOP_SIGNAL)),
            self.stopdelay.unwrap_or(default::STOP_DELAY),

            self.retry.unwrap_or(default::RETRY),

            self.successdelay.unwrap_or(default::SUCCESS_DELAY),

            self.exitcodes.as_ref().unwrap_or(&Vec::from(default::EXPECTED_EXIT_CODES)),

            self.restart.as_ref().unwrap_or(&default::RELAUNCH_MODE),

            self.env.as_ref().unwrap_or(&Vec::from(default::ENV)),
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
        let ext = watcher.path.extension().and_then(std::ffi::OsStr::to_str);

        match ext {
            Some("yml") | Some("yaml") => match serde_yaml::from_str(&content) {
                Ok(c) => Ok(c),
                Err(e) => panic!("parsing yaml {}", e),
            },
            Some("toml") => match toml::from_str(&content) {
                Ok(c) => Ok(c),
                Err(e) => Err(error::Taskmaster::Parse(e)),
            },
            _ => Err(error::Taskmaster::Cli),
        }
    }
}
