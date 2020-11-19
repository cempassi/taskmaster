use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;
use std::collections::HashMap;

use crate::error::TaskmasterError;
use crate::signal::Signal;
use crate::reader::ConfigFile;
use crate::task::Task;

#[derive(Debug)]
pub struct Config {
    pub tasks: HashMap<String, Task>
}

impl TryFrom<&ConfigFile> for Config {
    type Error = TaskmasterError;

    fn try_from(configFile: &ConfigFile) -> Result<Self, Self::Error> {
        let mut tasks: HashMap<String, Task> = HashMap::new();
        for task in &configFile.tasks {
            tasks.insert(task.name.clone(), Task::try_from(task).unwrap());
        }
        Ok(Config {
            tasks:tasks
        })
    }
}
