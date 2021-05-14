use super::message::Inter;
use nix::{
    sys::wait::{waitpid, WaitStatus},
    unistd::Pid,
};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::{self, JoinHandle},
};

pub struct Waiter {
    counter: Arc<AtomicUsize>,
    thread: Option<JoinHandle<()>>,
    sender: Sender<Inter>,
}

impl Waiter {
    pub fn new(sender: Sender<Inter>) -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
            thread: None,
            sender,
        }
    }

    pub fn wait_children(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);

        if self.thread.is_none() {
            self.spawn_waiting_thread();
        } else {
            log::debug!("waiting thread already running");
        }
    }

    fn spawn_waiting_thread(&mut self) {
        let sender = self.sender.clone();
        let counter = self.counter.clone();

        self.thread = Some(thread::spawn(move || {
            log::debug!("thread to wait subprocess spawned !");
            while counter.load(Ordering::SeqCst) > 0 {
                match waitpid(Pid::from_raw(-1), None) {
                    Ok(status) => match status {
                        WaitStatus::Exited(pid, _) | WaitStatus::Signaled(pid, _, _) => {
                            log::debug!("a process has exited {:?}", status);
                            sender.send(Inter::ChildrenExited(pid, status)).unwrap();
                        }
                        _ => {
                            log::warn!("exit status {:?} not handled", status);
                        }
                    },
                    Err(error) => {
                        log::error!("error while using waitpid: {}", error);
                        panic!("error while waiting for subproccess");
                    }
                }
            }
            log::debug!("wait counter at zero, finished waiting for subprocess");
        }));
    }

    pub fn done_wait_children(&mut self) {
        let previous_value = self.counter.fetch_sub(1, Ordering::SeqCst);

        if previous_value == 0 {
            panic!("We wasn't waiting !");
        } else if previous_value == 1 {
            self.thread
                .take()
                .expect("expected a waiting thread")
                .join()
                .expect("cannot join waiting thread");
        }
    }
}
