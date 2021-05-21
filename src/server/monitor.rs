use super::{inter::Inter, task::Task, waiter::WaitChildren};
use serde::Serialize;
use std::{
    fmt::{self, Debug, Display, Formatter},
    process::ExitStatus,
    sync::mpsc::Sender,
    time,
};

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
    children_pid: Vec<u32>,

    #[serde(skip)]
    state: Status,

    #[serde(skip)]
    sender: Sender<Inter>,
}

// impl Drop for Monitor {
//     fn drop(&mut self) {
//         for child in &mut self.children {
//             child.kill().expect("cannot kill children");
//         }
//         self.children.clear();
//     }
// }

impl Monitor {
    // Only create Monitoring struct
    pub fn new_only(id: String, task: Task, sender: Sender<Inter>) -> Self {
        Monitor {
            id,
            task,
            children_pid: Vec::new(),
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
        let current_status = self.status();
        if current_status == Status::Inactive
            || current_status == Status::Finished
            || current_status == Status::Failed
        {
            self.start_raw();
        } else {
            log::warn!("[{}] already started", self.id);
        }
    }

    fn start_raw(&mut self) {
        log::debug!("[{}] starting ...", self.id);
        let mut children = self.task.run();

        self.children_pid = children.iter_mut().map(|chld| chld.id()).collect();
        self.sender
            .send(Inter::ChildrenToWait(WaitChildren::new(
                self.id.clone(),
                children,
                time::Duration::from_secs(self.task.stopdelay.into()),
                self.task.stopsignal,
            )))
            .unwrap();
        self.change_state(Status::Active);
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(&mut self, task: Task) {
        if self.task != task {
            log::debug!("[{}] reloading ...", self.id);
            self.change_state(Status::Reloading);
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

    pub fn ev_child_has_exited(&mut self, pid: u32, status: ExitStatus) -> bool {
        if self.children_pid.iter().any(|&chld_pid| chld_pid == pid) {
            self.children_pid.retain(|&chld_pid| chld_pid != pid);
            self.handle_finished_child(status);
            if self.children_pid.len() == 0 {
                self.state = match self.state {
                    Status::Failing => Status::Failed,
                    _ => Status::Finished,
                };
            }
            true
        } else {
            log::error!("[{}] pid {} is not registred to this monitor", self.id, pid);
            false
        }
    }

    fn handle_finished_child(&mut self, status: ExitStatus) {
        if let Some(code) = status.code() {
            if !self.task.exitcodes.iter().any(|&wanted| wanted == code) {
                log::debug!(
                    "[{}] child exited with unexpeced status code {}",
                    self.id,
                    code
                );
                self.state = Status::Failing;
            }
        } else {
            log::warn!("[{}] unexpected exit status {:?}", self.id, status);
            self.state = Status::Failing;
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

impl Debug for Monitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(s) = serde_json::to_string(self) {
            write!(f, "{}", s)
        } else {
            Err(fmt::Error)
        }
    }
}
