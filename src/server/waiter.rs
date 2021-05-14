use super::message::Inter;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

pub struct Waiter {
    counter: Arc<AtomicUsize>,
    thread: Option<JoinHandle<()>>,
}

impl Waiter {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
            thread: None,
        }
    }

    pub fn wait_children(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
        if self.thread.is_none() {
            self.thread = Some(thread::spawn(|| {}));
        } else {
            log::debug!("waiting thread already running");
        }
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
