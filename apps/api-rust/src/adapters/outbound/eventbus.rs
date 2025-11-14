use anyhow::Result;
use async_trait::async_trait;
use std::any::Any;
use tracing::debug;
use crate::ports::EventBus;

pub struct NopBus;

impl NopBus {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl EventBus for NopBus {
    async fn publish(&self, topic: &str, _payload: Box<dyn Any + Send>) -> Result<()> {
        debug!(topic, "eventbus(nop): publish");
        Ok(())
    }
}