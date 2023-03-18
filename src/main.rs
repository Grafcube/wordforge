use crate::instance::new_database;
use activitypub_federation::config::FederationMiddleware;
use actix_files::{Files, NamedFile};
use actix_session::{
    config::CookieContentSecurity, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::{self, Compress},
    App, HttpServer,
};
use std::{env, io};

mod api;
mod instance;
mod objects;
mod util;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    #[cfg(not(debug_assertions))]
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "50505".to_string());
    let host = format!("{addr}:{port}");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is required");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let config = new_database(host.clone(), db_url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    log::info!("Starting server");
    log::info!("Listening on http://{host}");

    HttpServer::new(move || {
        let session = SessionMiddleware::builder(
            RedisActorSessionStore::new(redis_url.clone()),
            Key::generate(),
        )
        .cookie_secure(false) // TODO: Remove eventually
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_same_site(SameSite::Strict)
        .build();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(session)
            .wrap(Compress::default())
            .wrap(FederationMiddleware::new(config.clone()))
            .route("/user/{name}", api::users())
            .service(api::scope())
            .service(
                Files::new("/", "./ui/build")
                    .index_file("index.html")
                    .default_handler(
                        NamedFile::open("./ui/build/index.html").expect("Index file should exist"),
                    ),
            )
    })
    .bind(host)?
    .run()
    .await
}
