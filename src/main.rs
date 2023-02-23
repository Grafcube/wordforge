use activitypub_federation::request_data::ApubMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware::{self, Compress},
    App, HttpServer,
};
use instance::Database;
use std::io;

mod api;
mod instance;
mod objects;
mod schema;
mod util;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "50505".to_string());
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let data = Database::new(format!("{addr}:{port}"), db_url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    log::info!("Starting server");
    log::info!("Listening on http://{addr}:{port}");

    HttpServer::new(move || {
        let session =
            SessionMiddleware::builder(CookieSessionStore::default(), Key::generate()).build();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(session)
            .wrap(Compress::default())
            .wrap(ApubMiddleware::new(data))
            .service(api::scope())
    })
    .bind(format!("{addr}:{port}"))?
    .run()
    .await
}
