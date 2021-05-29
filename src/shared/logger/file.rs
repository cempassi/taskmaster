use super::{write_log, Config};
use log::{set_boxed_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

pub struct Logger {
    level: LevelFilter,
    config: Config,
    file: Mutex<File>,
}

impl Logger {
    pub fn init(level: LevelFilter, config: Config, file: File) -> Result<(), SetLoggerError> {
        set_max_level(level);
        set_boxed_logger(Self::new(level, config, file))
    }

    pub fn new(level: LevelFilter, config: Config, file: File) -> Box<Self> {
        Box::new(Self {
            level,
            config,
            file: Mutex::new(file),
        })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.lock().unwrap();
            let _ = write_log(&self.config, record, &mut *file);
        }
    }

    fn flush(&self) {
        let _ = self.file.lock().unwrap().flush();
    }
}
