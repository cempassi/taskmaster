use std::convert::TryFrom;
use std::sync::mpsc::{channel, Sender};

mod communication;
mod default;
pub mod error;
mod inter;
mod listener;
mod monitor;
mod nix_utils;
mod relaunch;
mod signal;
mod state;
mod task;
mod watcher;

use crate::shared::message::Message;

use self::{communication::Com, inter::Inter, listener::Listener, state::State, watcher::Watcher};

struct Server {
    state: State,
    event: Sender<Inter>,
}

pub fn start(config: &str) -> Result<(), error::Taskmaster> {
    let (sender, event) = channel::<Inter>();
    let (response, receiver) = channel::<Com>();
    let mut watcher = Watcher::try_from(config)?;
    let mut listener = Listener::new();
    let mut server = Server {
        state: State::new(sender.clone(), response.clone()),
        event: sender.clone(),
    };

    watcher.run(sender.clone());
    listener.run(sender.clone(), receiver);

    signal::handle_signals(sender)?;
    loop {
        if let Ok(message) = event.recv() {
            log::info!("received internal message: {:?}", message);
            match message {
                Inter::FromClient(msg) => {
                    server.handle_client_message(msg);
                    response.send(Com::End).unwrap();
                }
                Inter::Reload => server.reload_config(&watcher),
                Inter::Quit => break,
            }
        };
    }
    Ok(())
}

impl Server {
    fn reload_config(&mut self, watcher: &Watcher) {
        self.state.reload(watcher)
    }

    fn handle_client_message(&mut self, message: Message) {
        match message {
            Message::Reload => self.event.send(Inter::Reload).unwrap(),
            Message::Start { id: taskname } => self.state.start(&taskname),
            Message::Info { id: taskname } => self.state.info(&taskname),
            Message::Stop { id: taskname } => self.state.stop(&taskname),
            Message::List => self.state.list(),
            Message::Status { id: taskname } => self.state.status(&taskname),
            Message::Restart { id: taskname } => {
                self.state.stop(&taskname);
                self.state.start(&taskname);
            }
            Message::Quit => self
                .event
                .send(Inter::Quit)
                .expect("cannot send quit message"),
        };
    }
}
