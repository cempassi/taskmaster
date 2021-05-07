use super::relaunch::Relaunch;
use libc::mode_t;

pub const AUTOSTART: bool = false;
pub const NUMPROCESS: u32 = 1;
pub const UMASK: mode_t = 0;
pub const WORKDIR: &str = ".";
pub const RELAUNCH_MODE: Relaunch = Relaunch::Never;
pub const RETRY: u32 = 0;
pub const EXPECTED_EXIT_CODES: [i32; 1] = [0];
pub const SUCCESS_DELAY: u32 = 0;
pub const STOP_SIGNAL: &str = "TERM";
pub const STOP_DELAY: u32 = 2;
pub const STDOUT: &str = "/dev/null";
pub const STDERR: &str = "/dev/null";
pub const ENV: [String; 0] = [];
