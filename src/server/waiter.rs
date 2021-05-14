use super::message::Inter;
use std::sync::{atomic::AtomicUsize, Arc};

pub struct Waiter {
    counter: Arc<AtomicUsize>,
}

impl Waiter {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}
