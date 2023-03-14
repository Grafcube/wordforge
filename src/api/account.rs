use crate::{instance::Database, util::USERNAME_RE};
use activitypub_federation::{core::signatures::generate_actor_keypair, request_data::RequestData};
use actix_session::Session;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    get, post, web, HttpResponse,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Utc};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct NewUser {
    display_name: String,
    #[validate(regex = "USERNAME_RE")]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize, Deserialize)]
struct InsertedUser {
    id: Uuid,
    preferred_username: String,
    name: String,
    published: DateTime<Utc>,
    email: String,
}

#[derive(Clone, Serialize, Deserialize, Validate)]
struct Login {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
    client_app: String,
    #[validate(url)]
    client_website: Option<String>,
}

#[post("/accounts")]
async fn create(
    pool: RequestData<Database>,
    info: web::Json<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    create_account(info.into_inner(), pool.get_pool())
        .await
        .map(|v| HttpResponse::Ok().json(v))
}

async fn create_account(info: NewUser, conn: &PgPool) -> Result<InsertedUser, actix_web::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = argon2
        .hash_password(info.password.into_bytes().as_slice(), &salt)
        .map_err(ErrorInternalServerError)?
        .to_string();

    let keypair = generate_actor_keypair()?;

    query_as!(
        InsertedUser,
        "INSERT INTO users \
        (preferred_username, name, public_key, private_key, email, password) \
        VALUES ($1, $2, $3, $4, $5, $6) \
        RETURNING id, preferred_username, name, published, email",
        info.username,
        info.display_name,
        keypair.public_key,
        keypair.private_key,
        info.email,
        password,
    )
    .fetch_one(conn)
    .await
    .map_err(ErrorBadRequest)
}

#[post("/login")]
async fn login(
    pool: RequestData<Database>,
    info: web::Json<Login>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    match verify_session(info.clone(), pool.get_pool()).await {
        Ok(id) => {
            session.insert("id", id)?;
            session.insert("client_app", info.client_app.clone())?;
            session.insert("client_website", info.client_website.clone())?;
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => Err(e),
    }
}

async fn verify_session(info: Login, conn: &PgPool) -> Result<Uuid, actix_web::Error> {
    struct UserInfo {
        pub id: Uuid,
        pub password: String,
    }

    let res = query_as!(
        UserInfo,
        "SELECT id, password FROM users WHERE email=$1",
        info.email
    )
    .fetch_one(conn)
    .await
    .map_err(|_| ErrorBadRequest("Email address is not registered"))?;
    let password_hash = PasswordHash::new(&res.password).map_err(ErrorInternalServerError)?;

    match Argon2::default().verify_password(info.password.as_bytes(), &password_hash) {
        Ok(_) => Ok(res.id),
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[get("/validate")]
async fn validate(
    pool: RequestData<Database>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let id = session
        .get::<Uuid>("id")?
        .ok_or_else(|| ErrorUnauthorized("Not signed in"))?;
    let name = query!("SELECT name FROM users WHERE id=$1", id)
        .fetch_one(pool.get_pool())
        .await
        .map_err(ErrorNotFound)?
        .name;
    Ok(HttpResponse::Ok().body(name))
}
