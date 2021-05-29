pub mod config;
pub mod file;
pub mod simple;
mod writer;

pub use config::Config;
use writer::write_log;
