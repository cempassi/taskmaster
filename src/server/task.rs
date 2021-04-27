use serde::Deserialize;
use std::convert::TryFrom;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::vec::Vec;

use super::{error, reader::ReadTask};

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
    numprocess: i32,
    autostart: bool,
    umask: i16,
    workingdir: PathBuf,
    stdout: PathBuf,
    stderr: PathBuf,
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
            numprocess: readtask.numprocess,
            autostart: readtask.autostart,
            umask: readtask.umask,
            workingdir: PathBuf::from(readtask.workingdir.as_str()),
            stdout: PathBuf::from(readtask.stdout.as_str()),
            stderr: PathBuf::from(readtask.stderr.as_str()),
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
