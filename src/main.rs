use std::fs;

mod config;

use crate::config::{ReadConfig, Config};

fn main(){
    
    let readconfig: ReadConfig = ReadConfig::new("./task.toml").unwrap();

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
