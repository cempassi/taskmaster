use std::{
    convert::TryFrom,
    marker,
    str::FromStr,
    sync::mpsc::{channel, Sender},
};

mod communication;
mod default;
pub mod error;
mod formatter;
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

use self::{
    communication::Com,
    formatter::{Formatter, Human, Json, MessageFormat, Yaml},
    inter::Inter,
    listener::Listener,
    state::State,
    watcher::Watcher,
};

struct Server<F>
where
    F: Formatter,
{
    state: State<F>,
    event: Sender<Inter>,
    _marker: marker::PhantomData<F>,
}

pub fn start(config: &str, format: &str) -> Result<(), error::Taskmaster> {
    log::info!(
        "starting server with config at {} and format {}",
        config,
        format
    );
    let format = MessageFormat::from_str(format).unwrap();
    match format {
        MessageFormat::Human => start_raw::<Human>(config),
        MessageFormat::Yaml => start_raw::<Yaml>(config),
        MessageFormat::Json => start_raw::<Json>(config),
    }
}

pub fn start_raw<F: Formatter>(config: &str) -> Result<(), error::Taskmaster> {
    let (sender, event) = channel::<Inter>();
    let (response, receiver) = channel::<Com>();

    let mut watcher = Watcher::try_from(config)?;
    let mut listener = Listener::new();
    let mut server = Server {
        state: State::<F>::new(sender.clone(), response.clone()),
        event: sender.clone(),
        _marker: marker::PhantomData,
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

impl<F: Formatter> Server<F> {
    fn reload_config(&mut self, watcher: &Watcher) {
        self.state.reload(watcher)
    }

    fn handle_client_message(&mut self, message: Message) {
        match message {
            Message::Reload => self.event.send(Inter::Reload).unwrap(),
            Message::Start(taskname) => self.state.start(&taskname),
            Message::Info(taskname) => self.state.info(&taskname),
            Message::Stop(taskname) => self.state.stop(&taskname),
            Message::List => self.state.list(),
            Message::Status(taskname) => self.state.status(&taskname),
            Message::Restart(taskname) => {
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
