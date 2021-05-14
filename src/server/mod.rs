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
mod waiter;
mod watcher;

use self::{
    communication::Communication, listener::Listener, message::Inter, state::State, waiter::Waiter,
    watcher::Watcher,
};

struct Server {
    state: State,
    event_sender: Sender<Inter>,
}

pub fn start(config: &str) -> Result<(), error::Taskmaster> {
    let (sender, receiver) = channel::<Inter>();
    let mut watcher = Watcher::try_from(config)?;
    let mut msg_listener = Listener::new();
    let mut waiter = Waiter::new(sender.clone());
    let mut server = Server {
        state: State::new(sender.clone()),
        event_sender: sender.clone(),
    };

    watcher.run(sender.clone());
    msg_listener.run(sender.clone());

    signal::handle_signals(sender)?;
    loop {
        if let Ok(message) = receiver.recv() {
            log::info!("received internal message: {:?}", message);
            match message {
                Inter::ChildrenExited(_pid, _status) => unimplemented!(),
                Inter::ChildrenToWait(count) => waiter.wait_children(count),
                Inter::NoMoreChildrenToWait => waiter.done_wait_children(),
                Inter::FromClient(com) => server.handle_client_message(com),
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

    fn handle_client_message(&mut self, com: Communication) {
        use crate::shared::message::Message;

        match com.message {
            Message::Reload => self.event_sender.send(Inter::Reload).unwrap(),
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
