use crate::ports::Rng;
use async_trait::async_trait;
use rand::{rng, Rng as _};

#[derive(Debug, Clone, Copy)]
pub struct SystemRng;

impl SystemRng {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemRng {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Rng for SystemRng {
    async fn rand_int(&self, min: i32, max: i32) -> i32 {
        tokio::task::spawn_blocking(move || {
            // Use random_range to fix deprecation in newer rand versions,
            // or fallback to gen_range on the thread_rng().
            let mut rng = rng();
            rng.random_range(min..max)
        })
        .await
        .unwrap_or_else(|e| {
            eprintln!("Blocking task for rand_int panicked: {:?}", e);
            min
        })
    }
}
