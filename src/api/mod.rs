use activitypub_federation::FEDERATION_CONTENT_TYPE;
use actix_web::{guard, web, Route, Scope};

pub mod account;
pub mod book;
pub mod user;

pub fn scope() -> Scope {
    web::scope("/api/v1")
        .service(account::create)
        .service(account::login)
        .service(account::validate)
        .service(book::new_book)
}

pub fn users() -> Route {
    web::route()
        .guard(guard::Header("accept", FEDERATION_CONTENT_TYPE))
        .to(user::get_user)
}
