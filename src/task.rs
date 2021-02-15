use serde::Deserialize;
use std::convert::TryFrom;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::vec::Vec;

use crate::error::TaskmasterError;
use crate::reader::ReadTask;

#[derive(Debug, Deserialize)]
enum AutoRestart {
    Unexpected,
    True,
    False,
}

#[derive(Debug)]
pub struct Task {
    name: String,
    cmd: Vec<String>,
    numprocess: i32,
    umask: i16,
    workingdir: PathBuf,
    stdout: File,
    stderr: File,
}

impl TryFrom<ReadTask> for Task {
    type Error = TaskmasterError;

    fn try_from(readtask: ReadTask) -> Result<Self, Self::Error> {
        Ok(Self {
            name: readtask.name.clone(),
            cmd: readtask.cmd.split(" ").map(|s| s.to_string()).collect(),
            numprocess: readtask.numprocess,
            umask: readtask.umask,
            workingdir: PathBuf::from(readtask.workingdir.as_str()),
            stdout: File::create(readtask.stdout.as_str())?,
            stderr: File::create(readtask.stderr.as_str())?,
        })
    }
}

impl Task {
    pub fn run(&self) {
        let mut command: Command = Command::new(&self.cmd[0]);
        if self.cmd.len() > 1 {
            command.args(&self.cmd[1..]);
        }
        command.current_dir(self.workingdir.as_path());
        command.stdout(self.stdout.try_clone().unwrap());
        command.stderr(self.stderr.try_clone().unwrap());
        command.status().expect("Couldn't run command!");
    }
}
