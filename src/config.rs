use std::path::PathBuf;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::error::TaskmasterError;
use crate::reader::ConfigFile;
use crate::task::Task;

#[derive(Debug)]
pub struct Config {
    pub tasks: HashMap<String, Task>,
}

impl TryFrom<&PathBuf> for Config {
    type Error = TaskmasterError;

    fn try_from(path: &PathBuf) -> Result<Self, TaskmasterError> {
        let configfile: ConfigFile = ConfigFile::try_from(path)?;

        let mut tasks: HashMap<String, Task> = HashMap::new();
        for task in configfile.task {
            tasks.insert(task.name.clone(), Task::try_from(task)?);
        }
        Ok(Config {tasks})
    }
}
