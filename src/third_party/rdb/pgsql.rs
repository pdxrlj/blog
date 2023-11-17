use std::error::Error;
use std::time::Duration;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::third_party::config::config::Config;

async fn pgsql_conn(config: &Config) -> Result<Pool<Postgres>, Box<dyn Error>> {
    let pgConn = PgPoolOptions::new()
        .max_connections(config.db.max_open_conns as u32)
        .max_lifetime(Duration::from_secs(config.db.conn_max_lifetime.into()))
        .idle_timeout(Duration::from_secs(config.db.idle_max_lifetime.into()))
        .connect(config.db.endpoint.as_str())
        .await?;

    Ok(pgConn)
}