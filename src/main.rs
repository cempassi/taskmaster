use std::convert::TryFrom;
use std::sync::mpsc::channel;

mod config;
mod watcher;
mod error;
mod reader;
mod signal;
mod task;
//mod worker;

use config::Config;
use watcher::Watcher;
use error::TaskmasterError;

type Result<T> = std::result::Result<T, TaskmasterError>;

pub enum Message {
    Reload,
    Launch(String),
    Stop
}

fn main() -> Result<()> {
    let (sender, receiver) = channel();
    let mut config = Config::new();
    let mut watcher = Watcher::try_from(r"config.toml")?;

    watcher.run(sender);
    loop {
        match receiver.recv() {
            Ok(message) => match message {
                Message::Reload => {
                    config.reload(&watcher);
                }
                Message::Launch(_task) => {
                    unimplemented!();
                    //if let Some(to_run) = config.tasks.get(&task) {
                    //}
                }
                Message::Stop => break,
            },
            Err(_) => (),
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
