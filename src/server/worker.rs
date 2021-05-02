use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{convert::TryFrom, sync::Arc};
use std::{process::Child, thread};

use super::reader::ReadTask;
use super::task::Task;

pub enum Action {
    Reload(ReadTask),
    Stop,
    Finished,
}

fn monitor(m: Arc<Mutex<Vec<Child>>>, sender: Sender<Action>) {
    thread::spawn(move || {
        let delay: Duration = Duration::from_secs(10);
        let mut finished: Vec<u32> = Vec::new();
        log::debug!("Start monitoring");
        loop {
            let mut jobs = m.lock().unwrap();

            for child in &mut *jobs {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        log::debug!("Exited with status {}", status);
                        finished.push(child.id())
                    }
                    Ok(None) => {
                        log::debug!("Not finished yet!");
                    }
                    Err(_) => {
                        log::debug!("Something went wrong");
                    }
                }
            }
            jobs.retain(|child| finished.iter().any(|&done| done == child.id()));
            finished.clear();
            if jobs.is_empty() {
                log::debug!("Finished!");
                sender.send(Action::Finished).unwrap();
                break;
            }
            drop(jobs);
            log::debug!("Went to sleep");
            thread::sleep(delay);
        }
    });
}

pub fn run(task: Task, sender: Sender<Action>, receiver: Receiver<Action>) {
    thread::spawn(move || {
        let jobs = task.run();

        let m = Arc::new(Mutex::new(jobs));
        monitor(m.clone(), sender);
        loop {
            if let Ok(action) = receiver.try_recv() {
                match action {
                    Action::Reload(t) => {
                        let task = Task::try_from(&t).unwrap();
                        let mut vec = m.lock().unwrap();
                        vec.iter_mut().for_each(|child| child.kill().unwrap());
                        vec.clear();
                        *vec = task.run();
                    }
                    Action::Stop => {
                        let mut vec = m.lock().unwrap();
                        vec.iter_mut().for_each(|child| child.kill().unwrap());
                        break;
                    }
                    Action::Finished => break,
                }
            }
        }
    });
}
