use super::communication::Communication;
use nix::{sys::wait::WaitStatus, unistd::Pid};

#[derive(Debug, Clone)]
pub enum Inter {
    // When we receive a message from the client
    FromClient(Communication),

    // Server need to quit
    Quit,

    // Reload the configuration file
    Reload,

    // When a child have exited
    ChildHasExited(Pid, WaitStatus),

    // When we've to wait `usize` children
    ChildrenToWait(usize),

    // When all the children in a task have exited
    NoMoreChildrenToWait,
}
