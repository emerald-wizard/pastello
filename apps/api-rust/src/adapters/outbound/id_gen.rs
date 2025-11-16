use crate::ports::IDGen;
use async_trait::async_trait;
use uuid::Uuid;

// --- FIX: Add pub ---
pub struct UuidGenerator;

impl UuidGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UuidGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IDGen for UuidGenerator {
    async fn new_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}