use crate::ports::IDGen;
use uuid::Uuid;

pub struct UuidGen;

impl UuidGen {
    pub fn new() -> Self { Self }
}

impl IDGen for UuidGen {
    fn new_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}