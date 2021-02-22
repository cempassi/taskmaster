use std::path::PathBuf;
use std::convert::TryFrom;
use std::thread;

mod config;
mod error;
mod reader;
mod signal;
mod task;
mod config_watcher;

use config::Config;
use config_watcher::ConfigWatcher;
use error::TaskmasterError;

type Result<T> = std::result::Result<T, TaskmasterError>;

fn task_thread(config: Config) -> Result<()> {
    for (name, task) in config.tasks.into_iter() {
        println!("Running task: {}", name);
        let child = thread::spawn(move || {
            task.run();
        });
        child.join().unwrap();
    }
    Ok(())
}

fn main() -> Result<()> {
    let config_path = PathBuf::from(r"./config.toml");
    let config: Config = Config::try_from(&config_path)?;
    let file_watcher: ConfigWatcher = ConfigWatcher::try_from(&config_path)?;
    task_thread(config)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
