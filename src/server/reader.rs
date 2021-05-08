use super::watcher::Watcher;
use super::{default, error, relaunch::Relaunch};
use libc::{gid_t, mode_t, uid_t};
use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs;

pub type ConfigFile = BTreeMap<String, ReadTask>;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct ReadTask {
    pub cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub numprocess: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub umask: Option<mode_t>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workingdir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopsignal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopdelay: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub successdelay: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exitcodes: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<Relaunch>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<gid_t>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<uid_t>,
}

impl fmt::Display for ReadTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Command: {}\nNumber of processes: {}\nAutostart: {}\nUmask: {:#05o}\nWorking Directory: {}\nStdout: {}\nStderr: {}\nStop signal: {}\nStop delay: {}\nretry: {}\nSuccess Delay: {}\nExit Codes: {:?}\nRestart: {}\nEnv: {:?}\nPermission: uid: {:?}, gid: {:?}",
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

            self.uid,
            self.gid,
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
            Some("yml") | Some("yaml") => {
                log::info!("try parsing in YAML format");
                match serde_yaml::from_str(&content) {
                    Ok(c) => Ok(c),
                    Err(e) => Err(error::Taskmaster::ParseYaml(e)),
                }
            }
            Some("toml") => {
                log::info!("try parsing in TOML format");
                match toml::from_str(&content) {
                    Ok(c) => Ok(c),
                    Err(e) => Err(error::Taskmaster::ParseToml(e)),
                }
            }
            _ => {
                log::error!("not handler for ext {:?}", ext);
                Err(error::Taskmaster::Cli)
            }
        }
    }
}
