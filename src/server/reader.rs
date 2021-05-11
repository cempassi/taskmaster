use super::watcher::Watcher;
use super::{default, error, relaunch::Relaunch, signal::Signal};
use libc::{gid_t, mode_t, uid_t};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs::{self, File};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command};

pub type ConfigFile = BTreeMap<String, ReadTask>;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct ReadTask {
    pub cmd: String,

    #[serde(skip)]
    args: Vec<String>,

    #[serde(default = "default::autostart")]
    pub autostart: bool,

    #[serde(default = "default::numprocess")]
    pub numprocess: u32,

    #[serde(default = "default::umask")]
    pub umask: mode_t,

    #[serde(default = "default::workdir")]
    pub workingdir: PathBuf,

    #[serde(default = "default::stop_signal")]
    pub stopsignal: Signal,

    #[serde(default = "default::stop_delay")]
    pub stopdelay: u32,

    #[serde(default = "default::stdout")]
    pub stdout: PathBuf,

    #[serde(default = "default::stderr")]
    pub stderr: PathBuf,

    #[serde(default = "default::retry")]
    pub retry: u32,

    #[serde(default = "default::success_delay")]
    pub successdelay: u32,

    #[serde(default = "default::exit_codes")]
    pub exitcodes: Vec<i32>,

    #[serde(default = "default::relaunch_mode")]
    pub restart: Relaunch,

    #[serde(default = "default::env")]
    pub env: Vec<String>,

    pub gid: Option<gid_t>,
    pub uid: Option<uid_t>,
}

impl fmt::Display for ReadTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Command: {}\nNumber of processes: {}\nAutostart: {}\nUmask: {:#05o}\nWorking Directory: {:?}\nStdout: {:?}\nStderr: {:?}\nStop signal: {}\nStop delay: {}\nretry: {}\nSuccess Delay: {}\nExit Codes: {:?}\nRestart: {}\nEnv: {:?}\nPermission: uid: {:?}, gid: {:?}",
            self.cmd,
            self.numprocess,
            self.autostart,
            self.umask,
            self.workingdir,

            self.stdout,
            self.stderr,

            self.stopsignal,
            self.stopdelay,

            self.retry,

            self.successdelay,

            self.exitcodes,

            self.restart,

            self.env,

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

impl ReadTask {
    pub fn run(&self) -> Vec<Child> {
        let mut jobs = Vec::new();
        let mut command = Command::new(&self.args[0]);
        let stdout = File::create(self.stdout.as_path()).unwrap();
        let stderr = File::create(self.stderr.as_path()).unwrap();
        self.setup_command(&mut command);
        if self.args.len() > 1 {
            command.args(&self.args[1..]);
        }
        command.current_dir(self.workingdir.as_path());
        command.stdout(stdout);
        command.stderr(stderr);
        for _ in 0..self.numprocess {
            jobs.push(command.spawn().expect("Couldn't run command!"));
        }
        jobs
    }

    fn update_args(&mut self) -> Vec<String> {
        self.args = self
            .cmd
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect();
        self.args
    }

    fn setup_command(&self, command: &mut impl CommandExt) {
        if let Some(uid) = self.uid {
            command.uid(uid);
        }
        if let Some(gid) = self.gid {
            command.gid(gid);
        }
        if self.umask != 0 {
            let umask: mode_t = self.umask;
            unsafe {
                command.pre_exec(move || {
                    libc::umask(umask);
                    Ok(())
                });
            }
        }
    }

    pub fn check_exit_status(&self, status: std::process::ExitStatus) -> bool {
        status.code().map_or(false, |exitcode| {
            self.exitcodes.iter().any(|&code| code != exitcode)
        })
    }
}
