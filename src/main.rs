use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct ReadConfig {
    numprocess: i32,
    umask: i32,
    stopsignal: String,
}

fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("./task.toml").unwrap();
    let config: ReadConfig = toml::from_str(&content).unwrap();

    println!(
        "config.numprocess: {}, config.umask: {}, config.stopsignal: {:?}",
        config.numprocess, config.umask, config.stopsignal
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
