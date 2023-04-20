use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub(crate) fn Auth(cx: Scope) -> impl IntoView {
    let (errormsg, set_errormsg) = create_signal(cx, String::new());

    view! { cx,
        <Body class="main-screen"/>
        <Login set_errormsg=set_errormsg/>
    }
}

#[component]
fn Login(cx: Scope, set_errormsg: WriteSignal<String>) -> impl IntoView {
    let login = create_server_action::<ServerLogin>(cx);
    view! { cx,
        <ActionForm action=login>
            <input type="email" class="basic-input" placeholder="Email" name="email" required/>
            <input
                type="password"
                class="basic-input"
                placeholder="Password (minimum 8 characters)"
                name="password"
                required
            />
            <input type="submit" class="button-1" value="Sign in"/>
            <input type="hidden" name="client_app" value="Web"/>
        </ActionForm>
    }
}

#[component]
fn ErrorView(cx: Scope, message: String) -> impl IntoView {
    view! { cx, <p class="text-red-800">{message}</p> }
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

    match argon2::PasswordVerifier::verify_password(
        &argon2::Argon2::default(),
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
        Err(e) => Err({
            resp.set_status(StatusCode::UNAUTHORIZED);
            ServerFnError::ServerError(e.to_string())
        }),
    }
}
