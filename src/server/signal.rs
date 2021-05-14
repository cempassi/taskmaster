use serde::{Deserialize, Serialize};
use signal_hook::{
    consts::{SIGHUP, SIGINT},
    iterator::Signals,
};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::sync::mpsc::Sender;

use super::{communication::Communication, Message};
use crate::error;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Signal {
    Hup,
    Int,
    Quit,
    Ill,
    Trap,
    Abrt,
    Emt,
    Fpe,
    Kill,
    Bus,
    Segv,
    Sys,
    Pipe,
    Alrm,
    Term,
    Urg,
    Stop,
    Tstp,
    Cont,
    Chld,
    Ttin,
    Ttou,
    Io,
    Xcpu,
    Xfsz,
    Vtal,
    Prof,
    Winc,
    Info,
    Usr1,
    Usr2,
}

impl FromStr for Signal {
    type Err = error::Taskmaster;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HUP" => Ok(Signal::Hup),
            "INT" => Ok(Signal::Int),
            "QUIT" => Ok(Signal::Quit),
            "ILL" => Ok(Signal::Ill),
            "TRAP" => Ok(Signal::Trap),
            "ABRT" => Ok(Signal::Abrt),
            "EMT" => Ok(Signal::Emt),
            "FPE" => Ok(Signal::Fpe),
            "KILL" => Ok(Signal::Kill),
            "BUS" => Ok(Signal::Bus),
            "SEGV" => Ok(Signal::Segv),
            "SYS" => Ok(Signal::Sys),
            "PIPE" => Ok(Signal::Pipe),
            "ALRM" => Ok(Signal::Alrm),
            "TERM" => Ok(Signal::Term),
            "URG" => Ok(Signal::Urg),
            "STOP" => Ok(Signal::Stop),
            "TSTP" => Ok(Signal::Tstp),
            "CONT" => Ok(Signal::Cont),
            "CHLD" => Ok(Signal::Chld),
            "TTIN" => Ok(Signal::Ttin),
            "TTOU" => Ok(Signal::Ttou),
            "IO" => Ok(Signal::Io),
            "XCPU" => Ok(Signal::Xcpu),
            "XFSZ" => Ok(Signal::Xfsz),
            "VTAL" => Ok(Signal::Vtal),
            "PROF" => Ok(Signal::Prof),
            "WINC" => Ok(Signal::Winc),
            "INFO" => Ok(Signal::Info),
            "USR1" => Ok(Signal::Usr1),
            "USR2" => Ok(Signal::Usr2),
            &_ => Err(error::Taskmaster::Signal),
        }
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Signal::Hup => "HUP",
            Signal::Int => "INT",
            Signal::Quit => "QUIT",
            Signal::Ill => "ILL",
            Signal::Trap => "TRAP",
            Signal::Abrt => "ABRT",
            Signal::Emt => "EMT",
            Signal::Fpe => "FPE",
            Signal::Kill => "KILL",
            Signal::Bus => "BUS",
            Signal::Segv => "SEGV",
            Signal::Sys => "SYS",
            Signal::Pipe => "PIPE",
            Signal::Alrm => "ALRM",
            Signal::Term => "TERM",
            Signal::Urg => "URG",
            Signal::Stop => "STOP",
            Signal::Tstp => "TSTP",
            Signal::Cont => "CONT",
            Signal::Chld => "CHLD",
            Signal::Ttin => "TTIN",
            Signal::Ttou => "TTOU",
            Signal::Io => "IO",
            Signal::Xcpu => "XCPU",
            Signal::Xfsz => "XFSZ",
            Signal::Vtal => "VTAL",
            Signal::Prof => "PROF",
            Signal::Winc => "WINC",
            Signal::Info => "INFO",
            Signal::Usr1 => "USR1",
            Signal::Usr2 => "USR2",
        };
        write!(f, "{}", s)
    }
}

pub fn handle_signals(sender: Sender<Communication>) -> Result<(), error::Taskmaster> {
    let mut watching_signals = match Signals::new(&[SIGHUP, SIGINT]) {
        Ok(c) => c,
        Err(e) => return Err(error::Taskmaster::Io(e)),
    };
    std::thread::spawn(move || {
        for sig in watching_signals.forever() {
            match sig {
                SIGHUP => {
                    log::debug!("received SIGHUP, send Reload message");
                    sender
                        .send(Communication::new(Message::Reload, None))
                        .unwrap()
                }
                SIGINT => {
                    log::debug!("received SIGINT, sending Quit message");
                    sender
                        .send(Communication::new(Message::Quit, None))
                        .unwrap()
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
