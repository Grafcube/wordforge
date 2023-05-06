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
    Ok(String),
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
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=|cx| {
                        view! { cx,
                            <Overlay>
                                <Home/>
                            </Overlay>
                        }
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
                                    Some(ValidationResult::Ok(_)) => {
                                        view! { cx, <Redirect path="/"/> }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Unauthorized) => {
                                        view! { cx,
                                            <Overlay>
                                                <Auth/>
                                            </Overlay>
                                        }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Error(e)) => {
                                        error!("ValidationResult::Error@app::Router: {}", e);
                                        view! { cx,
                                            <Overlay>
                                                <InternalErrorPage/>
                                            </Overlay>
                                        }
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
                                    Some(ValidationResult::Ok(_)) => {
                                        view! { cx,
                                            <Overlay>
                                                <CreateBook/>
                                            </Overlay>
                                        }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Unauthorized) => {
                                        view! { cx, <Redirect path="/auth"/> }
                                            .into_view(cx)
                                    }
                                    Some(ValidationResult::Error(e)) => {
                                        error!("ValidationResult::Error@app::Router: {}", e);
                                        view! { cx,
                                            <Overlay>
                                                <InternalErrorPage/>
                                            </Overlay>
                                        }
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
                        view! { cx,
                            <Overlay>
                                <NovelView/>
                            </Overlay>
                        }
                    }
                    ssr=SsrMode::Async
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="items-center overflow-auto text-center">
            <p class="mx-auto text-6xl">"EVENTS"</p>
            <p class="mx-auto text-6xl">"RECOMMENDATIONS"</p>
        </div>
    }
}

#[component]
fn Overlay(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <Body class="main-screen"/>
        <Topbar/>
        <div class="fixed flex flex-row w-full">
            <Sidebar/>
            {children(cx)}
        </div>
    }
}

#[component]
fn Topbar(cx: Scope) -> impl IntoView {
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
    let validator = create_resource(cx, || (), move |_| validate(cx));

    view! { cx,
        <div class="sticky flex flex-col items-start p-1 text-xl align-top h-screen left-0 top-0 w-60 dark:bg-gray-700">
            <Transition fallback=|| ()>
                {move || {
                    let text = validator
                        .read(cx)
                        .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())));
                    match text {
                        None => {
                            view! { cx,
                                <span class="m-1 w-[95%] p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700">
                                    "Spinner"
                                </span>
                            }
                                .into_view(cx)
                        }
                        Some(ValidationResult::Ok(_)) => {
                            view! { cx,
                                <A
                                    href="/create"
                                    class="m-1 w-[95%] p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700"
                                >
                                    "Create new book"
                                </A>
                            }
                                .into_view(cx)
                        }
                        Some(ValidationResult::Unauthorized) => {
                            view! { cx,
                                <A
                                    href="/auth"
                                    class="m-1 w-[95%] p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700"
                                >
                                    "Sign in / Sign up"
                                </A>
                            }
                                .into_view(cx)
                        }
                        Some(ValidationResult::Error(e)) => {
                            error!("ValidationResult::Error@app::Sidebar: {}", e);
                            view! { cx,
                                <span class="m-1 w-[95%] p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700">
                                    "Something went wrong"
                                </span>
                            }
                                .into_view(cx)
                        }
                    }
                }}
            </Transition>
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
        UserValidateResult::Ok(apub_id) => Ok(ValidationResult::Ok(apub_id)),
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
