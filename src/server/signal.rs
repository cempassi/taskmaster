use signal_hook::{
    consts::{SIGHUP, SIGINT},
    iterator::Signals,
};
use std::sync::mpsc::Sender;

use super::message::Inter;
use crate::error;

pub fn handle_signals(sender: Sender<Inter>) -> Result<(), error::Taskmaster> {
    let mut watching_signals = match Signals::new(&[SIGHUP, SIGINT]) {
        Ok(c) => c,
        Err(e) => return Err(error::Taskmaster::Io(e)),
    };
    std::thread::spawn(move || {
        for sig in watching_signals.forever() {
            match sig {
                SIGHUP => {
                    log::debug!("received SIGHUP, send Reload message");
                    sender.send(Inter::Reload).unwrap()
                }
                SIGINT => {
                    log::debug!("received SIGINT, sending Quit message");
                    sender.send(Inter::Quit).unwrap()
                }
                _ => {
                    log::error!("unhandled signal value {}", sig);
                    unreachable!()
                }
            };
        }
    });
    Ok(())
}
