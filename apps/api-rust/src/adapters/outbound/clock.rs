use chrono::{DateTime, Utc};
use crate::ports::Clock;

pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self { Self }
}

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}