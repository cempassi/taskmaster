use log::{set_boxed_logger, LevelFilter, Metadata, Record, SetLoggerError};
use std::time::Instant;

pub struct Simple {
    pub level: LevelFilter,
    now: Option<Instant>,
}

impl Simple {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        set_boxed_logger(Simple::new(level))
    }

    pub fn new(level: LevelFilter) -> Box<Self> {
        Box::new(Self { level, now: None })
    }

    pub fn set_instant(&mut self, instant: Instant) {
        self.now = Some(instant);
    }
}

impl log::Log for Simple {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{:>5}[{:04}] {}",
                record.level(),
                self.now.map_or(0, |time| time.elapsed().as_secs()),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
