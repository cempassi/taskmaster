use log::{Level, Metadata, Record};
use std::time::Instant;

pub static mut LOGGER: Simple = Simple {
    level: Level::Debug,
    now: None,
};

pub struct Simple {
    pub level: Level,
    now: Option<Instant>,
}

impl Simple {
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
            )
        }
    }

    fn flush(&self) {}
}
