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
            Ok(Ok(_)) => (),
            Ok(Err(v)) => set_errormsg(v),
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
) -> Result<Result<String, String>, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_web::http::StatusCode;
    use leptos_actix::ResponseOptions;
    use sqlx::PgPool;
    use std::sync::Arc;
    use wordforge_api::account::{self, LoginAuthError, LoginResult};

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<Arc<PgPool>> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let session = <actix_session::Session as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match account::login(
        pool.app_data().as_ref(),
        session,
        email,
        password,
        client_app,
        client_website,
    )
    .await
    {
        LoginResult::Ok(apub_id) => {
            leptos_actix::redirect(cx, "/");
            Ok(Ok(apub_id))
        }
        LoginResult::InternalServerError(e) => Err(ServerFnError::ServerError(e)),
        LoginResult::BadRequest(e) => {
            resp.set_status(StatusCode::BAD_REQUEST);
            Ok(Err(e))
        }
        LoginResult::Unauthorized(LoginAuthError::Email) => {
            resp.set_status(StatusCode::UNAUTHORIZED);
            Ok(Err("Email address is not registered".to_string()))
        }
        LoginResult::Unauthorized(LoginAuthError::Password) => {
            resp.set_status(StatusCode::UNAUTHORIZED);
            Ok(Err("Wrong password".to_string()))
        }
    }
}

#[component]
fn Register(cx: Scope, set_errormsg: WriteSignal<String>) -> impl IntoView {
    let register = create_server_action::<ServerRegister>(cx);
    let response = register.value();
    let err = move || {
        response.get().map(|v| match v {
            Ok(Ok(_)) => (),
            Ok(Err(v)) => set_errormsg(v),
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
) -> Result<Result<String, String>, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_web::http::StatusCode;
    use leptos_actix::ResponseOptions;
    use wordforge_api::{
        account::{self, RegisterAuthError, RegistrationResult},
        DbHandle,
    };

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<DbHandle> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match account::register(
        pool,
        display_name,
        username,
        email.clone(),
        password.clone(),
    )
    .await
    {
        RegistrationResult::Ok => login(cx, email, password, client_app, client_website).await,
        RegistrationResult::BadRequest(e) => {
            resp.set_status(StatusCode::BAD_REQUEST);
            Ok(Err(e))
        }
        RegistrationResult::Conflict(RegisterAuthError::Email) => {
            resp.set_status(StatusCode::CONFLICT);
            Ok(Err("Email is already registered".to_string()))
        }
        RegistrationResult::Conflict(RegisterAuthError::Username) => {
            resp.set_status(StatusCode::BAD_REQUEST);
            Ok(Err("Username is taken".to_string()))
        }
        RegistrationResult::InternalServerError(e) => Err(ServerFnError::ServerError(e)),
    }
}
