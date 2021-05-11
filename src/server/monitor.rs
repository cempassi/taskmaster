use super::task::Task;
use serde::Serialize;
use std::fmt::{self, Debug, Display, Formatter};
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Copy, Clone, Serialize, PartialEq)]
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

    #[serde(skip)]
    children: Arc<Mutex<Vec<Child>>>,

    #[serde(skip)]
    state: Arc<Mutex<Status>>,
}

impl Monitor {
    // Only create Monitoring struct
    pub fn new_only(id: String, task: Task) -> Self {
        Monitor {
            id,
            task,
            children: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(Status::Inactive)),
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
        *self.state.lock().unwrap() = status;
    }

    pub fn start(&mut self) {
        log::debug!("[{}] starting ...", self.id);
        self.children.lock().unwrap().extend(self.task.run());
        self.spaw_children_watcher();
        self.change_state(Status::Active);
    }

    fn spaw_children_watcher(&mut self) {
        let id = self.id.clone();
        let children_mutex = self.children.clone();
        let state_mutex = self.state.clone();
        let expected_exit_codes = self.task.exitcodes.clone();

        std::thread::spawn(move || {
            let sleep_delay = Duration::from_secs(10);
            loop {
                let mut children = children_mutex.lock().unwrap();
                let mut finished = Vec::<u32>::new();

                for child in children.iter_mut() {
                    match child.try_wait() {
                        Ok(Some(st)) => {
                            log::debug!(
                                "[{}] child-{} finished with status {}",
                                id,
                                child.id(),
                                st
                            );
                            match st.code() {
                                Some(code) => {
                                    if !expected_exit_codes.iter().any(|wanted| code == *wanted) {
                                        *state_mutex.lock().unwrap() = Status::Failed;
                                    }
                                }
                                None => {
                                    *state_mutex.lock().unwrap() = Status::Failed;
                                }
                            }
                            // FIXME: check status with exitcodes and update monitor status
                            finished.push(child.id())
                        }
                        Ok(None) => {
                            log::debug!("[{}] child-{} not finished yet!", id, child.id());
                        }
                        Err(e) => {
                            log::error!(
                                "[{}] while waiting for child {} got error {}",
                                id,
                                child.id(),
                                e
                            );
                        }
                    }
                }

                children.retain(|child| !finished.iter().any(|&id| id == child.id()));

                if children.is_empty() {
                    log::info!("[{}] task finished !", id);
                    let mut state = state_mutex.lock().unwrap();
                    if *state != Status::Failed {
                        *state = Status::Finished;
                    }
                    // FIXME: update monitor status
                    break;
                }
                drop(children);
                log::debug!("[{}] watcher went to sleep", id);
                std::thread::sleep(sleep_delay);
            }
        });
    }

    pub fn stop(&mut self) {
        log::debug!("[{}] stopping ...", self.id);
        self.change_state(Status::Stopping);
        self.stop_raw();
        self.change_state(Status::Stopped);
    }

    fn stop_raw(&mut self) {
        let mut children = self.children.lock().unwrap();
        children
            .iter_mut()
            .for_each(|child| child.kill().expect("cannot kill children"));
        children.clear();
    }

    pub fn status(&self) -> Status {
        *self.state.lock().unwrap()
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
