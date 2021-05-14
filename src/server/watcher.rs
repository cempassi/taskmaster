use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use super::error;
use super::{communication::Communication, Message};

#[derive(Clone)]
struct PathData {
    mtime: SystemTime,
    last_check: Option<Instant>,
}

#[derive(Clone)]
pub struct Watcher {
    pub path: PathBuf,
    pub sender: Option<Sender<Communication>>,
    data: PathData,
}

impl TryFrom<&str> for Watcher {
    type Error = error::Taskmaster;

    fn try_from(p: &str) -> Result<Self, error::Taskmaster> {
        let path = PathBuf::from(p);

        if path.exists() {
            let watcher = Self {
                path,
                sender: None,
                data: PathData {
                    mtime: SystemTime::now(),
                    last_check: None,
                },
            };
            Ok(watcher)
        } else {
            Err(error::Taskmaster::InvalidConf)
        }
    }
}

impl Watcher {
    pub fn run(&mut self, sender: Sender<Communication>) {
        let path = self.path.clone();
        let mut data = self.data.clone();
        self.sender = Some(sender.clone());
        thread::spawn(move || loop {
            let delay: Duration = Duration::from_secs(10);
            if path.is_file() {
                match path.metadata() {
                    Err(_) => {
                        log::error!("can't Access metadata");
                    }
                    Ok(metadata) => {
                        let mtime = metadata.modified().unwrap();
                        if mtime == data.mtime {
                            log::debug!("Nothing to be done");
                        } else {
                            log::info!("ask to reload config");
                            data.mtime = mtime;
                            let com = Communication {
                                message: Message::Reload,
                                channel: None,
                            };
                            sender.send(com).unwrap();
                        }
                    }
                }
            } else {
                log::error!(
                    "{} is not a file / not exist",
                    path.clone().into_os_string().into_string().unwrap()
                );
            }
            thread::sleep(delay);
        });
    }

    pub fn send(&self, msg: Message) {
        if let Some(sender) = &self.sender {
            let com = Communication {
                message: msg,
                channel: None,
            };
            sender.send(com).unwrap();
        }
    }
}
