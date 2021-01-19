use std::str::FromStr;
use std::thread;

mod config;
mod error;
mod reader;
mod signal;
mod task;

use error::TaskmasterError;
use config::Config;

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
        println!("Running task: {}", name);
        let child = thread::spawn(move || {
            task.run();
        });
        child.join().unwrap();
    }
    Ok(())
}

fn main() -> Result<()> {
    let config: Config = Config::from_str("./config.toml")?;
    task_thread(config)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
