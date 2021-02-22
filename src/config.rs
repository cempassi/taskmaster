use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::error::TaskmasterError;
use crate::reader::ConfigFile;
use crate::task::Task;

#[derive(Debug)]
pub struct Config {
    pub tasks: HashMap<String, Task>,
}

impl FromStr for Config {
    type Err = TaskmasterError;

    fn from_str(path: &str) -> Result<Self, TaskmasterError> {
        let configfile: ConfigFile = ConfigFile::from_str(path)?;

        let mut tasks: HashMap<String, Task> = HashMap::new();
        for task in configfile.task {
            tasks.insert(task.name.clone(), Task::try_from(task).unwrap());
        }
        Ok(Config {tasks})
    }
}
