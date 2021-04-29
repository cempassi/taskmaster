use std::str::FromStr;

use crate::error;

#[derive(Debug, PartialEq, Eq)]
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
