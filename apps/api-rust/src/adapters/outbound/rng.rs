use crate::ports::Rng;
use rand::{self, Rng as RandRng};

/// Implements the Rng port using rand::thread_rng
pub struct ThreadRng;

impl ThreadRng {
    pub fn new() -> Self { Self }
}

impl Rng for ThreadRng {
    fn intn(&self, n: i32) -> i32 {
        if n <= 0 {
            return 0; // Guard, same as Go's rand.Intn
        }
        rand::thread_rng().gen_range(0..n)
    }
}