use std::str::FromStr;

use crate::error::TaskmasterError;

#[derive(Debug)]
pub enum Signal {
    SIGHUP,
    SIGINT,
    SIGQUIT,
    SIGILL,
    SIGTRAP,
    SIGABRT,
    SIGEMT,
    SIGFPE,
    SIGKILL,
    SIGBUS,
    SIGSEGV,
    SIGSYS,
    SIGPIPE,
    SIGALRM,
    SIGTERM,
    SIGURG,
    SIGSTOP,
    SIGTSTP,
    SIGCONT,
    SIGCHLD,
    SIGTTIN,
    SIGTTOU,
    SIGIO,
    SIGXCPU,
    SIGXFSZ,
    SIGVTAL,
    SIGPROF,
    SIGWINC,
    SIGINFO,
    SIGUSR1,
    SIGUSR2,
}

impl FromStr for Signal {
    type Err = TaskmasterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HUP" => Ok(Signal::SIGHUP),
            "INT" => Ok(Signal::SIGINT),
            "QUIT" => Ok(Signal::SIGQUIT),
            "ILL" => Ok(Signal::SIGILL),
            "TRAP" => Ok(Signal::SIGTRAP),
            "ABRT" => Ok(Signal::SIGABRT),
            "EMT" => Ok(Signal::SIGEMT),
            "FPE" => Ok(Signal::SIGFPE),
            "KILL" => Ok(Signal::SIGKILL),
            "BUS" => Ok(Signal::SIGBUS),
            "SEGV" => Ok(Signal::SIGSEGV),
            "SYS" => Ok(Signal::SIGSYS),
            "PIPE" => Ok(Signal::SIGPIPE),
            "ALRM" => Ok(Signal::SIGALRM),
            "TERM" => Ok(Signal::SIGTERM),
            "URG" => Ok(Signal::SIGURG),
            "STOP" => Ok(Signal::SIGSTOP),
            "TSTP" => Ok(Signal::SIGTSTP),
            "CONT" => Ok(Signal::SIGCONT),
            "CHLD" => Ok(Signal::SIGCHLD),
            "TTIN" => Ok(Signal::SIGTTIN),
            "TTOU" => Ok(Signal::SIGTTOU),
            "IO" => Ok(Signal::SIGIO),
            "XCPU" => Ok(Signal::SIGXCPU),
            "XFSZ" => Ok(Signal::SIGXFSZ),
            "VTAL" => Ok(Signal::SIGVTAL),
            "PROF" => Ok(Signal::SIGPROF),
            "WINC" => Ok(Signal::SIGWINC),
            "INFO" => Ok(Signal::SIGINFO),
            "USR1" => Ok(Signal::SIGUSR1),
            "USR2" => Ok(Signal::SIGUSR2),
            &_ => Err(TaskmasterError::Signal),
        }
    }
}
