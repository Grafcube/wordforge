use crate::{
    components::{basicinput::*, errorview::*},
    path::AuthQueries,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub(crate) fn Auth(cx: Scope) -> impl IntoView {
    let (errormsg, set_errormsg) = create_signal(cx, String::new());
    let query = use_query::<AuthQueries>(cx);
    let path = move || {
        query
            .with(|q| {
                q.clone().map(|p| {
                    p.redirect_to
                        .is_empty()
                        .then(|| "/".to_string())
                        .unwrap_or_else(|| p.redirect_to)
                })
            })
            .unwrap()
    };

    view! { cx,
        <Title text="Sign in or create an account"/>
        <div class="mx-auto w-full">
            <div class="flex flex-col sm:flex-row w-full mx-auto max-w-3xl text-2xl m-4 justify-center text-center place-content-center items-center">
                <Login redirect_to=path() set_errormsg=set_errormsg/>
                <Register redirect_to=path() set_errormsg=set_errormsg/>
            </div>
            <div class="flex mx-auto text-2xl m-4 justify-center text-center">
                <ErrorView message=errormsg/>
            </div>
        </div>
    }
}

#[component]
fn Login(cx: Scope, redirect_to: String, set_errormsg: WriteSignal<String>) -> impl IntoView {
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
            <div class="relative">
                <input type="email" class="basic-input peer" placeholder=" " name="email" required/>
                <FloatingLabel target="email">"Email"</FloatingLabel>
            </div>
            <div class="relative">
                <input
                    type="password"
                    class="basic-input peer"
                    placeholder=" "
                    name="password"
                    required
                />
                <FloatingLabel target="password">"Password"</FloatingLabel>
            </div>
            <div class="relative">
                <input type="hidden" name="client_app" value="Web"/>
                <input type="hidden" name="redirect_to" value=redirect_to/>
                <input type="submit" class="button-1" value="Sign in"/>
            </div>
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
    redirect_to: String,
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
            leptos_actix::redirect(cx, &redirect_to);
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
fn Register(cx: Scope, redirect_to: String, set_errormsg: WriteSignal<String>) -> impl IntoView {
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
            <div class="relative">
                <input
                    type="text"
                    class="basic-input peer"
                    placeholder=" "
                    name="display_name"
                    required
                />
                <FloatingLabel target="display_name">"Display name"</FloatingLabel>
            </div>
            <div class="relative">
                <input
                    type="text"
                    class="basic-input peer"
                    placeholder=" "
                    name="username"
                    required
                />
                <FloatingLabel target="username">"Username"</FloatingLabel>
            </div>
            <div class="relative">
                <input type="email" class="basic-input peer" placeholder=" " name="email" required/>
                <FloatingLabel target="email">"Email"</FloatingLabel>
            </div>
            <div class="relative">
                <input
                    type="password"
                    class="basic-input peer"
                    placeholder=" "
                    name="password"
                    required
                />
                <FloatingLabel target="password">"Password"</FloatingLabel>
            </div>
            <input type="hidden" name="client_app" value="Web"/>
            <input type="hidden" name="redirect_to" value=redirect_to/>
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
    redirect_to: String,
) -> Result<Result<String, String>, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_web::{http::StatusCode, web};
    use leptos_actix::ResponseOptions;
    use wordforge_api::{
        account::{self, RegisterAuthError, RegistrationResult},
        util::AppState,
        DbHandle,
    };

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<DbHandle> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let state = <web::Data<AppState> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match account::register(
        state,
        pool,
        display_name,
        username,
        email.clone(),
        password.clone(),
    )
    .await
    {
        RegistrationResult::Ok => {
            login(cx, email, password, client_app, client_website, redirect_to).await
        }
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
