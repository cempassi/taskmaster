use std::time;

pub struct Config {
    pub instant: Option<time::Instant>,
}

impl Config {
    pub fn new(instant: Option<time::Instant>) -> Self {
        Self { instant }
    }
}
