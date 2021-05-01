use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

mod default;
pub mod error;
mod listener;
mod reader;
mod relaunch;
mod signal;
mod state;
mod task;
mod watcher;
mod worker;

use self::watcher::Watcher;
use self::{listener::Listener, state::State};

pub struct Communication {
    message: Message,
    channel: Option<Sender<String>>,
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

pub fn start(config: &str) {
    let (sender, receiver) = channel();
    let mut state = State::new();
    let mut watcher = Watcher::try_from(config).unwrap();
    let mut listener = Listener::new();

    listener.run(sender.clone());
    watcher.run(sender);
    loop {
        if let Ok(com) = receiver.recv() {
            log::info!("received message: {:?}", com.message);
            match com.message {
                Message::Reload => state.reload(&watcher),
                Message::Start(task) => state.start(&task),
                Message::Stop(task) => state.stop(&task),
                Message::List => state.list(&com.channel.unwrap()),
                Message::Status(_task) => {
                    unimplemented!();
                }
                Message::Quit => break,
            };
        };
    }
}
