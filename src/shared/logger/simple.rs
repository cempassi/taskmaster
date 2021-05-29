use super::{write_log, Config};
use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::stdout;

pub struct Logger {
    level: LevelFilter,
    config: Config,
}

impl Logger {
    pub fn init(level: LevelFilter, config: Config) -> Result<(), SetLoggerError> {
        set_max_level(level);
        set_boxed_logger(Self::new(level, config))
    }

    pub fn new(level: LevelFilter, config: Config) -> Box<Self> {
        Box::new(Self { level, config })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let std = stdout();
            let _ = write_log(&self.config, record, &mut std.lock());
        }
    }

    fn flush(&self) {}
}
