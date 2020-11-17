use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::TaskmasterError;
use crate::signal::Signal;
use crate::reader::ReadConfig;

#[derive(Debug)]
pub struct Config {
    numprocess: i32,
    umask: i32,
    stopsignal: Signal,
    workingdir: PathBuf,
}

impl TryFrom<&ReadConfig> for Config {
    type Error = TaskmasterError;

    fn try_from(readconf: &ReadConfig) -> Result<Self, Self::Error> {
        Ok(Config {
            numprocess: readconf.numprocess,
            umask: readconf.umask,
            stopsignal: Signal::from_str(readconf.stopsignal.as_str())?,
            workingdir: PathBuf::from(readconf.workingdir.as_str()),
        })
    }
}
