// FIX: Corrected trait import name from IDGen to IdGenerator (E0432)
use crate::ports::IdGenerator; 
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
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

impl IdGenerator for UuidGenerator {
    fn new_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}