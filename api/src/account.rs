use crate::{
    util::{AppState, USERNAME_RE},
    DbHandle,
};
use activitypub_federation::{config::Data, http_signatures::generate_actor_keypair};
use actix_session::Session;
use actix_web::web;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum UserValidateError {
    #[error("User Unauthorized: {0}")]
    Unauthorized(String),
    #[error("User NotFound: {0}")]
    NotFound(String),
    #[error("User InternalServerError: {0}")]
    InternalServerError(String),
}

pub async fn validate(
    conn: &PgPool,
    session: Session,
) -> Result<(String, String), UserValidateError> {
    let id = match session.get::<String>("id") {
        Err(e) => return Err(UserValidateError::InternalServerError(e.to_string())),
        Ok(Some(u)) => u,
        Ok(None) => return Err(UserValidateError::Unauthorized("Not signed in".to_string())),
    };
    session.renew();
    let name = match query!("SELECT apub_id, name FROM users WHERE apub_id=$1", id)
        .fetch_optional(conn)
        .await
    {
        Ok(Some(v)) => (v.apub_id, v.name),
        Ok(None) => {
            return Err(UserValidateError::Unauthorized(
                "Expired session".to_string(),
            ))
        }
        Err(e) => return Err(UserValidateError::NotFound(e.to_string())),
    };
    Ok(name)
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("LoginError: BadRequest: {0}")]
    BadRequest(String),
    #[error("LoginError: Unauthorized")]
    Unauthorized(FormAuthError),
    #[error("LoginError: InternalServerError: {0}")]
    InternalServerError(String),
}

#[derive(Debug, Error)]
pub enum FormAuthError {
    #[error("Email not found")]
    Email,
    #[error("Wrong password")]
    Password,
    #[error("Username not found")]
    Username,
}

pub async fn login(
    pool: &PgPool,
    session: Session,
    email: String,
    password: String,
    client_app: String,
    client_website: Option<String>,
) -> Result<String, LoginError> {
    #[derive(Deserialize, Validate)]
    struct LoginData {
        #[validate(email)]
        email: String,
        #[validate(length(min = 8))]
        password: String,
        client_app: String,
        #[validate(url)]
        client_website: Option<String>,
    }

    let info = LoginData {
        email,
        password,
        client_app,
        client_website,
    };

    info.validate()
        .map_err(|e| LoginError::BadRequest(e.to_string()))?;

    let res = sqlx::query!(
        "SELECT apub_id, password FROM users WHERE lower(email)=$1",
        info.email.to_lowercase()
    )
    .fetch_one(pool)
    .await
    .map_err(|_| LoginError::Unauthorized(FormAuthError::Email))?;

    let password_hash = PasswordHash::new(&res.password)
        .map_err(|e| LoginError::InternalServerError(e.to_string()))?;

    match PasswordVerifier::verify_password(
        &Argon2::default(),
        info.password.as_bytes(),
        &password_hash,
    ) {
        Ok(_) => {
            session
                .insert("id", &res.apub_id)
                .map_err(|e| LoginError::InternalServerError(e.to_string()))?;
            session
                .insert("client_app", &info.client_app)
                .map_err(|e| LoginError::InternalServerError(e.to_string()))?;
            session
                .insert("client_website", &info.client_website)
                .map_err(|e| LoginError::InternalServerError(e.to_string()))?;
            Ok(res.apub_id)
        }
        Err(_) => Err(LoginError::Unauthorized(FormAuthError::Password)),
    }
}

pub enum RegistrationError {
    Conflict(FormAuthError),
    BadRequest(String),
    InternalServerError(String),
}

pub async fn register(
    state: web::Data<AppState>,
    pool: Data<DbHandle>,
    display_name: String,
    username: String,
    email: String,
    password: String,
) -> Result<(), RegistrationError> {
    #[derive(Debug, Deserialize, Serialize, Validate)]
    struct NewUser {
        display_name: String,
        #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
        username: String,
        #[validate(email)]
        email: String,
        #[validate(length(min = 8))]
        password: String,
    }

    let info = NewUser {
        display_name,
        username,
        email,
        password,
    };

    info.validate()
        .map_err(|e| RegistrationError::BadRequest(e.to_string()))?;

    let scheme = &state.scheme;

    match query!(
        r#"SELECT
           EXISTS(SELECT 1 FROM users WHERE preferred_username = $1) AS username,
           EXISTS(SELECT 1 FROM users WHERE email = $2) AS email"#,
        info.username.to_lowercase(),
        info.email.to_lowercase()
    )
    .fetch_one(pool.app_data().as_ref())
    .await
    {
        Err(e) => return Err(RegistrationError::InternalServerError(e.to_string())),
        Ok(v) => {
            match v.email {
                None => (),
                Some(e) => {
                    if e {
                        return Err(RegistrationError::Conflict(FormAuthError::Email));
                    }
                }
            };
            match v.username {
                None => (),
                Some(u) => {
                    if u {
                        return Err(RegistrationError::Conflict(FormAuthError::Username));
                    }
                }
            };
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(info.password.into_bytes().as_slice(), &salt)
        .map_err(|e| RegistrationError::InternalServerError(e.to_string()))?
        .to_string();
    let keypair = generate_actor_keypair()
        .map_err(|e| RegistrationError::InternalServerError(e.to_string()))?;

    query!(
        r#"INSERT INTO users
           (apub_id, preferred_username, name, inbox, outbox, public_key, private_key, email, password)
           VALUES (lower($1), $2, $3, $4, $5, $6, $7, $8, $9)"#,
        format!("{}://{}/user/{}", scheme, pool.domain(), info.username.to_lowercase()),
        info.username,
        info.display_name,
        format!("{}://{}/user/{}/inbox", scheme, pool.domain(), info.username.to_lowercase()),
        format!("{}://{}/user/{}/outbox", scheme, pool.domain(), info.username.to_lowercase()),
        keypair.public_key,
        keypair.private_key,
        info.email,
        password,
    )
    .execute(pool.app_data().as_ref())
    .await
    .map_err(|e| RegistrationError::InternalServerError(e.to_string()))?;

    Ok(())
}
