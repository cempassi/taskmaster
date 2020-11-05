use std::fs;

mod config;

use crate::config::{ReadConfig, Config};

fn main() {
    let content = fs::read_to_string("./task.toml").unwrap();
    let readconfig: ReadConfig = toml::from_str(&content).unwrap();

    println!("readconfig: {:?}", readconfig);

    let config: Config = readconfig.into();

    println!("config: {:?}", config );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
