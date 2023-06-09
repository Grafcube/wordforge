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
                        .unwrap_or(p.redirect_to)
                })
            })
            .unwrap_or_else(|e| {
                warn!("AuthQuery: {e}");
                "/".to_string()
            })
    };

    view! { cx,
        <Title text="Sign in or create an account"/>
        <div class="mx-auto w-full">
            <div class="flex flex-col md:flex-row w-full mx-auto max-w-3xl text-2xl m-4 justify-center text-center place-content-center items-center">
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
                    minlength=8
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
    use actix_session::Session;
    use leptos_actix::extract;
    use wordforge_api::{
        account::{self, FormAuthError, LoginError},
        DbHandle,
    };

    let (pool, session) = extract(cx, |pool: Data<DbHandle>, session: Session| async move {
        (pool, session)
    })
    .await?;

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
        Ok(apub_id) => {
            leptos_actix::redirect(
                cx,
                if redirect_to == "/auth" {
                    "/"
                } else {
                    &redirect_to
                },
            );
            Ok(Ok(apub_id))
        }
        Err(LoginError::InternalServerError(e)) => Err(ServerFnError::ServerError(e)),
        Err(LoginError::BadRequest(e)) => Ok(Err(e)),
        Err(LoginError::Unauthorized(FormAuthError::Email)) => {
            Ok(Err("Email not found".to_string()))
        }
        Err(LoginError::Unauthorized(FormAuthError::Password)) => {
            Ok(Err("Wrong password".to_string()))
        }
        _ => unreachable!(),
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
                    minlength=8
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
    use actix_web::web;
    use leptos_actix::extract;
    use wordforge_api::{
        account::{self, FormAuthError, RegistrationError},
        util::AppState,
        DbHandle,
    };

    let (pool, state) = extract(
        cx,
        |pool: Data<DbHandle>, state: web::Data<AppState>| async move { (pool, state) },
    )
    .await?;

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
        Ok(_) => login(cx, email, password, client_app, client_website, redirect_to).await,
        Err(RegistrationError::BadRequest(e)) => Ok(Err(e)),
        Err(RegistrationError::Conflict(FormAuthError::Email)) => {
            Ok(Err("Email in use".to_string()))
        }
        Err(RegistrationError::Conflict(FormAuthError::Username)) => {
            Ok(Err("Username is taken".to_string()))
        }
        Err(RegistrationError::InternalServerError(e)) => Err(ServerFnError::ServerError(e)),
        _ => unreachable!(),
    }
}
