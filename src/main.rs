use std::convert::TryFrom;
use std::sync::mpsc::channel;

mod state;
mod watcher;
mod error;
mod reader;
mod signal;
mod task;
mod worker;

use state::State;
use watcher::Watcher;
use error::TaskmasterError;

type Result<T> = std::result::Result<T, TaskmasterError>;

pub enum Message {
    Reload,
    Start(String),
    Stop(String),
    Status(String),
    Quit
}

fn main() -> Result<()> {
    let (sender, receiver) = channel();
    let mut state = State::new();
    let mut watcher = Watcher::try_from(r"config.toml")?;

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
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
