use super::{default, error, nix_utils, relaunch::Relaunch, watcher::Watcher};
use nix::{
    sys::{
        signal::Signal,
        stat::{self, Mode},
    },
    unistd::{Gid, Uid},
};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs::{self, File};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command};

pub type ConfigFile = BTreeMap<String, Task>;

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

#[derive(Deserialize, Serialize)]
struct TaskPartial {
    pub cmd: String,

    #[serde(default = "default::autostart")]
    pub autostart: bool,

    #[serde(default = "default::numprocess")]
    pub numprocess: u32,

    #[serde(default = "default::umask", with = "nix_utils::SerdeMode")]
    pub umask: Mode,

    #[serde(default = "default::workdir")]
    pub workingdir: PathBuf,

    #[serde(with = "nix_utils::SerdeSignal", default = "default::stop_signal")]
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

    #[serde(with = "nix_utils::SerdeOptionnalUidGid", default)]
    pub uid: Option<Uid>,
    #[serde(with = "nix_utils::SerdeOptionnalUidGid", default)]
    pub gid: Option<Gid>,
}

impl From<Task> for TaskPartial {
    fn from(task: Task) -> TaskPartial {
        TaskPartial {
            cmd: task.cmd,
            autostart: task.autostart,
            numprocess: task.numprocess,
            umask: task.umask,
            workingdir: task.workingdir,
            stopsignal: task.stopsignal,
            stopdelay: task.stopdelay,
            stdout: task.stdout,
            stderr: task.stderr,
            retry: task.retry,
            successdelay: task.successdelay,
            exitcodes: task.exitcodes,
            restart: task.restart,
            env: task.env,
            gid: task.gid,
            uid: task.uid,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    cmd: String,
    args: Vec<String>,
    pub autostart: bool,
    pub numprocess: u32,
    umask: Mode,
    workingdir: PathBuf,
    stopsignal: Signal,
    stopdelay: u32,
    stdout: PathBuf,
    stderr: PathBuf,
    retry: u32,
    successdelay: u32,
    pub exitcodes: Vec<i32>,
    restart: Relaunch,
    env: Vec<String>,
    uid: Option<Uid>,
    gid: Option<Gid>,
}

impl<'de> Deserialize<'de> for Task {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let partial = TaskPartial::deserialize(deserializer)?;
        Ok(Task::from(partial))
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let partial: TaskPartial = self.clone().into();
        partial.serialize(serializer)
    }
}

impl From<TaskPartial> for Task {
    fn from(partial: TaskPartial) -> Self {
        Self {
            cmd: partial.cmd.clone(),
            args: partial
                .cmd
                .split(' ')
                .map(std::string::ToString::to_string)
                .collect(),
            autostart: partial.autostart,
            numprocess: partial.numprocess,
            umask: partial.umask,
            workingdir: partial.workingdir,
            stopsignal: partial.stopsignal,
            stopdelay: partial.stopdelay,
            stdout: partial.stdout,
            stderr: partial.stderr,
            retry: partial.retry,
            successdelay: partial.successdelay,
            exitcodes: partial.exitcodes,
            restart: partial.restart,
            env: partial.env,
            gid: partial.gid,
            uid: partial.uid,
        }
    }
}

impl fmt::Display for Task {
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

impl Task {
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

    fn setup_command(&self, command: &mut impl CommandExt) {
        if let Some(uid) = self.uid {
            command.uid(uid.as_raw());
        }
        if let Some(gid) = self.gid {
            command.gid(gid.as_raw());
        }
        if self.umask != default::umask() {
            let umask: Mode = self.umask;
            unsafe {
                command.pre_exec(move || {
                    stat::umask(umask);
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
