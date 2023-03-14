use actix_web::{web, Scope};

pub mod account;

pub fn scope() -> Scope {
    web::scope("/api/v1")
        .service(account::create)
        .service(account::login)
        .service(account::validate)
}
