use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{convert::TryFrom, sync::Arc};
use std::{process::Child, thread};

use crate::reader::ReadTask;
use crate::task::Task;

pub enum Action {
    Reload(ReadTask),
    Finished,
}

fn monitor(m: Arc<Mutex<Vec<Child>>>, sender: Sender<Action>) {
    thread::spawn(move || {
        let delay: Duration = Duration::from_secs(10);
        let mut finished: Vec<u32> = Vec::new();
        println!("Start monitoring");
        loop {
            let mut jobs = m.lock().unwrap();

            for child in &mut *jobs {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("Exited with status {}", status);
                        finished.push(child.id())
                    }
                    Ok(None) => {
                        println!("Not finished yet!");
                    }
                    Err(_) => {
                        println!("Something went wrong");
                    }
                }
            }
            jobs.retain(|child| finished.iter().any(|&done| done == child.id()));
            finished.clear();
            if jobs.is_empty() {
                println!("Finished!");
                sender.send(Action::Finished).unwrap();
                break;
            } else {
                drop(jobs);
                println!("Went to sleep");
                thread::sleep(delay);
            }
        }
    });
}

pub fn run(task: Task, sender: Sender<Action>, receiver: Receiver<Action>) {
    thread::spawn(move || {
        let mut jobs = Vec::new();
        jobs.push(task.run());

        let m = Arc::new(Mutex::new(jobs));
        monitor(m.clone(), sender);
        loop {
            if let Ok(action) = receiver.try_recv() {
                match action {
                    Action::Reload(t) => {
                        let task = Task::try_from(&t).unwrap();
                        let mut vec = m.lock().unwrap();
                        vec.clear();
                        vec.push(task.run());
                        drop(vec);
                    }
                    Action::Finished => break,
                }
            }
        }
    });
}
