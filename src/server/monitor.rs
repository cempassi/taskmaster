use super::task::Task;
use serde::Serialize;
use std::fmt::{self, Debug, Display, Formatter};
use std::process::Child;

#[derive(Copy, Clone, Serialize)]
pub enum Status {
    Inactive,
    Active,
    Reloading,
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

    #[serde(skip_serializing)]
    children: Vec<Child>,
    state: Status,
}

impl Monitor {
    // Only create Monitoring struct
    pub fn new_only(id: String, task: Task) -> Self {
        Monitor {
            id,
            task,
            children: Vec::new(),
            state: Status::Inactive,
        }
    }

    // Create new Monitoring struct and start the task if required
    pub fn new(id: String, task: Task) -> Self {
        let mut mon = Monitor::new_only(id, task);
        if mon.task.autostart {
            mon.start();
        }
        mon
    }

    fn change_state(&mut self, status: Status) {
        self.state = status
    }

    pub fn start(&mut self) {
        // code here
        self.change_state(Status::Active);
        unimplemented!();
    }

    pub fn stop(&mut self) {
        self.change_state(Status::Stopping);
        self.stop_raw();
        self.change_state(Status::Stopped);
    }

    fn stop_raw(&mut self) {
        self.children
            .iter_mut()
            .for_each(|child| child.kill().expect("cannot kill children"));
        self.children.clear();
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(&mut self, task: Task) {
        if self.task != task {
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
