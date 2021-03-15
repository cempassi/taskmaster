use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::convert::TryFrom;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub mod error;
mod reader;
mod state;
mod task;
mod watcher;
mod worker;

use self::state::State;
use self::watcher::Watcher;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    List,
    Quit,
}

fn process_message(stream: UnixStream, sender: Sender<Message>) {
    println!("Ready to recieve.");
    let mut response = stream.try_clone().expect("Couldn't clone socket");
    let mut de = serde_json::Deserializer::from_reader(stream);

    if let Ok(msg) = Message::deserialize(&mut de) {
        println!("Recieved {:?}", msg);
        sender.send(msg).unwrap();
        response.write_all(b"Ok").unwrap();
    }
    println!("End of transmission.");
}

fn listen(listner: UnixListener, sender: Sender<Message>) {
    thread::spawn(move || {
        for stream in listner.incoming() {
            match stream {
                Ok(stream) => {
                    let s = sender.clone();
                    thread::spawn(move || process_message(stream, s));
                }
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            }
        }
    });
}

pub fn start_server(config: &str) {
    let (sender, receiver) = channel();
    let mut state = State::new();
    let mut watcher = Watcher::try_from(config).unwrap();
    let listner = UnixListener::bind("/tmp/taskmaster.sock").unwrap();

    listen(listner, sender.clone());
    watcher.run(sender);
    loop {
        if let Ok(message) = receiver.recv() {
            match message {
                Message::Reload => state.reload(&watcher),
                Message::Start(task) => state.start(&task),
                Message::Stop(task) => state.stop(&task),
                Message::List => state.list(),
                Message::Status(_task) => {
                    unimplemented!();
                }
                Message::Quit => break,
            };
        };
    }
}
