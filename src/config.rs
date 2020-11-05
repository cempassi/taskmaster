use serde::Deserialize;
use std::convert::From;

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

#[derive(Debug, Deserialize)]
pub struct ReadConfig {
    numprocess: i32,
    umask: i32,
    stopsignal: String,
    workingdir: String,
}

#[derive(Debug)]
pub struct Config {
    numprocess: i32,
    umask: i32,
    stopsignal: Signal,
    workingdir: String,
}

impl From<ReadConfig> for Config {
    fn from(readconf: ReadConfig) -> Self {
        let signal: Option<Signal> = match readconf.stopsignal.as_str() {
            "HUP" => Some(Signal::SIGHUP),
            "INT" => Some(Signal::SIGINT),
            "QUIT" => Some(Signal::SIGQUIT),
            "ILL" => Some(Signal::SIGILL),
            "TRAP" => Some(Signal::SIGTRAP),
            "ABRT" => Some(Signal::SIGABRT),
            "EMT" => Some(Signal::SIGEMT),
            "FPE" => Some(Signal::SIGFPE),
            "KILL" => Some(Signal::SIGKILL),
            "BUS" => Some(Signal::SIGBUS),
            "SEGV" => Some(Signal::SIGSEGV),
            "SYS" => Some(Signal::SIGSYS),
            "PIPE" => Some(Signal::SIGPIPE),
            "ALRM" => Some(Signal::SIGALRM),
            "TERM" => Some(Signal::SIGTERM),
            "URG" => Some(Signal::SIGURG),
            "STOP" => Some(Signal::SIGSTOP),
            "TSTP" => Some(Signal::SIGTSTP),
            "CONT" => Some(Signal::SIGCONT),
            "CHLD" => Some(Signal::SIGCHLD),
            "TTIN" => Some(Signal::SIGTTIN),
            "TTOU" => Some(Signal::SIGTTOU),
            "IO" => Some(Signal::SIGIO),
            "XCPU" => Some(Signal::SIGXCPU),
            "XFSZ" => Some(Signal::SIGXFSZ),
            "VTAL" => Some(Signal::SIGVTAL),
            "PROF" => Some(Signal::SIGPROF),
            "WINC" => Some(Signal::SIGWINC),
            "INFO" => Some(Signal::SIGINFO),
            "USR1" => Some(Signal::SIGUSR1),
            "USR2" => Some(Signal::SIGUSR2),
            &_ => None,
        };

        Config {
            numprocess: readconf.numprocess,
            umask: readconf.umask,
            stopsignal: signal.unwrap(),
            workingdir: readconf.workingdir,
        }
    }
}

//Status d'un job
//enum State {
//	RUNNING,
//	STOPPED,
//	EXITED,
//	KILLED
//}
//
//// Enum des instructions à envoyer sur le task
//enum Instruction {
//	START,
//	RESTART,
//	STOP,
//	STATUS,
//	SHUTDOWN
//}
//
//
//// Un enum pour les erreurs pas mal pour la gestion et centraliser les messages
//enum Error {
//	InvalidCmd
//}
//
//// Structure du gestionnaire de job control avec le fichier de conf
//struct taskmaster {
//	configFile: str,
//
//}
//
//// Structure de job avec la commande, le status, l'option d autorestart et le starttime (à completer)
//struct Job {
//	cmd: Cmd,
//	state: State,
//	autorestart: bool,
//	starttime: temps(),
//}
