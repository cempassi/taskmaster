use nix::sys::signal::Signal;
use std::{process::Child, time};

#[derive(Debug)]
pub struct WaitChildren {
    pub namespace: String,
    pub children: Vec<Child>,
    pub stopdelay: time::Duration,
    pub stopsignal: Signal,
}

impl WaitChildren {
    pub fn new(
        namespace: String,
        children: Vec<Child>,
        stopdelay: time::Duration,
        stopsignal: Signal,
    ) -> Self {
        Self {
            namespace,
            children,
            stopdelay,
            stopsignal,
        }
    }
}
