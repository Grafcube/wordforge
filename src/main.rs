use actix_web::{middleware, web, App, HttpServer};
use instance::Instance;
use std::io;

mod instance;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let addr = std::env::var("SERVER_ADDR").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or("50505".to_string());
    let instance = Instance::new(format!("{addr}:{port}"))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(instance.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1").route("/", web::get().to(|| async { "ActivityPub Test" })),
            )
    })
    .bind(format!("{addr}:{port}"))?
    .run()
    .await
}
