use super::relaunch::Relaunch;
use nix::sys::{signal::Signal, stat::Mode};
use std::{collections::BTreeMap, path::PathBuf};

pub fn autostart() -> bool {
    false
}

pub fn numprocess() -> u32 {
    1
}

pub fn umask() -> Mode {
    Mode::from_bits(0).expect("cannot create default mode")
}

pub fn workdir() -> PathBuf {
    PathBuf::from(".")
}

pub fn relaunch_mode() -> Relaunch {
    Relaunch::Never
}

pub fn retry() -> u32 {
    0
}

pub fn exit_codes() -> Vec<i32> {
    vec![0]
}

pub fn success_delay() -> u32 {
    0
}

pub fn stop_signal() -> Signal {
    Signal::SIGTERM
}

pub fn stop_delay() -> u32 {
    2
}

pub fn stdout() -> String {
    String::from("/dev/null")
}

pub fn stderr() -> String {
    String::from("/dev/null")
}

pub fn env() -> BTreeMap<String, String> {
    BTreeMap::new()
}
