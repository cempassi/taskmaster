use super::{relaunch::Relaunch, signal::Signal};
use libc::mode_t;

pub fn autostart() -> bool {
    false
}

pub fn numprocess() -> u32 {
    1
}

pub fn umask() -> mode_t {
    0
}

pub fn workdir() -> String {
    String::from(".")
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
    Signal::Term
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

pub fn env() -> Vec<String> {
    Vec::new()
}
