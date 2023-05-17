use crate::{
    fallback::*,
    routes::{auth::*, novel::*},
};
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ValidationResult {
    Ok(String),
    Unauthorized(String),
    Error(String),
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let validate = create_blocking_resource(cx, || (), move |_| validate(cx));
    let valid = move |cx| {
        validate
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())))
    };

    provide_context(cx, validate);
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/wordforge.css"/>
        <Link rel="icon" href="/favicon.svg"/>
        <Title text="Wordforge: Federated creative writing"/>
        <Router>
            <Overlay validator=validate>
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <Home/> }
                        }
                    />
                    <Route
                        path="/auth"
                        view=move |cx| {
                            view! { cx,
                                <Suspense fallback=|| ()>
                                    {move || match valid(cx) {
                                        None => ().into_view(cx),
                                        Some(ValidationResult::Ok(_)) => {
                                            view! { cx, <Redirect path="/"/> }
                                                .into_view(cx)
                                        }
                                        Some(ValidationResult::Unauthorized(e)) => {
                                            log!("Validation: {}", e);
                                            view! { cx, <Auth/> }
                                                .into_view(cx)
                                        }
                                        Some(ValidationResult::Error(e)) => {
                                            error!("ValidationResult::Error@app::Router: {}", e);
                                            view! { cx, <InternalErrorPage/> }
                                                .into_view(cx)
                                        }
                                    }}
                                </Suspense>
                            }
                        }
                    />
                    <Route
                        path="/create"
                        view=move |cx| {
                            view! { cx,
                                <Suspense fallback=|| ()>
                                    {move || match valid(cx) {
                                        None => ().into_view(cx),
                                        Some(ValidationResult::Ok(_)) => {
                                            view! { cx, <CreateBook/> }
                                                .into_view(cx)
                                        }
                                        Some(ValidationResult::Unauthorized(e)) => {
                                            log!("Validation: {}", e);
                                            view! { cx, <Redirect path="/auth"/> }
                                                .into_view(cx)
                                        }
                                        Some(ValidationResult::Error(e)) => {
                                            error!("ValidationResult::Error@app::Router: {}", e);
                                            view! { cx, <InternalErrorPage/> }
                                                .into_view(cx)
                                        }
                                    }}
                                </Suspense>
                            }
                        }
                    />
                    <Route
                        path="/novel/:uuid"
                        view=|cx| {
                            view! { cx, <NovelView/> }
                        }
                    />
                </Routes>
            </Overlay>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <p class="text-6xl">"EVENTS"</p>
        <p class="text-6xl">"RECOMMENDATIONS"</p>
    }
}

#[component]
fn Overlay(
    cx: Scope,
    validator: Resource<(), Result<ValidationResult, ServerFnError>>,
    children: Children,
) -> impl IntoView {
    let loc = use_location(cx);
    let redirect_path = create_memo(cx, move |_| {
        format!(
            "{}{}{}",
            loc.pathname.get(),
            loc.search.get(),
            loc.hash.get()
        )
    });
    create_effect(cx, move |prev| {
        loc.pathname.track();
        // Don't run on first load, only subsequent navigations
        if prev.is_some() {
            validator.refetch();
        }
    });

    view! { cx,
        <Body class="main-screen"/>
        <Topbar/>
        <div class="flex flex-row w-screen">
            <Sidebar validator=validator redirect_path=redirect_path/>
            <div class="sm:ml-60 overflow-y-auto w-full">{children(cx)}</div>
        </div>
        <BottomBar validator=validator redirect_path=redirect_path/>
    }
}

#[component]
fn Topbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="sticky top-0 w-screen z-40 dark:bg-gray-950 m-0 p-0 h-0 sm:h-auto sm:p-1 invisible sm:visible">
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
fn Sidebar(
    cx: Scope,
    validator: Resource<(), Result<ValidationResult, ServerFnError>>,
    redirect_path: Memo<String>,
) -> impl IntoView {
    let valid = create_memo(cx, move |_| {
        validator
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())))
    });

    view! { cx,
        <div class="fixed flex flex-none flex-col z-40 pt-1 pl-0.5 items-start text-xl align-top h-screen left-0 w-0 dark:bg-gray-700 invisible sm:w-60 sm:visible">
            <Transition fallback=move || {
                view! { cx,
                    <span class="m-1 w-[95%] p-2 rounded-md text-center cursor-wait dark:bg-purple-600 hover:dark:bg-purple-700">
                        <Icon
                            icon=CgIcon::CgSpinner
                            class="dark:stroke-white py-1 w-10 h-10 m-auto animate-spin pointer-events-none"
                        />
                    </span>
                }
                    .into_view(cx)
            }>
                {move || {
                    match valid() {
                        None => {
                            view! { cx,
                                <span class="m-1 w-[95%] p-2 rounded-md text-center cursor-wait dark:bg-purple-600 hover:dark:bg-purple-700">
                                    <Icon
                                        icon=CgIcon::CgSpinner
                                        class="dark:stroke-white py-1 w-10 h-10 m-auto animate-spin pointer-events-none"
                                    />
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
                        Some(ValidationResult::Unauthorized(e)) => {
                            log!("Validation: {}", e);
                            view! { cx,
                                <A
                                    href=format!("/auth?redirect_to={}", redirect_path())
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
            <A href="/explore" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Local"
            </A>
            <A href="/explore/public" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Public"
            </A>
        </div>
    }
}

#[component]
fn BottomBar(
    cx: Scope,
    validator: Resource<(), Result<ValidationResult, ServerFnError>>,
    redirect_path: Memo<String>,
) -> impl IntoView {
    let valid = create_memo(cx, move |_| {
        validator
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())))
    });

    view! { cx,
        <div class="fixed flex flex-row z-40 max-h-40 justify-around overflow-hidden bottom-0 mt-auto w-screen m-0 p-1 visible sm:invisible dark:bg-gray-950">
            <A href="/">
                <Icon
                    icon=OcIcon::OcHomeLg
                    class="dark:stroke-white py-1 w-10 h-10 stroke-0 my-auto"
                />
            </A>
            <A href="/explore">
                <Icon
                    icon=OcIcon::OcGlobeLg
                    class="dark:stroke-white py-1 w-10 h-10 stroke-0 my-auto"
                />
            </A>
            <Icon
                icon=OcIcon::OcSearchLg
                class="dark:stroke-white py-1 w-10 h-10 stroke-0 my-auto cursor-pointer"
            />
            <Transition fallback=|| ()>
                {move || {
                    match valid() {
                        None => {
                            view! { cx,
                                <Icon
                                    icon=CgIcon::CgSpinner
                                    class="dark:stroke-white py-1 w-10 h-10 m-auto animate-spin cursor-wait"
                                />
                            }
                                .into_view(cx)
                        }
                        Some(ValidationResult::Ok(_)) => {
                            view! { cx,
                                <A href="/create">
                                    <Icon
                                        icon=OcIcon::OcPencilLg
                                        class="dark:stroke-white w-10 h-10 py-1 my-auto stroke-0 cursor-pointer"
                                    />
                                </A>
                                <span class="w-8 h-8 my-auto rounded-full cursor-pointer bg-pink-500">
                                    <p class="hidden">"TODO: Account/Profile flyout menu"</p>
                                </span>
                            }
                                .into_view(cx)
                        }
                        Some(ValidationResult::Unauthorized(e)) => {
                            log!("Validation: {}", e);
                            view! { cx,
                                <A href=format!("/auth?redirect_to={}", redirect_path())>
                                    <Icon
                                        icon=OcIcon::OcPersonAddLg
                                        class="dark:stroke-white py-1 w-10 h-10 my-auto stroke-0 cursor-pointer"
                                    />
                                </A>
                            }
                                .into_view(cx)
                        }
                        Some(ValidationResult::Error(e)) => {
                            error!("ValidationResult::Error@app::BottomBar: {}", e);
                            view! { cx,
                                <Icon
                                    icon=OcIcon::OcCircleSlashLg
                                    class="dark:stroke-white py-1 w-10 h-10 my-auto stroke-0"
                                />
                            }
                                .into_view(cx)
                        }
                    }
                }}
            </Transition>
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
        UserValidateResult::Unauthorized(v) => Ok(ValidationResult::Unauthorized(v)),
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
