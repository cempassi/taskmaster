use std::convert::TryFrom;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Sender, channel};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

pub mod error;
mod reader;
mod state;
mod task;
mod watcher;
mod worker;

use self::state::State;
use self::watcher::Watcher;

pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    Quit,
}

fn process_message(stream: UnixStream, sender: Sender<Message>) {
    let stream = BufReader::new(stream);
    println!("Ready to recieve.");
    for line in stream.lines() {
        println!("Recieved {}", line.unwrap());
    }
    println!("End of transmission.");
}

fn listen(sender: Sender<Message>) {
    thread::spawn(move || {
        let listner = UnixListener::bind("/tmp/taskmaster.sock").unwrap();

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

    listen(sender.clone());
    watcher.run(sender);
    loop {
        if let Ok(message) = receiver.recv() {
            match message {
                Message::Reload => {
                    state.reload(&watcher);
                }
                Message::Start(task) => state.start(&task),
                Message::Stop(task) => state.stop(&task),
                Message::Status(_task) => {
                    unimplemented!();
                }
                Message::Quit => break,
            };
        };
    }
}
