use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{convert::TryFrom, sync::Arc};
use std::{process::Child, thread};

use super::reader::ReadTask;
use super::task::Task;

pub enum Action {
    Reload(ReadTask),
    Status,
    Stop,
}

// Status of the task
pub enum Status {
    NotStarted,
    Running,
    Failing,
    Finished,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Status::NotStarted => "not started",
            Status::Running => "running",
            Status::Failing => "failing",
            Status::Finished => "finished",
        };
        write!(f, "{}", name)
    }
}

fn monitor(m: Arc<(String, Mutex<Vec<Child>>)>, sender: Sender<Status>) {
    thread::spawn(move || {
        let delay: Duration = Duration::from_secs(10);
        let mut finished: Vec<u32> = Vec::new();
        log::debug!("Start monitoring");
        loop {
            let mut jobs = m.1.lock().unwrap();
            let name = &m.0;

            for child in &mut *jobs {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        log::debug!("[{}] child exited with status {}", name, status);
                        finished.push(child.id())
                    }
                    Ok(None) => {
                        log::debug!("[{}] Not finished yet!", name);
                    }
                    Err(_) => {
                        log::debug!("[{}] Something went wrong", name);
                    }
                }
            }
            jobs.retain(|child| !finished.iter().any(|&done| done == child.id()));
            finished.clear();
            if jobs.is_empty() {
                log::debug!("[{}] Finished!", name);
                sender.send(Status::Finished).unwrap();
                break;
            }
            drop(jobs);
            log::debug!("[{}] Went to sleep", name);
            thread::sleep(delay);
        }
    });
}

pub fn run(taskname: &str, task: Task, sender: Sender<Status>, receiver: Receiver<Action>) {
    let taskid = String::from(taskname);
    thread::spawn(move || {
        let jobs = task.run();

        let m = Arc::new((taskid, Mutex::new(jobs)));
        monitor(m.clone(), sender.clone());
        loop {
            if let Ok(action) = receiver.try_recv() {
                match action {
                    Action::Reload(t) => {
                        let task = Task::try_from(&t).unwrap();
                        let mut vec = m.1.lock().unwrap();
                        vec.iter_mut().for_each(|child| child.kill().unwrap());
                        vec.clear();
                        *vec = task.run();
                    }
                    Action::Stop => {
                        let mut vec = m.1.lock().unwrap();
                        vec.iter_mut().for_each(|child| child.kill().unwrap());
                        break;
                    }
                    Action::Status => {
                        sender.send(Status::Running).unwrap();
                    }
                }
            }
        }
    });
}
