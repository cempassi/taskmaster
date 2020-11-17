use std::convert::TryFrom;
use std::str::FromStr;

mod config;
mod error;
mod reader;
mod signal;
mod task;

use error::TaskmasterError;
use config::Config;
use reader::ConfigFile;
use task::Task;

type Result<T> = std::result::Result<T, TaskmasterError>;

fn main() -> Result<()> {
    let configfile: ConfigFile = ConfigFile::from_str("./config.toml")?;
    let _config: Config = Config::try_from(&configfile.config)?;
    let tasks: Vec<Task> = configfile
        .tasks
        .into_iter()
        .map(|readtask| Task::try_from(&readtask).unwrap())
        .collect();

    for task in &tasks {
        task.run();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
