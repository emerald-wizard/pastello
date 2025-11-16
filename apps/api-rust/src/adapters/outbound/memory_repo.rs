use crate::ports::Repo;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;

// --- FIX: Add pub ---
pub struct MemoryRepo<T>
where
    T: Clone + Send + Sync + 'static,
{
    store: Arc<DashMap<String, T>>,
}

impl<T> MemoryRepo<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }
}

impl<T> Default for MemoryRepo<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}


#[async_trait]
impl<T> Repo<T> for MemoryRepo<T>
where
    T: Clone + Send + Sync + 'static,
{
    async fn find(&self, id: &str) -> Option<T> {
        self.store.get(id).map(|item| item.value().clone())
    }

    async fn save(&self, id: &str, item: T) {
        self.store.insert(id.to_string(), item);
    }
}