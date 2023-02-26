use crate::{actors::person::User, instance::Database, util::USERNAME_RE};
use activitypub_federation::{core::signatures::generate_actor_keypair, request_data::RequestData};
use actix_web::{error::ErrorBadRequest, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::{query_as, PgPool};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct NewUser {
    display_name: String,
    #[validate(regex = "USERNAME_RE")]
    username: String,
    #[validate(email)]
    email: String,
}

#[post("/accounts")]
async fn create(
    pool: RequestData<Database>,
    info: web::Json<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    match create_account(info.into_inner(), pool.get_pool()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(e),
    }
    .map_err(ErrorBadRequest)
}

async fn create_account(info: NewUser, conn: &PgPool) -> anyhow::Result<User> {
    let keypair = generate_actor_keypair()?;

    query_as!(
        User,
        "INSERT INTO users \
        (preferred_username, name, public_key, private_key, email) \
        VALUES ($1, $2, $3, $4, $5) RETURNING *",
        info.username,
        info.display_name,
        keypair.public_key,
        keypair.private_key,
        info.email
    )
    .fetch_one(conn)
    .await
    .map_err(anyhow::Error::new)
}
