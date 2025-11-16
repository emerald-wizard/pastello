use crate::ports::Rng;
use async_trait::async_trait;
// --- FIX: Import the Rng trait from the rand crate to get .gen_range() ---
use rand::Rng as RandRng;

pub struct ThreadRng;

impl ThreadRng {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ThreadRng {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Rng for ThreadRng {
    // --- FIX: Implement the 'rand_int' method from the trait ---
    async fn rand_int(&self, min: i32, max: i32) -> i32 {
        if min >= max {
            // Guard against panic if min >= max
            return min;
        }
        // Use the rand crate's thread_rng to generate the number
        rand::thread_rng().gen_range(min..max)
    }
}