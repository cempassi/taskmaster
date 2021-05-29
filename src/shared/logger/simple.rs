use super::{config::Config, writer::write_log};
use log::{set_boxed_logger, set_max_level, LevelFilter, Metadata, Record, SetLoggerError};
use std::io::stdout;

pub struct Simple {
    pub level: LevelFilter,
    config: Config,
}

impl Simple {
    pub fn init(level: LevelFilter, config: Config) -> Result<(), SetLoggerError> {
        set_max_level(level);
        set_boxed_logger(Simple::new(level, config))
    }

    pub fn new(level: LevelFilter, config: Config) -> Box<Self> {
        Box::new(Self { level, config })
    }
}

impl log::Log for Simple {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut std = stdout();
            let _ = write_log(&self.config, record, &mut std);
        }
    }

    fn flush(&self) {}
}
