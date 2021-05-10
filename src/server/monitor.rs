use super::task::Task;
use serde::Serialize;
use std::fmt::{self, Debug, Formatter};
use std::process::Child;

#[derive(Copy, Clone, Serialize)]
pub enum Status {
    Inactive,
    Active,
    Reloading,
    Finished,
    Failing,
}

#[derive(Serialize)]
pub struct Monitor {
    task: Task,

    #[serde(skip_serializing)]
    children: Vec<Child>,
    state: Status,
}

impl Monitor {
    // Only create Monitoring struct
    pub fn new_only(task: Task) -> Self {
        Monitor {
            task,
            children: Vec::new(),
            state: Status::Inactive,
        }
    }

    // Create new Monitoring struct and start the task if required
    pub fn new(task: Task) -> Self {
        let mut mon = Monitor::new_only(task);
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
        // code here
        self.change_state(Status::Finished);
        unimplemented!();
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(&mut self, task: Task) {
        self.change_state(Status::Reloading);
        unimplemented!()
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
