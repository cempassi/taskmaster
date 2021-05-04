use log::{Level, Metadata, Record};

pub struct Simple {
    pub level: Level,
}

impl log::Log for Simple {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args())
        }
    }

    fn flush(&self) {}
}
