use crate::shared::message::Message;
use std::convert::TryFrom;
use std::sync::mpsc::{channel, Sender};

mod communication;
mod default;
pub mod error;
mod listener;
mod message;
mod monitor;
mod nix_utils;
mod relaunch;
mod signal;
mod state;
mod task;
mod watcher;

use self::{
    communication::Communication, listener::Listener, message::Inter, state::State,
    watcher::Watcher,
};

struct Server {
    state: State,
    watcher: Watcher,
    msg_listener: Listener,
    event_sender: Sender<Inter>,
}

pub fn start(config: &str) -> Result<(), error::Taskmaster> {
    let (sender, receiver) = channel::<Inter>();
    let mut state = State::new();
    let mut watcher = Watcher::try_from(config).unwrap();
    let mut listener = Listener::new();

    listener.run(sender.clone());
    watcher.run(sender.clone());

    let mut server = Server {
        state,
        watcher,
        msg_listener: listener,
        event_sender: sender.clone(),
    };

    signal::handle_signals(sender)?;
    loop {
        if let Ok(message) = receiver.recv() {
            log::info!("received internal message: {:?}", message);
            match message {
                Inter::ChildrenExited(pid, status) => unimplemented!(),
                Inter::ChildrenToWait => unimplemented!(),
                Inter::FromClient(com) => server.handle_client_message(com),
                Inter::Reload => server.reload_config(),
                Inter::Quit => break,
            }
        };
    }
    Ok(())
}

impl Server {
    fn reload_config(&mut self) {
        self.state.reload(&self.watcher)
    }

    fn handle_client_message(&mut self, com: Communication) {
        match com.message {
            Message::Reload => self.event_sender.send(Inter::Reload),
            Message::Start(taskname) if com.channel.is_some() => {
                self.state.start(&taskname);
            }
            Message::Start(taskname) => self.state.start(&taskname),
            Message::Stop(taskname) => self.state.stop(&taskname),
            Message::List => self.state.list(&com.channel.unwrap()),
            Message::Status(taskname) => self.state.status(&taskname, &com.channel.unwrap()),
            Message::Quit => self
                .event_sender
                .send(Inter::Quit)
                .expect("cannot send quit message"),
        };
    }
}
