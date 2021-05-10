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
        unimplemented!();
        self.change_state(Status::Active);
    }

    pub fn stop(&mut self) {
        unimplemented!();
        self.change_state(Status::Finished);
    }

    pub fn status(&self) -> Status {
        self.state
    }

    pub fn reload(&mut self, task: Task) {
        self.change_state(Status::Reloading);
        unimplemented!()
    }
}
