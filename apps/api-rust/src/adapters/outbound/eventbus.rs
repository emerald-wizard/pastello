use crate::ports::EventBus;
use anyhow::Result;
use std::any::Any;
// Removed unused imports (async_trait, tokio_stream)

#[derive(Debug, Clone)]
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

impl EventBus for NopEventBus {
    // FIX: Corrected method signature to match trait (E0050)
    fn publish(&self, _event: Box<dyn Any + Send>) -> Result<()> {
        Ok(())
    }
}