use nix::{sys::wait::WaitStatus, unistd::Pid};
use std::convert::TryFrom;
use std::sync::mpsc::{channel, Sender};

mod communication;
mod default;
pub mod error;
mod listener;
mod inter;
mod monitor;
mod nix_utils;
mod relaunch;
mod signal;
mod state;
mod task;
mod waiter;
mod watcher;

use crate::shared::message::Message;

use self::{
    communication::Com, listener::Listener, inter::Inter, state::State, waiter::Waiter,
    watcher::Watcher,
};

struct Server {
    state: State,
    event_sender: Sender<Inter>,
}

pub fn start(config: &str) -> Result<(), error::Taskmaster> {
    let (sender, event) = channel::<Inter>();
    let (response, receiver) = channel::<Com>();
    let mut watcher = Watcher::try_from(config)?;
    let mut listener = Listener::new();
    let mut waiter = Waiter::new(sender.clone());
    let mut server = Server {
        state: State::new(sender.clone(), response),
        event_sender: sender.clone(),
    };

    watcher.run(sender.clone());
    listener.run(sender.clone(), receiver);

    signal::handle_signals(sender)?;
    loop {
        if let Ok(message) = event.recv() {
            log::info!("received internal message: {:?}", message);
            match message {
                Inter::ChildHasExited(pid, status) => server.ev_child_has_exited(pid, status),
                Inter::ChildrenToWait(count) => waiter.wait_children(count),
                Inter::NoMoreChildrenToWait => waiter.done_wait_children(),
                Inter::FromClient(msg) => server.handle_client_message(msg),
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
            Message::Reload => self.event_sender.send(Inter::Reload).unwrap(),
            Message::Start(taskname) => self.state.start(&taskname),
            Message::Stop(taskname) => self.state.stop(&taskname),
            Message::List => self.state.list(),
            Message::Status(taskname) => self.state.status(&taskname),
            Message::Quit => self
                .event_sender
                .send(Inter::Quit)
                .expect("cannot send quit message"),
        };
    }

    fn ev_child_has_exited(&mut self, pid: Pid, status: WaitStatus) {
        self.state.ev_child_has_exited(pid, status);
    }
}
