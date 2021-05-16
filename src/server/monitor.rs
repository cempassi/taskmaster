use super::{message::Inter, task::Task};
use nix::{
    sys::{
        signal::{self, Signal},
        wait::WaitStatus,
    },
    unistd::Pid,
};
use serde::Serialize;
use std::fmt::{self, Debug, Display, Formatter};
use std::process::{Child, ExitStatus};
use std::sync::mpsc::Sender;

#[derive(Copy, Clone, Serialize, PartialEq)]
pub enum Status {
    Inactive,
    Active,
    Reloading,
    Failing,
    Finished,
    Failed,
    Stopping,
    Stopped,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Inactive => "inactive",
            Status::Active => "active",
            Status::Reloading => "reloading",
            Status::Failing => "failing",
            Status::Finished => "finished",
            Status::Failed => "failed",
            Status::Stopping => "stopping",
            Status::Stopped => "stopped",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize)]
pub struct Monitor {
    id: String,
    task: Task,

    #[serde(skip)]
    children: Vec<Child>,

    #[serde(skip)]
    state: Status,

    #[serde(skip)]
    sender: Sender<Inter>,
}

impl Monitor {
    // Only create Monitoring struct
    pub fn new_only(id: String, task: Task, sender: Sender<Inter>) -> Self {
        Monitor {
            id,
            task,
            children: Vec::new(),
            state: Status::Inactive,
            sender,
        }
    }

    // Create new Monitoring struct and start the task if required
    pub fn new(id: String, task: Task, sender: Sender<Inter>) -> Self {
        let mut mon = Monitor::new_only(id, task, sender);
        if mon.task.autostart {
            mon.start();
        }
        mon
    }

    fn change_state(&mut self, status: Status) {
        self.state = status;
    }

    pub fn start(&mut self) {
        if self.status() == Status::Inactive {
            self.start_raw();
        } else {
            log::warn!("[{}] already started", self.id);
        }
    }

    fn start_raw(&mut self) {
        log::debug!("[{}] starting ...", self.id);
        self.children.extend(self.task.run());
        self.sender
            .send(Inter::ChildrenToWait(self.task.numprocess as usize))
            .unwrap();
        self.change_state(Status::Active);
    }

    pub fn stop(&mut self) {
        if self.status() == Status::Active {
            log::debug!("[{}] stopping ...", self.id);
            self.change_state(Status::Stopping);
            self.stop_raw();
            self.change_state(Status::Stopped);
        } else {
            log::warn!("[{}] can't stop task that is not started", self.id);
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn stop_raw(&mut self) {
        // send stop signal to children
        self.children.iter_mut().for_each(|child| {
            signal::kill(Pid::from_raw(child.id() as i32), Signal::SIGSTOP)
                .expect("cannot send stop signal to child")
        });
        // wait stop delay
        std::thread::sleep(std::time::Duration::from_secs(self.task.stopdelay.into()));

        // kill remaining children that don't have exited
        self.children.iter_mut().for_each(|child| {
            let to_kill = child_has_exited(child).is_none();
            if to_kill {
                child.kill().expect("cannot kill children")
            }
        });
        self.children.clear();
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(&mut self, task: Task) {
        if self.task != task {
            log::debug!("[{}] reloading ...", self.id);
            self.change_state(Status::Reloading);
            self.stop_raw();
            self.task = task;
            if self.task.autostart {
                self.start();
            } else {
                self.change_state(Status::Inactive);
            }
        }
    }

    pub fn get_task(&self) -> &Task {
        &self.task
    }

    pub fn ev_child_has_exited(&mut self, pid: Pid, status: &WaitStatus) -> bool {
        let raw_pid = pid.as_raw().abs() as u32;

        if let Some(index) = self.children.iter().position(|child| child.id() == raw_pid) {
            self.check_wait_status(raw_pid, status);
            log::debug!("[{}] remove children {}", self.id, raw_pid);
            self.children.remove(index);
            if self.children.is_empty() {
                self.update_finished_task_status();
            }
            true
        } else {
            false
        }
    }

    fn check_wait_status(&mut self, pid: u32, status: &WaitStatus) {
        if let WaitStatus::Exited(_, code) = status {
            if !self.task.exitcodes.iter().any(|wanted| wanted == code) {
                log::debug!(
                    "[{}] child {} exited with unexpected status code {}",
                    self.id,
                    pid,
                    code
                );
                self.state = Status::Failing;
            }
        } else {
            log::warn!("[{}] unexpected wait status {:?}", self.id, status);
            self.state = Status::Failing
        }
    }

    fn update_finished_task_status(&mut self) {
        if self.state == Status::Failing {
            self.state = Status::Failed;
        } else {
            self.state = Status::Finished;
        }
    }
}

fn child_has_exited(child: &mut Child) -> Option<ExitStatus> {
    match child.try_wait() {
        Ok(None) => None,
        Err(e) => {
            log::error!("while waiting for child {} got error {}", child.id(), e);
            None
        }
        Ok(Some(status)) => Some(status),
    }
}

impl Debug for Monitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(s) = serde_json::to_string(self) {
            write!(f, "{}", s)
        } else {
            Err(fmt::Error)
        }
    }
}
