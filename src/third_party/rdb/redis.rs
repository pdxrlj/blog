use std::error::Error;

use redis::Connection as RedisConnection;

use crate::third_party::config::config::Config;

pub trait IRedis {
    fn new(config: &Config) -> Result<Redis, Box<dyn Error>> {
        let conn = redis_conn(config)?;
        Ok(Redis {
            endpoint: "".to_string(),
            redis_key_prefix: "".to_string(),
            conn,
            db: 0,
        })
    }

    fn set_db(&mut self, db: i8);

    fn set(&self);
}

pub struct Redis {
    endpoint: String,
    redis_key_prefix: String,
    conn: RedisConnection,
    db: i8,
}

impl IRedis for Redis {
    fn set_db(&mut self, db: i8) {
        self.db = db;
    }

    fn set(&self) {
        unimplemented!()
    }
}

pub fn redis_conn(config: &Config) -> Result<RedisConnection, Box<dyn Error>> {
    let client = redis::Client::open(config.redis.endpoint.as_str())?;
    let conn = client.get_connection()?;

    Ok(conn)
}
