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
use validator::Validate;

pub enum UserValidateResult {
    Ok(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError(String),
}

pub async fn validate(conn: &PgPool, session: Session) -> UserValidateResult {
    let id = match session.get::<String>("id") {
        Err(e) => return UserValidateResult::InternalServerError(e.to_string()),
        Ok(Some(u)) => u,
        Ok(None) => return UserValidateResult::Unauthorized("Not signed in".to_string()),
    };
    session.renew();
    let name = match query!("SELECT name FROM users WHERE apub_id=$1", id)
        .fetch_optional(conn)
        .await
    {
        Ok(Some(v)) => v.name,
        Ok(None) => return UserValidateResult::Unauthorized("Expired session".to_string()),
        Err(e) => return UserValidateResult::NotFound(e.to_string()),
    };
    UserValidateResult::Ok(name)
}

pub enum LoginResult {
    Ok(String),
    BadRequest(String),
    Unauthorized(LoginAuthError),
    InternalServerError(String),
}

pub enum LoginAuthError {
    Email,
    Password,
}

pub async fn login(
    pool: &PgPool,
    session: Session,
    email: String,
    password: String,
    client_app: String,
    client_website: Option<String>,
) -> LoginResult {
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

    if let Err(e) = info.validate() {
        return LoginResult::BadRequest(e.to_string());
    }

    let res = match sqlx::query!(
        "SELECT apub_id, password FROM users WHERE lower(email)=$1",
        info.email.to_lowercase()
    )
    .fetch_one(pool)
    .await
    {
        Ok(res) => res,
        Err(_) => return LoginResult::Unauthorized(LoginAuthError::Email),
    };

    let password_hash = match PasswordHash::new(&res.password) {
        Ok(v) => v,
        Err(e) => return LoginResult::InternalServerError(e.to_string()),
    };

    match PasswordVerifier::verify_password(
        &Argon2::default(),
        info.password.as_bytes(),
        &password_hash,
    ) {
        Ok(_) => {
            if let Err(e) = session.insert("id", &res.apub_id) {
                return LoginResult::InternalServerError(e.to_string());
            };
            if let Err(e) = session.insert("client_app", &info.client_app) {
                return LoginResult::InternalServerError(e.to_string());
            };
            if let Err(e) = session.insert("client_website", &info.client_website) {
                return LoginResult::InternalServerError(e.to_string());
            };
            LoginResult::Ok(res.apub_id)
        }
        Err(_) => LoginResult::Unauthorized(LoginAuthError::Password),
    }
}

pub enum RegistrationResult {
    Ok,
    Conflict(RegisterAuthError),
    BadRequest(String),
    InternalServerError(String),
}

pub enum RegisterAuthError {
    Email,
    Username,
}

pub async fn register(
    state: web::Data<AppState>,
    pool: Data<DbHandle>,
    display_name: String,
    username: String,
    email: String,
    password: String,
) -> RegistrationResult {
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

    if let Err(e) = info.validate() {
        return RegistrationResult::BadRequest(e.to_string());
    }

    let scheme = state.scheme.clone();

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
        Err(e) => return RegistrationResult::InternalServerError(e.to_string()),
        Ok(v) => {
            match v.email {
                None => (),
                Some(e) => {
                    if e {
                        return RegistrationResult::Conflict(RegisterAuthError::Email);
                    }
                }
            };
            match v.username {
                None => (),
                Some(u) => {
                    if u {
                        return RegistrationResult::Conflict(RegisterAuthError::Username);
                    }
                }
            };
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let password =
        match Argon2::default().hash_password(info.password.into_bytes().as_slice(), &salt) {
            Ok(p) => p.to_string(),
            Err(e) => return RegistrationResult::InternalServerError(e.to_string()),
        };
    let keypair = match generate_actor_keypair() {
        Ok(k) => k,
        Err(e) => return RegistrationResult::InternalServerError(e.to_string()),
    };

    if let Err(e) = query!(
        r#"INSERT INTO users
           (apub_id, preferred_username, name, inbox, outbox, public_key, private_key, email, password)
           VALUES (lower($1), $2, $3, $4, $5, $6, $7, $8, $9)"#,
        format!("{}://{}/user/{}", scheme.clone(), pool.domain(), info.username.to_lowercase()),
        info.username,
        info.display_name,
        format!("{}://{}/user/{}/inbox", scheme.clone(), pool.domain(), info.username.to_lowercase()),
        format!("{}://{}/user/{}/outbox", scheme.clone(), pool.domain(), info.username.to_lowercase()),
        keypair.public_key,
        keypair.private_key,
        info.email,
        password,
    )
    .execute(pool.app_data().as_ref())
    .await
    {
        return RegistrationResult::InternalServerError(e.to_string());
    };

    RegistrationResult::Ok
}
