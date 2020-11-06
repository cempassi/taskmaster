use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::vec::Vec;

use crate::error::TaskmasterError;

#[derive(Debug, Deserialize)]
enum AutoRestart {
    Unexpected,
    True,
    False,
}

#[derive(Debug, Deserialize)]
pub struct ReadTask {
    name: String,
    cmd: String,
    numprocess: i32,
    umask: i16,
    workingdir: String,
    stdout: String,
    stderr: String,
}

impl ReadTask {
    pub fn new(path: &str) -> Result<Self, TaskmasterError> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_e) => return Err(TaskmasterError::ReadFile),
        };

        let parsed = match toml::from_str(&content) {
            Ok(c) => Ok(c),
            Err(_e) => Err(TaskmasterError::Parse),
        };
        dbg!(&parsed);
        parsed
    }
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
            workingdir: PathBuf::from(readtask.workingdir),
            stdout: File::create(readtask.stdout).unwrap(),
            stderr: File::create(readtask.stderr).unwrap(),
        })
    }
}

impl FromStr for Task {
    type Err = TaskmasterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let task: Task = ReadTask::new(s)?.try_into()?;
        dbg!("config: {:?}", &task);
        Ok(task)
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

//Status d'un job
//enum State {
//	RUNNING,
//	STOPPED,
//	EXITED,
//	KILLED
//}
//
