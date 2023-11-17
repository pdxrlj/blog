mod third_party;

use third_party::{
    cache::cache::CacheEntry,
    config::config::{Config, IConfig},
    manager::Manager,
    rdb::redis::Redis,
};

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let third_party_server = server();
        let redis = third_party_server.get_redis();
        let config = third_party_server.get_config();
        let cache = third_party_server.get_cache();

        App::new()
            .app_data(web::Data::new(redis.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(cache.clone()))
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn server() -> Manager<CacheEntry, Config, Redis> {
    let c = config();
    Manager::<CacheEntry, Config, Redis>::new(c)
}

fn config() -> Config {
    let config = Config::default();
    let config_content = config
        .from_file()
        .expect("parse config from file have error");
    config_content
}
