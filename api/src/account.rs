use actix_session::Session;
use sqlx::{query, PgPool};

pub enum UserValidateResult {
    Ok(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError(String),
}

pub async fn validate(conn: &PgPool, session: Session) -> UserValidateResult {
    let id = match session.get::<String>("id") {
        Err(e) => return UserValidateResult::InternalServerError(e.to_string()),
        Ok(v) => match v {
            Some(u) => u,
            None => return UserValidateResult::Unauthorized("Not signed in".to_string()),
        },
    };
    session.renew();
    let name = match query!("SELECT name FROM users WHERE apub_id=$1", id)
        .fetch_one(conn)
        .await
    {
        Ok(v) => v.name,
        Err(e) => return UserValidateResult::NotFound(e.to_string()),
    };
    UserValidateResult::Ok(name)
}
