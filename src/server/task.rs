use serde::Deserialize;
use std::convert::TryFrom;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::str::FromStr;
use std::vec::Vec;

use super::{default, error, reader::ReadTask, signal};

#[derive(Debug, Deserialize)]
enum AutoRestart {
    Unexpected,
    True,
    False,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    cmd: Vec<String>,
    numprocess: u32,
    autostart: bool,
    umask: u16,
    workingdir: PathBuf,

    stdout: PathBuf,
    stderr: PathBuf,

    stopsignal: signal::Signal,
    stopdelay: u32,

    retry: u32,

    successdelay: u32,

    expected_exit_codes: Vec<i32>,
}

impl TryFrom<&ReadTask> for Task {
    type Error = error::Taskmaster;

    fn try_from(readtask: &ReadTask) -> Result<Self, Self::Error> {
        Ok(Self {
            name: readtask.name.clone(),
            cmd: readtask
                .cmd
                .split(' ')
                .map(std::string::ToString::to_string)
                .collect(),
            numprocess: readtask.numprocess.unwrap_or(default::NUMPROCESS),
            autostart: readtask.autostart.unwrap_or(default::AUTOSTART),
            umask: readtask.umask.unwrap_or(default::UMASK),

            retry: readtask.retry.unwrap_or(default::RETRY),

            successdelay: readtask.successdelay.unwrap_or(default::SUCCESS_DELAY),

            expected_exit_codes: readtask
                .exitcodes
                .as_ref()
                .unwrap_or(&default::EXPECTED_EXIT_CODES)
                .clone(),

            stopsignal: signal::Signal::from_str(
                &readtask
                    .stopsignal
                    .as_ref()
                    .unwrap_or(&String::from(default::STOP_SIGNAL)),
            )?,
            stopdelay: readtask.stopdelay.unwrap_or(default::STOP_DELAY),

            workingdir: PathBuf::from(
                readtask
                    .workingdir
                    .as_ref()
                    .unwrap_or(&String::from(default::WORKDIR))
                    .as_str(),
            ),
            stdout: PathBuf::from(
                readtask
                    .stdout
                    .as_ref()
                    .unwrap_or(&String::from(default::STDOUT))
                    .as_str(),
            ),
            stderr: PathBuf::from(
                readtask
                    .stderr
                    .as_ref()
                    .unwrap_or(&String::from(default::STDERR))
                    .as_str(),
            ),
        })
    }
}

impl Task {
    pub fn run(&self) -> Vec<Child> {
        let mut jobs = Vec::new();
        let mut command: Command = Command::new(&self.cmd[0]);
        let stdout = File::create(self.stdout.as_path()).unwrap();
        let stderr = File::create(self.stderr.as_path()).unwrap();
        if self.cmd.len() > 1 {
            command.args(&self.cmd[1..]);
        }
        command.current_dir(self.workingdir.as_path());
        command.stdout(stdout);
        command.stderr(stderr);
        for _ in 0..self.numprocess {
            jobs.push(command.spawn().expect("Couldn't run command!"));
        }
        jobs
    }
}
