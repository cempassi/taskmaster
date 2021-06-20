use signal_hook::{
    consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM},
    iterator::Signals,
};
use std::sync::mpsc::Sender;

use super::inter::Inter;
use crate::error;

fn sigstr(signum: i32) -> &'static str {
    match signum {
        SIGINT => "SIGINT",
        SIGTERM => "SIGTERM",
        SIGHUP => "SIGHUP",
        SIGQUIT => "SIGQUIT",
        _ => "unknown",
    }
}
pub fn handle_signals(sender: Sender<Inter>) -> Result<(), error::Taskmaster> {
    let mut watching_signals = match Signals::new(&[SIGHUP, SIGINT, SIGTERM, SIGQUIT]) {
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
                    log::debug!("received {:?}, sending Quit message", sig);
                    sender.send(Inter::Quit).unwrap()
                }
                SIGTERM => {
                    log::debug!("received SIGTERM, sending Quit message");
                    sender.send(Inter::Quit).unwrap()
                }
                SIGQUIT => {
                    log::debug!("received SIGQUIT, sending Quit message");
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

#[cfg(test)]
mod test_signal {
    use super::sigstr;
    use signal_hook::consts::{SIGCONT, SIGHUP, SIGINT, SIGQUIT, SIGTERM};

    #[test]
    fn test_sigstr() {
        assert_eq!(sigstr(SIGTERM), "SIGTERM");
        assert_eq!(sigstr(SIGQUIT), "SIGQUIT");
        assert_eq!(sigstr(SIGINT), "SIGINT");
        assert_eq!(sigstr(SIGHUP), "SIGHUP");
        assert_eq!(sigstr(SIGCONT), "unknown");
    }
}
