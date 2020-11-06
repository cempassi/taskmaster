use std::str::FromStr;

mod config;

use crate::config::Config;

fn main(){
    let _config: Config = Config::from_str("./task.tom").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
