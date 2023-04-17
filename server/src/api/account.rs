use crate::util::USERNAME_RE;
use activitypub_federation::{config::Data, http_signatures::generate_actor_keypair};
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
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct NewUser {
    display_name: String,
    #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize, Deserialize)]
struct InsertedUser {
    apub_id: String,
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
async fn create(pool: Data<PgPool>, info: web::Json<NewUser>) -> actix_web::Result<HttpResponse> {
    create_account(info.into_inner(), pool.domain(), pool.app_data())
        .await
        .map(|v| HttpResponse::Ok().json(v))
}

async fn create_account(
    info: NewUser,
    host: &str,
    conn: &PgPool,
) -> actix_web::Result<InsertedUser> {
    info.validate().map_err(ErrorBadRequest)?;
    if query!(
        "SELECT * FROM users WHERE lower(preferred_username)=$1",
        info.username.to_lowercase()
    )
    .fetch_one(conn)
    .await
    .is_ok()
    {
        return Err(ErrorBadRequest("Username already exists"));
    }

    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(info.password.into_bytes().as_slice(), &salt)
        .map_err(ErrorInternalServerError)?
        .to_string();
    let keypair = generate_actor_keypair()?;

    query_as!(
        InsertedUser,
        r#"INSERT INTO users
           (apub_id, preferred_username, name, inbox, outbox, public_key, private_key, email, password)
           VALUES (lower($1), $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING apub_id, preferred_username, name, published, email"#,
        format!("{}/user/{}", host, info.username.to_lowercase()),
        info.username,
        info.display_name,
        format!("{}/user/{}/inbox", host, info.username.to_lowercase()),
        format!("{}/user/{}/outbox", host, info.username.to_lowercase()),
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
    pool: Data<PgPool>,
    info: web::Form<Login>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    match verify_session(&info, pool.app_data()).await {
        Ok(id) => {
            session.insert("id", id)?;
            session.insert("client_app", &info.client_app)?;
            session.insert("client_website", &info.client_website)?;
            Ok(HttpResponse::Found()
                .append_header(("location", "/"))
                .finish())
        }
        Err(e) => Err(e),
    }
}

async fn verify_session(info: &Login, conn: &PgPool) -> actix_web::Result<String> {
    info.validate().map_err(ErrorBadRequest)?;
    let res = query!(
        "SELECT apub_id, password FROM users WHERE lower(email)=$1",
        info.email.to_lowercase()
    )
    .fetch_one(conn)
    .await
    .map_err(|_| ErrorBadRequest("Email address is not registered"))?;
    let password_hash = PasswordHash::new(&res.password).map_err(ErrorInternalServerError)?;

    match Argon2::default().verify_password(info.password.as_bytes(), &password_hash) {
        Ok(_) => Ok(res.apub_id),
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[get("/validate")]
async fn validate(pool: Data<PgPool>, session: Session) -> actix_web::Result<HttpResponse> {
    let id = session
        .get::<String>("id")?
        .ok_or_else(|| ErrorUnauthorized("Not signed in"))?;
    session.renew();
    let name = query!("SELECT name FROM users WHERE apub_id=$1", id)
        .fetch_one(pool.app_data())
        .await
        .map_err(ErrorNotFound)?
        .name;
    Ok(HttpResponse::Ok().body(name))
}
