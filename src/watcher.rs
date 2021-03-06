use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use crate::error::TaskmasterError;
use crate::server::Message;

#[derive(Clone)]
struct PathData {
    mtime: SystemTime,
    last_check: Option<Instant>,
}

#[derive(Clone)]
pub struct Watcher {
    pub path: PathBuf,
    pub sender: Option<Sender<Message>>,
    data: PathData,
}

impl TryFrom<&str> for Watcher {
    type Error = TaskmasterError;

    fn try_from(p: &str) -> Result<Self, TaskmasterError> {
        let path = PathBuf::from(p);

        let watcher = Self {
            path,
            sender: None,
            data: PathData {
                mtime: SystemTime::now(),
                last_check: None,
            },
        };
        Ok(watcher)
    }
}

impl Watcher {
    pub fn run(&mut self, sender: Sender<Message>) {
        let path = self.path.clone();
        let mut data = self.data.clone();
        self.sender = Some(sender.clone());
        thread::spawn(move || {
        loop {
            let delay: Duration = Duration::from_secs(10);
            if path.is_file() {
                match path.metadata() {
                    Err(_) => {
                        println!("Can't Access metadata");
                    }
                    Ok(metadata) => {
                        let mtime = metadata.modified().unwrap();
                        if mtime != data.mtime {
                            println!("Send signal to reload config");
                            data.mtime = mtime;
                            sender.send(Message::Reload).unwrap();
                        } else {
                            println!("Nothing to be done");
                        }
                    }
                }
            } else {
                println!("File has been ereased");
            }
            thread::sleep(delay);
        }});
    }

    pub fn send(&self, msg: Message) {
        if let Some(sender) = &self.sender {
            sender.send(msg).unwrap();
        }
    }
}
