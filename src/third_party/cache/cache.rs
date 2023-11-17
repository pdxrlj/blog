use moka::future::Cache;

pub trait ICache {
    fn insert(&self, key: &str, value: String);
    fn get(&self, key: &str) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    cache: Cache<&'static str, String>,
}

impl<'a> Default for CacheEntry {
    fn default() -> Self {
        CacheEntry {
            cache: Cache::builder().max_capacity(500 * 1024 * 1024).build(),
        }
    }
}

impl ICache for CacheEntry {
    fn insert(&self, key: &str, value: String) {
        unimplemented!()
    }
    fn get(&self, key: &str) -> Option<String> {
        unimplemented!()
    }
}

async fn get_cache() -> Cache<String, String> {
    // 500MiB
    Cache::builder().max_capacity(500 * 1024 * 1024).build()
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_cache() {
        let cache = get_cache().await;
        cache.insert("key".to_string(), "value".to_string()).await;
        let value = cache.get(&"key".to_string()).await;
        println!("cache: {:?}", value);
        let cache_with_key = "key".to_string();
        let demo = cache
            .get_with(cache_with_key, async move {
                // println!("key: {}", key);
                "value".to_string()
            })
            .await;

        println!("cache get_with: {:?}", demo);
    }
}
