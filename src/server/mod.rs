use std::sync::mpsc::Sender;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::mpsc::channel;

pub mod error;
mod reader;
mod state;
mod task;
mod watcher;
mod worker;

use self::state::State;
use self::watcher::Watcher;

pub struct Communication {
    message: Message,
    channel: Option<Sender<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    List,
    Quit,
}

pub fn start_server(config: &str) {
    let (sender, receiver) = channel();
    let mut state = State::new("/tmp/taskmaster.sock");
    let mut watcher = Watcher::try_from(config).unwrap();

    state.listen(sender.clone());
    watcher.run(sender);
    loop {
        if let Ok(com) = receiver.recv() {
            match com.message {
                Message::Reload => state.reload(&watcher),
                Message::Start(task) => state.start(&task),
                Message::Stop(task) => state.stop(&task),
                Message::List => state.list(com.channel.unwrap()),
                Message::Status(_task) => {
                    unimplemented!();
                }
                Message::Quit => break,
            };
        };
    }
}
