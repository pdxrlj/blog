use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize};

pub trait IConfig {
    fn from_file(&self) -> Result<Config, Box<dyn Error>>;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub redis: Redis,
    pub db: Db,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            redis: Redis {
                endpoint: "".to_string(),
                redis_key_prefix: "".to_string(),
            },
            db: Db {
                endpoint: "".to_string(),
                max_open_conns: 0,
                idle_max_lifetime: 0,
                conn_max_lifetime: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Redis {
    pub endpoint: String,
    pub redis_key_prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Db {
    pub endpoint: String,
    pub max_open_conns: i8,
    pub idle_max_lifetime: u64,
    pub conn_max_lifetime: u64,
}

impl IConfig for Config {
    fn from_file(&self) -> Result<Config, Box<dyn Error>> {
        let config_path = std::env::vars()
            .find(|(key, _)| key == "CONFIG_PATH")
            .map_or_else(|| "src/config/blog.toml".to_string(), |(_, path)| path);
        println!("config_path: {}", config_path);
        let config_text = fs::read_to_string(config_path)?;

        let config: Config = toml::from_str(&config_text)?;

        Ok(Config {
            redis: Redis {
                endpoint: config.redis.endpoint,
                redis_key_prefix: config.redis.redis_key_prefix,
            },
            db: Db {
                endpoint: config.db.endpoint,
                max_open_conns: config.db.max_open_conns,
                idle_max_lifetime: config.db.idle_max_lifetime,
                conn_max_lifetime: config.db.conn_max_lifetime,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_file() {
        let config = Config::new();
        let config = config.from_file();
        match config {
            Ok(config) => {
                println!("read config:{:#?}", config);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[test]
    fn test_parse_file_not_exists() {
        std::env::set_var("CONFIG_PATH", "");
        let config = Config::new();
        let config = config.from_file();
        std::env::remove_var("CONFIG_PATH");
        match config {
            Ok(config) => {
                println!("read config:{:#?}", config);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
