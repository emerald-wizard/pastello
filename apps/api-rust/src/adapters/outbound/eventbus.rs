use crate::ports::EventBus;
use async_trait::async_trait;
use std::any::Any;
use tokio_stream::Stream;

// --- FIX: Add pub ---
pub struct NopEventBus;

impl NopEventBus {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NopEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventBus for NopEventBus {
    async fn publish(&self, topic: &str, _payload: Box<dyn Any + Send>) -> anyhow::Result<()> {
        tracing::debug!("NopEventBus: Firing event on topic: {}", topic);
        Ok(())
    }

    async fn subscribe(
        &self,
        topic: &str,
    ) -> Box<dyn Stream<Item = Box<dyn Any + Send>> + Send + Unpin> {
        tracing::debug!("NopEventBus: Subscribing to topic: {}", topic);
        Box::new(tokio_stream::empty())
    }
}