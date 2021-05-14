use super::communication::Communication;
use nix::{sys::wait::WaitStatus, unistd::Pid};

pub enum Inter {
    // When we receive a message from the client
    FromClient(Communication),

    // When a children have exited
    ChildrenExited(Pid, WaitStatus),

    // When we've to wait children
    ChildrenToWait,
}
