use serde::Deserialize;
use std::fs;
use std::convert::From;

#[derive(Debug)]
pub enum Signal {
    SIGTERM,
    SIGSEGV
}

#[derive(Deserialize)]
struct ReadConfig {
    numprocess: i32,
    umask: i32,
    stopsignal: String,
    workingdir: String
}
#[derive(Debug)]
pub struct Config {
    numprocess: i32,
    umask: i32,
    stopsignal: Signal,
    workingdir: String
}

impl From<ReadConfig> for Config {
    fn from(readconf: ReadConfig) -> Self {
        
        let signal: Option<Signal> = match readconf.stopsignal.as_str(){
             "TERM" => Some(Signal::SIGTERM),
             "SEGV" => Some(Signal::SIGSEGV),
             &_ => None
        };

        Config {numprocess: readconf.numprocess,
                umask: readconf.umask,
                stopsignal: signal.unwrap(),
                workingdir: readconf.workingdir
        }
    }
}

fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("./task.toml").unwrap();
    let readconfig: ReadConfig = toml::from_str(&content).unwrap();

    println!(
        "readconfig.numprocess: {}, readconfig.umask: {}, readconfig.stopsignal: {:?}, readconfig.wokingdir: {:?}",
        readconfig.numprocess, readconfig.umask, readconfig.stopsignal, readconfig.workingdir
    );

    let config: Config = readconfig.into();
    
    println!(
        "config.numprocess: {}, config.umask: {}, config.stopsignal: {:?}, config.wokingdir: {:?}",
        config.numprocess, config.umask, config.stopsignal, config.workingdir
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
