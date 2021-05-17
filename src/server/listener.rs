use std::sync::{Arc,Mutex};
use serde::Deserialize;
use std::fs;
use std::io::prelude::*;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use super::{communication::Com, inter::Inter};
use crate::shared::message::Message;

pub struct Listener {
    pub sock: UnixListener,

}

impl Listener {
    pub fn new() -> Self {
        Self {
            sock: UnixListener::bind("/tmp/taskmaster.sock").unwrap(),
        }
    }

    pub fn run(&mut self, sender: Sender<Inter>, receiver: Receiver<Com>) {
        let listener = self.sock.try_clone().unwrap();
        let reference = Arc::new(Mutex::new(receiver));
        thread::spawn(move || {
            for stream in listener.incoming() {
                let copy = reference.clone();
                match stream {
                    Ok(stream) => {
                        let s = sender.clone();
                        thread::spawn(move || process_message(stream, &s, &copy));
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

fn process_message(stream: UnixStream, sender: &Sender<Inter>, receiver: &Arc<Mutex<Receiver<Com>>>) {
    log::info!("Ready to recieve.");
    let mut response = stream.try_clone().expect("Couldn't clone socket");
    let mut de = serde_json::Deserializer::from_reader(stream);
    let re = receiver.lock().unwrap();

    if let Ok(msg) = Message::deserialize(&mut de) {
        log::info!("Recieved {:?}", msg);
        sender.send(Inter::FromClient(msg)).unwrap();
        for res in re.iter() {
            match res {
                Com::Msg(data) => response.write_all(data.as_bytes()).unwrap(),
                Com::End => break
            }
        }
    }
    log::info!("End of transmission.");
}
