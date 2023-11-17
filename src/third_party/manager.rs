use crate::third_party::cache::cache::{CacheEntry, ICache};
use crate::third_party::config::config::{Config, IConfig};
use crate::third_party::rdb::redis::{IRedis, Redis};

pub struct Manager<C: ICache, G: IConfig, R: IRedis> {
    cache: C,
    config: G,
    redis: R,
}

impl<C, G, R> Manager<C, G, R>
where
    C: ICache,
    G: IConfig + Default,
    R: IRedis,
{
    pub fn new(config: Config) -> Manager<CacheEntry, Config, Redis> {
        let redis_conn = Redis::new(&config)
            .map_err(|e| "redis error new:".to_string() + e.to_string().as_str())
            .unwrap();

        Manager {
            cache: CacheEntry::default(),
            config,
            redis: redis_conn,
        }
    }

    pub fn get_cache(&self) -> &C {
        &self.cache
    }

    pub fn get_config(&self) -> &G {
        &self.config
    }

    pub fn get_redis(&self) -> &R {
        &self.redis
    }
}
