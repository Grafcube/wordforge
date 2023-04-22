use activitypub_federation::FEDERATION_CONTENT_TYPE;
use actix_web::{guard, web, Route, Scope};

pub mod account;
pub mod novel;
pub mod user;

pub fn scope() -> Scope {
    web::scope("/api/v1")
        .service(account::validate)
        .service(novel::new_novel)
}

pub fn users() -> Route {
    web::route()
        .guard(guard::Header("accept", FEDERATION_CONTENT_TYPE))
        .to(user::get_user)
}

pub fn novels() -> Route {
    web::route()
        .guard(guard::Header("accept", FEDERATION_CONTENT_TYPE))
        .to(novel::get_novel)
}
