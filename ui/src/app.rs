use crate::{
    components::{auth::*, novel::*},
    fallback::*,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ValidationResult {
    Ok,
    Unauthorized,
    Error(String),
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let validate = create_resource(cx, || (), move |_| validate(cx));
    let check_validate = move |cx| {
        validate
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())))
    };

    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/wordforge.css"/>
        <Link rel="icon" href="/favicon.svg"/>
        <Title text="Wordforge: Federated creative writing"/>
        <Router fallback=|cx| {
            view! { cx, <NotFoundPage/> }
                .into_view(cx)
        }>
            <Routes>
                <Route
                    path="/"
                    view=|cx| {
                        view! { cx, <Home/> }
                    }
                    ssr=SsrMode::Async
                />
                <Route
                    path="/auth"
                    view=move |cx| {
                        view! { cx,
                            <Suspense fallback=|| ()>
                                {match check_validate(cx) {
                                    None => ().into_view(cx),
                                    Some(ValidationResult::Ok) => {
                                        view! { cx, <Redirect path="/"/> }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Unauthorized) => {
                                        view! { cx, <Auth/> }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Error(e)) => {
                                        error!("ValidationResult::Error@app::Router: {}", e);
                                        view! { cx, <span class="text-red-900">"Something went wrong"</span> }
                                            .into_view(cx)
                                    }
                                }}
                            </Suspense>
                        }
                    }
                    ssr=SsrMode::Async
                />
                <Route
                    path="/create"
                    view=move |cx| {
                        view! { cx,
                            <Suspense fallback=|| ()>
                                {match check_validate(cx) {
                                    None => ().into_view(cx),
                                    Some(ValidationResult::Ok) => {
                                        view! { cx, <CreateBook/> }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Unauthorized) => {
                                        view! { cx, <Redirect path="/auth"/> }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Error(e)) => {
                                        error!("ValidationResult::Error@app::Router: {}", e);
                                        view! { cx, <span class="text-red-900">"Something went wrong"</span> }
                                            .into_view(cx)
                                    }
                                }}
                            </Suspense>
                        }
                    }
                    ssr=SsrMode::Async
                />
                <Route
                    path="/novel/:uuid"
                    view=|cx| {
                        view! { cx, <NovelView/> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <Body class="main-screen"/>
        <Topbar/>
        <div class="fixed flex flex-row">
            <Sidebar/>
            <div class="items-center overflow-auto">
                <p class="mx-auto text-6xl">"EVENTS"</p>
                <p class="mx-auto text-6xl">"RECOMMENDATIONS"</p>
            </div>
        </div>
    }
}

#[component]
pub fn Topbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="sticky top-0 w-screen dark:bg-gray-950 m-0 p-1">
            <A href="/" class="m-2 px-2 w-fit flex items-start align-middle">
                <img
                    src="/favicon.svg"
                    alt="Home"
                    width="40"
                    height="40"
                    class="mx-1 my-auto invert dark:invert-0"
                />
                <h1 class="mx-1 my-auto text-3xl text-left">"Wordforge"</h1>
            </A>
        </div>
    }
}

#[component]
fn Sidebar(cx: Scope) -> impl IntoView {
    let (action_target, set_action_target) = create_signal(cx, "/".to_string());
    let action = create_resource(cx, || (), move |_| validate(cx));

    view! { cx,
        <div class="sticky flex flex-col items-start p-1 text-xl align-top h-screen left-0 top-0 w-60 dark:bg-gray-700">
            <A
                href=action_target
                class="m-1 w-[95%] p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700"
            >
                <Suspense fallback=move || "Spinner">
                    {move || {
                        let text = action
                            .read(cx)
                            .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())));
                        let text = match text {
                            None => return "Spinner".to_string(),
                            Some(v) => v,
                        };
                        match text {
                            ValidationResult::Ok => {
                                set_action_target("/create".to_string());
                                "Create new book".to_string()
                            }
                            ValidationResult::Unauthorized => {
                                set_action_target("/auth".to_string());
                                "Sign in / Sign up".to_string()
                            }
                            ValidationResult::Error(e) => {
                                set_action_target("/".to_string());
                                error!("ValidationResult::Error@app::Sidebar: {}", e);
                                "Something went wrong".to_string()
                            }
                        }
                    }}
                </Suspense>
            </A>
            <A href="/" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Home"
            </A>
            <A href="/local" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Local"
            </A>
            <A href="/public" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Public"
            </A>
        </div>
    }
}

#[server(UserValidate, "/server")]
async fn validate(cx: Scope) -> Result<ValidationResult, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_web::http::StatusCode;
    use leptos_actix::ResponseOptions;
    use wordforge_api::{
        account::{self, UserValidateResult},
        DbHandle,
    };

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<DbHandle> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let session = <actix_session::Session as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match account::validate(pool.app_data().as_ref(), session).await {
        UserValidateResult::Ok(_) => Ok(ValidationResult::Ok),
        UserValidateResult::Unauthorized(v) => {
            log!("{}", v);
            Ok(ValidationResult::Unauthorized)
        }
        UserValidateResult::InternalServerError(v) => {
            resp.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            Ok(ValidationResult::Error(v))
        }
        UserValidateResult::NotFound(v) => {
            resp.set_status(StatusCode::NOT_FOUND);
            Ok(ValidationResult::Error(v))
        }
    }
}
