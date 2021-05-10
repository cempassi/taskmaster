use super::task::Task;
use std::process::Child;

#[derive(Copy, Clone)]
pub enum Status {
    Inactive,
    Active,
    Reloading,
    Finished,
    Failing,
}

pub struct Monitor {
    task: Task,
    children: Vec<Child>,
    state: Status,
}

impl Monitor {
    pub fn new(task: Task) -> Self {
        Monitor {
            task,
            children: Vec::new(),
            state: Status::Inactive,
        }
    }

    fn change_state(&mut self, status: Status) {
        self.state = status
    }

    pub fn start() {
        unimplemented!();
    }

    pub fn stop() {
        unimplemented!();
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(task: Task) {}
}
