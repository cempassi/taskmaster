use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{convert::TryFrom, sync::Arc};
use std::{
    process::{Child, ExitStatus},
    thread,
};

use super::reader::ReadTask;
use super::task::Task;

pub enum Action {
    Reload(ReadTask),
    Status,
    Stop,
}

// Status of the task
#[derive(Copy, Clone)]
pub enum Status {
    NotStarted,
    Running,
    Failing,
    Finished,
    Unknown,
    Stopped,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Status::NotStarted => "not started",
            Status::Running => "running",
            Status::Failing => "failing",
            Status::Finished => "finished",
            Status::Stopped => "stopped",
            Status::Unknown => "unknown",
        };
        write!(f, "{}", name)
    }
}

struct MonitorTask {
    name: String,
    childs: Vec<Child>,
    finished: Vec<(u32, ExitStatus)>,
    status: Status,
}

impl MonitorTask {
    fn new(name: String, childs: Vec<Child>) -> Self {
        MonitorTask {
            name,
            childs,
            finished: Vec::new(),
            status: Status::Running,
        }
    }
}

fn monitor(m: Arc<Mutex<MonitorTask>>, _sender: &Sender<Status>) {
    thread::spawn(move || {
        let delay: Duration = Duration::from_secs(10);
        // let mut finished: Vec<u32> = Vec::new();
        log::debug!("Start monitoring");
        loop {
            let mut mon = m.lock().unwrap();
            let name = mon.name.to_string();
            let mut finished = mon.finished.clone();

            for child in &mut mon.childs {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        log::debug!("[{}] child exited with status {}", name, status);
                        finished.push((child.id(), status))
                    }
                    Ok(None) => {
                        log::debug!("[{}] Not finished yet!", name);
                    }
                    Err(_) => {
                        log::debug!("[{}] Something went wrong", name);
                    }
                }
            }
            mon.childs
                .retain(|child| !finished.iter().any(|&(id, _st)| id == child.id()));
            mon.finished = finished;
            if mon.childs.is_empty() {
                log::debug!("[{}] Finished!", name);
                mon.status = Status::Finished;
                break;
            }
            drop(mon);
            log::debug!("[{}] Went to sleep", name);
            thread::sleep(delay);
        }
    });
}

pub fn run(taskname: &str, task: Task, sender: Sender<Status>, receiver: Receiver<Action>) {
    let taskid = String::from(taskname);
    thread::spawn(move || {
        let jobs = task.run();

        let m = Arc::new(Mutex::new(MonitorTask::new(taskid.to_string(), jobs)));
        monitor(m.clone(), &sender);
        loop {
            if let Ok(action) = receiver.try_recv() {
                match action {
                    Action::Reload(t) => {
                        let task = Task::try_from(&t).unwrap();
                        let mut mon = m.lock().unwrap();
                        mon.childs
                            .iter_mut()
                            .for_each(|child| child.kill().unwrap());
                        mon.childs.clear();
                        mon.childs = task.run();
                    }
                    Action::Stop => {
                        let mut mon = m.lock().unwrap();
                        mon.childs
                            .iter_mut()
                            .for_each(|child| child.kill().unwrap());
                        mon.status = Status::Stopped;
                        break;
                    }
                    Action::Status => {
                        let mon = m.lock().unwrap();

                        log::debug!("[{}] current worker status: {}", taskid, mon.status);
                        let st = match mon.status {
                            Status::Finished => {
                                if mon
                                    .finished
                                    .iter()
                                    .any(|&(_id, status)| task.check_exit_status(status))
                                {
                                    Status::Failing
                                } else {
                                    Status::Finished
                                }
                            }
                            _ => mon.status,
                        };
                        sender.send(st).unwrap();
                    }
                }
            }
        }
    });
}
