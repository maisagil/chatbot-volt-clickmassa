use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct TokenCache {
    cache: Arc<Cache<String, String>>,
}

impl TokenCache {
    pub fn new(ttl_seconds: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        Self {
            cache: Arc::new(cache),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).await
    }

    pub async fn set(&self, key: String, value: String) {
        self.cache.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub async fn invalidate_all(&self) {
        self.cache.invalidate_all();
    }
}
