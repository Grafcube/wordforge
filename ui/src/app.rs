use crate::{
    components::panel::*,
    fallback::*,
    routes::{auth::*, novel::*},
};
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ValidationError {
    Unauthorized(String),
    Error(String),
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let validate = create_blocking_resource(cx, || (), move |_| validate(cx));
    let valid = move |cx| {
        validate
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| Err(ValidationError::Error(e.to_string()))))
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
                                        Some(Ok(_)) => {
                                            view! { cx, <Redirect path="/"/> }
                                                .into_view(cx)
                                        }
                                        Some(Err(ValidationError::Unauthorized(e))) => {
                                            log!("Validation: {}", e);
                                            view! { cx, <Auth/> }
                                                .into_view(cx)
                                        }
                                        Some(Err(ValidationError::Error(e))) => {
                                            error!("ValidationError::Error@app::Router: {}", e);
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
                                        Some(Ok(_)) => {
                                            view! { cx, <CreateBook/> }
                                                .into_view(cx)
                                        }
                                        Some(Err(ValidationError::Unauthorized(e))) => {
                                            log!("Validation: {}", e);
                                            view! { cx, <Redirect path="/auth"/> }
                                                .into_view(cx)
                                        }
                                        Some(Err(ValidationError::Error(e))) => {
                                            error!("ValidationError::Error@app::Router: {}", e);
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
    validator: Resource<(), Result<Result<(String, String), ValidationError>, ServerFnError>>,
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
    let logout = create_action(cx, move |_: &()| logout(cx));
    let logout_res = logout.value();

    create_effect(cx, move |_| {
        loc.pathname.track();
        logout_res.track();
        validator.refetch();
    });

    create_effect(cx, move |_| match logout_res() {
        None => (),
        Some(Err(e)) => error!("{}", e.to_string()),
        Some(Ok(_)) => {
            if let Err(e) = use_navigate(cx)("/", NavigateOptions::default()) {
                error!("{}", e.to_string());
            }
        }
    });

    view! { cx,
        <Body class="main-screen"/>
        <div class="flex flex-row mx-auto max-w-[100rem]">
            <Sidebar validator=validator logout=logout redirect_path=redirect_path/>
            <div class="grow mb-12 md:mb-0 overflow-y-auto w-full">{children(cx)}</div>
        </div>
        <BottomBar validator=validator logout=logout redirect_path=redirect_path/>
    }
}

#[component]
fn Sidebar(
    cx: Scope,
    validator: Resource<(), Result<Result<(String, String), ValidationError>, ServerFnError>>,
    logout: Action<(), Result<(), ServerFnError>>,
    redirect_path: Memo<String>,
) -> impl IntoView {
    let valid = create_memo(cx, move |_| {
        validator
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| Err(ValidationError::Error(e.to_string()))))
    });
    let (name, set_name) = create_signal::<Option<String>>(cx, None);
    let panel = create_rw_signal(cx, false);

    view! { cx,
        <div class="md:flex flex-none flex-col p-2 items-start text-xl align-top h-screen w-0 hidden md:w-60">
            <A href="/" class="flex flex-row gap-2 w-full p-2 rounded-md hover:dark:bg-gray-800">
                <Icon
                    icon=OcIcon::OcHomeLg
                    class="dark:stroke-white py-1 w-10 h-10 stroke-0 my-auto"
                />
                <span class="my-auto">"Home"</span>
            </A>
            <A
                href="/explore"
                class="flex flex-row gap-2 w-full p-2 rounded-md hover:dark:bg-gray-800"
            >
                <Icon icon=LuIcon::LuComponent class="dark:stroke-white py-1 w-10 h-10 my-auto"/>
                <span class="my-auto">"Local"</span>
            </A>
            <A
                href="/explore/public"
                class="flex flex-row gap-2 w-full p-2 rounded-md hover:dark:bg-gray-800"
            >
                <Icon
                    icon=OcIcon::OcGlobeLg
                    class="dark:stroke-white py-1 w-10 h-10 stroke-0 my-auto"
                />
                <span class="my-auto">"Public"</span>
            </A>
            <span class="my-auto"></span>
            <div class="w-full">
                <Transition fallback=move || {
                    view! { cx,
                        <span class="w-full p-2 rounded-md text-center cursor-wait dark:bg-purple-600 hover:dark:bg-purple-700">
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
                                    <span class="w-full p-2 rounded-md text-center cursor-wait dark:bg-purple-600 hover:dark:bg-purple-700">
                                        <Icon
                                            icon=CgIcon::CgSpinner
                                            class="dark:stroke-white py-1 w-10 h-10 m-auto animate-spin pointer-events-none"
                                        />
                                    </span>
                                }
                                    .into_view(cx)
                            }
                            Some(Ok(_)) => {
                                view! { cx,
                                    <A
                                        href="/create"
                                        class="flex flex-row gap-2 w-full p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700"
                                    >
                                        <Icon
                                            icon=OcIcon::OcPencilLg
                                            class="dark:stroke-white w-10 h-10 py-1 my-auto stroke-0 cursor-pointer"
                                        />
                                        <span class="my-auto">"Create new book"</span>
                                    </A>
                                }
                                    .into_view(cx)
                            }
                            Some(Err(ValidationError::Unauthorized(e))) => {
                                log!("Validation: {}", e);
                                view! { cx,
                                    <A
                                        href=format!("/auth?redirect_to={}", redirect_path())
                                        class="flex flex-row gap-2 w-full p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700"
                                    >
                                        <Icon
                                            icon=OcIcon::OcPersonAddLg
                                            class="dark:stroke-white py-1 w-10 h-10 my-auto stroke-0 cursor-pointer"
                                        />
                                        <span class="my-auto">"Sign in / Sign up"</span>
                                    </A>
                                }
                                    .into_view(cx)
                            }
                            Some(Err(ValidationError::Error(e))) => {
                                error!("ValidationError::Error@app::Sidebar: {}", e);
                                view! { cx,
                                    <span class="w-full p-2 rounded-md text-center dark:bg-gray-600 hover:dark:bg-gray-700">
                                        "Something went wrong"
                                    </span>
                                }
                                    .into_view(cx)
                            }
                        }
                    }}
                </Transition>
            </div>
            <Transition fallback=|| ()>
                <Show
                    when=move || {
                        if let Some(Ok((_, name))) = valid() {
                            set_name(Some(name));
                            true
                        } else {
                            false
                        }
                    }
                    fallback=|_| ()
                >
                    <button
                        on:click=move |_| panel.set(!panel())
                        class="flex flex-row gap-3 w-full mt-2 p-2 rounded-md hover:dark:bg-gray-800"
                    >
                        <span class="flex-none w-10 h-10 my-auto rounded-full bg-pink-500"></span>
                        <span class="my-auto w-full rounded-md text-left whitespace-nowrap overflow-hidden overflow-ellipsis">
                            {name()}
                        </span>
                        <Show
                            when=panel
                            fallback=move |cx| {
                                view! { cx,
                                    <Icon
                                        icon=HiIcon::HiChevronUpSolidLg
                                        class="dark:stroke-white my-auto ml-auto h-8 w-8 pointer-events-none"
                                    />
                                }
                            }
                        >
                            <Icon
                                icon=HiIcon::HiChevronDownSolidLg
                                class="dark:stroke-white my-auto ml-auto h-8 w-8 pointer-events-none"
                            />
                        </Show>
                        <Panel
                            when=panel
                            class="absolute flex flex-col bottom-[4.5rem] -mx-2 p-2 w-56 dark:bg-gray-800 rounded-md"
                        >
                            <button
                                class="flex flex-row gap-2 my-auto text-left w-full p-2 rounded-md hover:dark:bg-gray-700"
                                on:click=move |_| logout.dispatch(())
                            >
                                <Icon
                                    icon=OcIcon::OcSignOutLg
                                    class="dark:stroke-white w-8 h-8 my-auto stroke-0 pointer-events-none"
                                />
                                <span class="my-auto">"Logout"</span>
                            </button>
                        </Panel>
                    </button>
                </Show>
            </Transition>
        </div>
    }
}

#[component]
fn BottomBar(
    cx: Scope,
    validator: Resource<(), Result<Result<(String, String), ValidationError>, ServerFnError>>,
    logout: Action<(), Result<(), ServerFnError>>,
    redirect_path: Memo<String>,
) -> impl IntoView {
    let valid = create_memo(cx, move |_| {
        validator
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| Err(ValidationError::Error(e.to_string()))))
    });
    let panel = create_rw_signal(cx, false);

    view! { cx,
        <div class="fixed bottom-0">
            <Panel when=panel class="p-2 rounded-t-xl w-full dark:bg-gray-950">
                <button
                    class="relative flex flex-row gap-3 my-auto text-left w-full p-3 rounded-md hover:dark:bg-gray-900"
                    on:click=move |_| {
                        logout.dispatch(());
                        panel.set(false);
                    }
                >
                    <Icon
                        icon=OcIcon::OcSignOutLg
                        class="dark:stroke-white w-6 h-6 my-auto stroke-0 pointer-events-none"
                    />
                    <span class="my-auto">"Logout"</span>
                </button>
            </Panel>
            <div class="flex flex-row max-h-12 justify-around z-40 overflow-hidden mt-auto w-screen m-0 p-1 md:hidden dark:bg-gray-950">
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
                            Some(Ok(_)) => {
                                view! { cx,
                                    <A href="/create">
                                        <Icon
                                            icon=OcIcon::OcPencilLg
                                            class="dark:stroke-white w-10 h-10 py-1 my-auto stroke-0 cursor-pointer"
                                        />
                                    </A>
                                    <button
                                        on:click=move |_| panel.set(!panel())
                                        class="w-8 h-8 my-auto rounded-full bg-pink-500"
                                    ></button>
                                }
                                    .into_view(cx)
                            }
                            Some(Err(ValidationError::Unauthorized(e))) => {
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
                            Some(Err(ValidationError::Error(e))) => {
                                error!("ValidationError::Error@app::BottomBar: {}", e);
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
        </div>
    }
}

#[server(UserValidate, "/server")]
async fn validate(cx: Scope) -> Result<Result<(String, String), ValidationError>, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_session::Session;
    use leptos_actix::extract;
    use wordforge_api::{
        account::{self, UserValidateError},
        DbHandle,
    };

    let (pool, session) = extract(cx, |pool: Data<DbHandle>, session: Session| async move {
        (pool, session)
    })
    .await?;

    Ok(account::validate(pool.app_data().as_ref(), session)
        .await
        .map_err(|e| match e {
            UserValidateError::Unauthorized(v) => ValidationError::Unauthorized(v),
            UserValidateError::InternalServerError(v) => ValidationError::Error(v),
            UserValidateError::NotFound(v) => ValidationError::Error(v),
        }))
}

#[server(Logout, "/server")]
async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    leptos_actix::extract(cx, |session: actix_session::Session| async move {
        session.purge();
    })
    .await
}
