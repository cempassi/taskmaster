use libc::{gid_t, mode_t, uid_t};
use serde::Deserialize;
use std::convert::TryFrom;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::vec::Vec;

use super::{error, reader::ReadTask, relaunch::Relaunch, signal};

#[derive(Debug, Deserialize)]
enum AutoRestart {
    Unexpected,
    True,
    False,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    cmd: Vec<String>,
    numprocess: u32,
    autostart: bool,
    umask: mode_t,
    workingdir: PathBuf,

    stdout: PathBuf,
    stderr: PathBuf,

    stopsignal: signal::Signal,
    stopdelay: u32,

    retry: u32,

    successdelay: u32,

    expected_exit_codes: Vec<i32>,

    restart: Relaunch,

    env: Vec<String>,
    uid: Option<uid_t>,
    gid: Option<gid_t>,
}

impl TryFrom<&ReadTask> for Task {
    type Error = error::Taskmaster;

    fn try_from(readtask: &ReadTask) -> Result<Self, Self::Error> {
        Ok(Self {
            cmd: readtask
                .cmd
                .split(' ')
                .map(std::string::ToString::to_string)
                .collect(),
            numprocess: readtask.numprocess,
            autostart: readtask.autostart,
            umask: readtask.umask,

            retry: readtask.retry,

            successdelay: readtask.successdelay,

            uid: readtask.uid,
            gid: readtask.gid,

            env: readtask.env.clone(),

            restart: readtask.restart.clone(),

            expected_exit_codes: readtask.exitcodes.clone(),

            stopsignal: readtask.stopsignal.clone(),
            stopdelay: readtask.stopdelay,

            workingdir: PathBuf::from(readtask.workingdir.as_str()),
            stdout: PathBuf::from(readtask.stdout.as_str()),
            stderr: PathBuf::from(readtask.stderr.as_str()),
        })
    }
}

impl Task {
    pub fn run(&self) -> Vec<Child> {
        let mut jobs = Vec::new();
        let mut command = Command::new(&self.cmd[0]);
        let stdout = File::create(self.stdout.as_path()).unwrap();
        let stderr = File::create(self.stderr.as_path()).unwrap();
        self.setup_command(&mut command);
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
            self.expected_exit_codes
                .iter()
                .any(|&code| code != exitcode)
        })
    }
}
