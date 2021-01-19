use std::convert::TryFrom;
use std::str::FromStr;
use std::thread;

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

#[derive(Debug)]
enum State {
    Running,
    Stopped,
    Killed,
    Finished,
    Init
}

pub struct Process {
    pid: i32
}

pub struct Task_monitoring {
    state: State,
    processes: Vec<Process>
}

fn task_thread(config: Config) -> Result<()>{
    for (name, task) in config.tasks.into_iter() {
        println!("{}", name);
        let child = thread::spawn(move || {
            task.run();
        });
    }
    Ok(())
}

fn main() -> Result<()> {
    let configfile: ConfigFile = ConfigFile::from_str("./config.toml")?;
    let config: Config = Config::try_from(&configfile)?;
    task_thread(config);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
