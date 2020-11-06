use std::str::FromStr;

mod config;
mod task;
mod signal;
mod error;

use crate::task::Task;
use crate::config::Config;

fn main(){
    let _config: Config = Config::from_str("./task.toml").unwrap();
    let task: Task = Task::from_str("./ls.toml").unwrap();
    task.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
