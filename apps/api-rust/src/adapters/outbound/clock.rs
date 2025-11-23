use crate::ports::Clock;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub struct SystemClock;

impl SystemClock {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Clock for SystemClock {
    // FIX: Corrected method name from `now` to `now_utc` (E0407)
    fn now_utc(&self) -> DateTime<Utc> {
        Utc::now()
    }
}