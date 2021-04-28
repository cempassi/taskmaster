use super::relaunch::Relaunch;

pub const AUTOSTART: bool = false;
pub const NUMPROCESS: u32 = 1;
pub const UMASK: u16 = 0;
pub const WORKDIR: &str = ".";
pub const RELAUNCH_MODE: Relaunch = Relaunch::Never;
pub const RETRY: u32 = 0;
pub static EXPECTED_EXIT_CODES: Vec<i32> = Vec::<i32>::new();
pub const SUCCESS_DELAY: u32 = 0;
pub const STOP_SIGNAL: &str = "QUIT";
pub const STOP_DELAY: u32 = 2;
pub const STDOUT: &str = "/dev/null";
pub const STDERR: &str = "/dev/null";
// const ENV // TODO
