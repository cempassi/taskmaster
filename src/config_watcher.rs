use std::convert::TryFrom;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use crate::error::TaskmasterError;

#[derive(Clone)]
struct PathData {
    mtime: SystemTime,
    last_check: Option<Instant>,
}

#[derive(Clone)]
pub struct ConfigWatcher<'a> {
    path: &'a PathBuf,
    data: PathData,
}

impl<'a> TryFrom<&'a PathBuf> for ConfigWatcher<'a> {
    type Error = TaskmasterError;

    fn try_from(path: &'a PathBuf) -> Result<Self, TaskmasterError> {
        let mut p = ConfigWatcher {
            path,
            data: PathData {
                mtime: SystemTime::now(),
                last_check: None,
            },
        };
        p.run();
        Ok(p)
    }
}

impl<'a> ConfigWatcher<'a> {
    fn run(&mut self) {
        for _ in 1..3 {
            let delay: Duration = Duration::from_secs(10);
            if self.path.is_file() {
                match self.path.metadata() {
                    Err(_) => {
                        println!("Can't Access metadata");
                    }
                    Ok(metadata) => {
                        let mtime = metadata.modified().unwrap();
                        if mtime != self.data.mtime {
                            println!("Send signal to reload config");
                            self.data.mtime = mtime;
                        } else {
                            println!("Nothing to be done");
                        }
                    }
                }
            } else {
                println!("File has been ereased");
            }
            thread::sleep(delay);
        }
    }
}
