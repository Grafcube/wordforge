mod handlers;
mod instance;
mod objects;
mod schema;
mod util;

use activitypub_federation::request_data::ApubMiddleware;
use actix_web::{middleware, web, App, HttpServer};
use handlers::*;
use instance::Database;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "50505".to_string());
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let data = Database::new(format!("{addr}:{port}"), db_url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(ApubMiddleware::new(data.clone()))
            .service(
                web::scope("/api/v1")
                    .route("/", web::get().to(|| async { "ActivityPub Test" }))
                    .service(account::create),
            )
    })
    .bind(format!("{addr}:{port}"))?
    .run()
    .await
}
