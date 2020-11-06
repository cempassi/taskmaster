use serde::Deserialize;
use std::convert::From;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::signal::Signal;
use crate::error::TaskmasterError;


#[derive(Debug, Deserialize)]
pub struct ReadConfig {
    numprocess: i32,
    umask: i32,
    stopsignal: String,
    workingdir: String,
}

impl ReadConfig {
    pub fn new(path: &str) -> Result<Self, TaskmasterError> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_e) => return Err(TaskmasterError::ReadFile),
        };

        match toml::from_str(&content) {
            Ok(c) => Ok(c),
            Err(_e) => Err(TaskmasterError::Parse),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    numprocess: i32,
    umask: i32,
    stopsignal: Signal,
    workingdir: PathBuf,
}

impl FromStr for Config {
    type Err = TaskmasterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config: Config = ReadConfig::new(s)?.into();
        dbg!("config: {:?}", &config);
        Ok(config)
    }
}

impl From<ReadConfig> for Config {
    fn from(readconf: ReadConfig) -> Self {
        let signal: Result<Signal, TaskmasterError> =
            Signal::from_str(readconf.stopsignal.as_str());

        Config {
            numprocess: readconf.numprocess,
            umask: readconf.umask,
            stopsignal: signal.unwrap(),
            workingdir: PathBuf::from(readconf.workingdir),
        }
    }
}

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
