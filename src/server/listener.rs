use serde::Deserialize;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use super::{communication::Communication, message::Inter, Message};

pub struct Listener {
    pub sock: UnixListener,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            sock: UnixListener::bind("/tmp/taskmaster.sock").unwrap(),
        }
    }

    pub fn run(&mut self, sender: Sender<Inter>) {
        let listener = self.sock.try_clone().unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let s = sender.clone();
                        thread::spawn(move || process_message(stream, &s));
                    }
                    Err(err) => {
                        log::error!("{}", err);
                        break;
                    }
                }
            }
        });
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        fs::remove_file("/tmp/taskmaster.sock").unwrap();
    }
}

fn process_message(stream: UnixStream, sender: &Sender<Inter>) {
    log::info!("Ready to recieve.");
    let mut response = stream.try_clone().expect("Couldn't clone socket");
    let mut de = serde_json::Deserializer::from_reader(stream);

    if let Ok(msg) = Message::deserialize(&mut de) {
        log::info!("Recieved {:?}", msg);
        let (snd, receiver) = channel();
        let com = Communication {
            message: msg,
            channel: Some(snd),
        };
        sender.send(Inter::FromClient(com)).unwrap();
        for res in receiver.iter() {
            response.write_all(res.as_bytes()).unwrap();
        }
    }
    log::info!("End of transmission.");
}
