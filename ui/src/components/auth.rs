use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub(crate) fn Auth(cx: Scope) -> impl IntoView {
    let (errormsg, set_errormsg) = create_signal(cx, String::new());

    view! { cx,
        <Body class="main-screen"/>
        <div class="flex flex-col md:flex-row mx-auto max-w-3xl text-2xl m-4 justify-center text-center place-content-center items-center">
            <Login set_errormsg=set_errormsg/>
            <Register set_errormsg=set_errormsg/>
        </div>
        <div class="flex mx-auto text-2xl m-4 justify-center text-center">
            <ErrorView message=errormsg/>
        </div>
    }
}

#[component]
fn ErrorView(cx: Scope, message: ReadSignal<String>) -> impl IntoView {
    view! { cx, <p class="text-red-800">{message}</p> }
}

#[component]
fn Login(cx: Scope, set_errormsg: WriteSignal<String>) -> impl IntoView {
    let login = create_server_action::<ServerLogin>(cx);
    let response = login.value();
    let err = move || {
        response.get().map(|v| match v {
            Ok(v) => set_errormsg(v),
            Err(e) => set_errormsg(e.to_string()),
        })
    };

    view! { cx,
        <ActionForm action=login class="space-y-4 p-4 w-full">
            <input type="email" class="basic-input" placeholder="Email" name="email" required/>
            <input
                type="password"
                class="basic-input"
                placeholder="Password (minimum 8 characters)"
                name="password"
                required
            />
            <input type="hidden" name="client_app" value="Web"/>
            <input type="submit" class="button-1" value="Sign in"/>
            {err}
        </ActionForm>
    }
}

#[server(ServerLogin, "/server")]
pub async fn login(
    cx: Scope,
    email: String,
    password: String,
    client_app: String,
    client_website: Option<String>,
) -> Result<String, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_web::http::StatusCode;
    use argon2::{Argon2, PasswordVerifier};
    use leptos_actix::ResponseOptions;
    use serde::{Deserialize, Serialize};
    use sqlx::PgPool;
    use std::sync::Arc;
    use validator::Validate;

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<Arc<PgPool>> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let session = <actix_session::Session as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    #[derive(Serialize, Deserialize, Validate)]
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

    let e = info.validate();
    if e.is_err() {
        resp.set_status(StatusCode::BAD_REQUEST);
        return Ok(e.err().unwrap().to_string());
    }

    let res = sqlx::query!(
        "SELECT apub_id, password FROM users WHERE lower(email)=$1",
        info.email.to_lowercase()
    )
    .fetch_one(pool.app_data().as_ref())
    .await;

    let res = match res {
        Ok(res) => res,
        Err(_) => {
            resp.set_status(StatusCode::UNAUTHORIZED);
            return Ok("Email address is not registered".to_string());
        }
    };

    let password_hash = argon2::PasswordHash::new(&res.password)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match PasswordVerifier::verify_password(
        &Argon2::default(),
        info.password.as_bytes(),
        &password_hash,
    ) {
        Ok(_) => {
            session
                .insert("id", &res.apub_id)
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
            session
                .insert("client_app", &info.client_app)
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
            session
                .insert("client_website", &info.client_website)
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
            leptos_actix::redirect(cx, "/");
            Ok(res.apub_id)
        }
        Err(e) => {
            resp.set_status(StatusCode::UNAUTHORIZED);
            Ok(e.to_string())
        }
    }
}

#[component]
fn Register(cx: Scope, set_errormsg: WriteSignal<String>) -> impl IntoView {
    let register = create_server_action::<ServerRegister>(cx);
    let response = register.value();
    let err = move || {
        response.get().map(|v| match v {
            Ok(v) => set_errormsg(v),
            Err(e) => set_errormsg(e.to_string()),
        })
    };

    view! { cx,
        <ActionForm action=register class="space-y-4 p-4 w-full">
            <input
                type="text"
                class="basic-input"
                placeholder="Display name"
                name="display_name"
                required
            />
            <input type="text" class="basic-input" placeholder="Username" name="username" required/>
            <input type="email" class="basic-input" placeholder="Email" name="email" required/>
            <input
                type="password"
                class="basic-input"
                placeholder="Password"
                name="password"
                required
            />
            <input type="hidden" name="client_app" value="Web"/>
            <input type="submit" class="button-1" value="Sign up"/>
            {err}
        </ActionForm>
    }
}

#[server(ServerRegister, "/server")]
pub async fn register(
    cx: Scope,
    display_name: String,
    username: String,
    email: String,
    password: String,
    client_app: String,
    client_website: Option<String>,
) -> Result<String, ServerFnError> {
    use crate::util::USERNAME_RE;
    use activitypub_federation::{config::Data, http_signatures::generate_actor_keypair};
    use actix_web::http::StatusCode;
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHasher,
    };
    use leptos_actix::ResponseOptions;
    use serde::{Deserialize, Serialize};
    use sqlx::{query, PgPool};
    use std::sync::Arc;
    use validator::Validate;

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<Arc<PgPool>> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

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

    let e = info.validate();
    if e.is_err() {
        resp.set_status(StatusCode::BAD_REQUEST);
        return Ok(e.err().unwrap().to_string());
    }

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
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
        Ok(v) => {
            match v.email {
                None => (),
                Some(e) => {
                    if e {
                        resp.set_status(StatusCode::BAD_REQUEST);
                        return Ok("Email is already registered".to_string());
                    }
                }
            };
            match v.username {
                None => (),
                Some(u) => {
                    if u {
                        resp.set_status(StatusCode::BAD_REQUEST);
                        return Ok("Username is already is use".to_string());
                    }
                }
            };
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(info.password.clone().into_bytes().as_slice(), &salt)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .to_string();
    let keypair =
        generate_actor_keypair().map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    query!(
        r#"INSERT INTO users
           (apub_id, preferred_username, name, inbox, outbox, public_key, private_key, email, password)
           VALUES (lower($1), $2, $3, $4, $5, $6, $7, $8, $9)"#,
        format!("{}/user/{}", pool.domain(), info.username.to_lowercase()),
        info.username,
        info.display_name,
        format!("{}/user/{}/inbox", pool.domain(), info.username.to_lowercase()),
        format!("{}/user/{}/outbox", pool.domain(), info.username.to_lowercase()),
        keypair.public_key,
        keypair.private_key,
        info.email,
        password,
    )
    .execute(pool.app_data().as_ref())
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    login(cx, info.email, info.password, client_app, client_website).await
}
