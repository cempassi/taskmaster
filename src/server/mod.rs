use std::convert::TryFrom;
use std::sync::mpsc::channel;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

mod state;
mod watcher;
mod reader;
mod task;
mod worker;
pub mod error;

use self::state::State;
use self::watcher::Watcher;


pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    Quit
}

fn process_message(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("Recieved {}", line.unwrap());
    }
}

fn listen() {
    let listner = UnixListener::bind("/tmp/taskmaster.sock").unwrap();

    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| process_message(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}

pub fn start_server(config: &str) {
    let (sender, receiver) = channel();
    let mut state = State::new();
    let mut watcher = Watcher::try_from(config).unwrap();

    listen();
    watcher.run(sender);
    loop {
        if let Ok(message) = receiver.recv() {
            match message {
                Message::Reload => {
                    state.reload(&watcher);
                }
                Message::Start(task) => {
                    state.start(&task)
                }
                Message::Stop(task) => {
                    state.stop(&task)
                }
                Message::Status(_task) => {
                    unimplemented!();
                }
                Message::Quit => break,
            };
        };
    }
}

