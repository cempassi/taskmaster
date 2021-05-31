use super::manager::WaitChildren;
use crate::shared::message::Message;
use std::process::ExitStatus;

#[derive(Debug)]
pub enum Inter {
    // When we receive a message from the client
    FromClient(Message),

    // Server need to quit
    Quit,

    // Reload the configuration file
    Reload,

    // When a child have exited
    ChildHasExited(String, u32, ExitStatus),

    // When we've to wait `usize` children
    ChildrenToWait(WaitChildren),

    // When all the children in a task have exited
    NoMoreChildrenToWait,
}
