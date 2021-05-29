use std::time;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub instant: Option<time::Instant>,
}

impl Config {
    pub fn new(instant: Option<time::Instant>) -> Self {
        Self { instant }
    }
}
