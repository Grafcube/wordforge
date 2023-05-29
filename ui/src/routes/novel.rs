use crate::{
    app::ValidationResult,
    components::{basicinput::*, errorview::*, listbox::*, toggle::*},
    fallback::*,
    path::NovelViewParams,
};
use isolang::Language;
use leptos::{ev::KeyboardEvent, html::*, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn CreateBook(cx: Scope) -> impl IntoView {
    let summary = create_node_ref::<Textarea>(cx);
    let cw = create_node_ref::<Input>(cx);

    let (title, set_title) = create_signal(cx, String::new());
    let (tags, set_tags) = create_signal(cx, String::new());
    let (errormsg, set_errormsg) = create_signal(cx, String::new());

    let create = create_server_action::<CreateNovel>(cx);
    let response = create.value();
    let err = move || {
        response.get().map(|v| match v {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => set_errormsg(e),
            Err(e) => set_errormsg(e.to_string()),
        })
    };

    let genres = create_resource(cx, || (), move |_| get_genres());
    let roles = create_resource(cx, || (), move |_| get_roles());
    let langs = create_resource(cx, || (), move |_| get_langs());

    let genre = create_rw_signal(cx, String::new());
    let role = create_rw_signal(cx, String::new());
    let lang = create_rw_signal(cx, String::new());
    let sensitive = create_rw_signal(cx, false);

    let line_input_handler = move |ev, setter: WriteSignal<String>| {
        let re = regex::Regex::new(r#"[\r\n]+"#).unwrap();
        let value = event_target_value(&ev);
        let value = re.replace_all(&value, "");
        setter(value.to_string());
        let target = event_target::<web_sys::HtmlElement>(&ev);
        let style = target.style();
        style.set_property("height", "auto").unwrap();
        style
            .set_property("height", &format!("{}px", target.scroll_height()))
            .unwrap();
    };

    view! { cx,
        <Title text="Create a new book"/>
        <div class="mx-auto w-full">
            <h1 class="p-2 text-3xl text-center">"Create a new book"</h1>
            <div class="flex justify-center text-center place-content-center items-center">
                <ActionForm action=create class="space-y-4 p-4 w-full max-w-xl">
                    <div class="relative">
                        <textarea
                            class="basic-input max-h-40 overflow-y-auto resize-none peer"
                            placeholder=" "
                            name="title"
                            rows=1
                            wrap="soft"
                            on:keydown=move |ev: KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    ev.prevent_default();
                                    summary().unwrap().focus().unwrap();
                                }
                            }
                            prop:value=title
                            on:input=move |ev| line_input_handler(ev, set_title)
                            on:paste=move |ev| line_input_handler(ev, set_title)
                            required
                        ></textarea>
                        <FloatingLabel target="title">"Title"</FloatingLabel>
                    </div>
                    <div class="relative">
                        <textarea
                            class="basic-input peer"
                            placeholder=" "
                            node_ref=summary
                            name="summary"
                        ></textarea>
                        <FloatingLabel target="summary">"Summary"</FloatingLabel>
                    </div>
                    <input type="hidden" name="genre" value=move || genre.get()/>
                    <Transition fallback=|| ()>
                        {move || match genres.read(cx) {
                            None => {
                                view! { cx,
                                    <Icon
                                        icon=CgIcon::CgSpinner
                                        class="block dark:stroke-white py-1 w-10 h-10 mx-auto animate-spin pointer-events-none"
                                    />
                                }
                                    .into_view(cx)
                            }
                            Some(Ok(items)) => {
                                view! { cx,
                                    <FilterListbox
                                        option=genre
                                        name="genre"
                                        label="Genre"
                                        initial="Select a genre"
                                        items=items
                                    />
                                }
                                    .into_view(cx)
                            }
                            Some(Err(e)) => {
                                error!("{}", e.to_string());
                                view! { cx, <span>"Something went wrong"</span> }
                                    .into_view(cx)
                            }
                        }}
                    </Transition>
                    <input type="hidden" name="role" value=move || role.get()/>
                    <Transition fallback=|| ()>
                        {move || match roles.read(cx) {
                            None => {
                                view! { cx,
                                    <Icon
                                        icon=CgIcon::CgSpinner
                                        class="block dark:stroke-white py-1 w-10 h-10 mx-auto animate-spin pointer-events-none"
                                    />
                                }
                                    .into_view(cx)
                            }
                            Some(Ok(items)) => {
                                view! { cx,
                                    <FilterListbox
                                        option=role
                                        name="role"
                                        label="Your role"
                                        initial="Select your role"
                                        items=items
                                    />
                                }
                                    .into_view(cx)
                            }
                            Some(Err(e)) => {
                                error!("{}", e.to_string());
                                view! { cx, <span>"Something went wrong"</span> }
                                    .into_view(cx)
                            }
                        }}
                    </Transition>
                    <input type="hidden" name="lang" value=move || lang.get()/>
                    <Transition fallback=|| ()>
                        {move || match langs.read(cx) {
                            None => {
                                view! { cx,
                                    <Icon
                                        icon=CgIcon::CgSpinner
                                        class="block dark:stroke-white py-1 w-10 h-10 mx-auto animate-spin pointer-events-none"
                                    />
                                }
                                    .into_view(cx)
                            }
                            Some(Ok(items)) => {
                                view! { cx,
                                    <FilterListbox
                                        option=lang
                                        name="lang"
                                        label="Language"
                                        initial="Select the book's language"
                                        items=items
                                    />
                                }
                                    .into_view(cx)
                            }
                            Some(Err(e)) => {
                                error!("{}", e.to_string());
                                view! { cx, <span>"Something went wrong"</span> }
                                    .into_view(cx)
                            }
                        }}
                    </Transition>
                    <div class="relative">
                        <textarea
                            class="basic-input max-h-40 overflow-y-auto resize-none peer"
                            placeholder=" "
                            name="tags"
                            rows=1
                            wrap="soft"
                            on:keydown=move |ev: KeyboardEvent| {
                                if ev.key() == "Enter" {
                                    ev.prevent_default();
                                    cw().unwrap().focus().unwrap();
                                }
                            }
                            prop:value=tags
                            on:input=move |ev| line_input_handler(ev, set_tags)
                            on:paste=move |ev| line_input_handler(ev, set_tags)
                        ></textarea>
                        <FloatingLabel target="tags">"Tags"</FloatingLabel>
                    </div>
                    <input type="hidden" name="cw" value=move || sensitive().to_string()/>
                    <div class="flex justify-start">
                        <Toggle value=sensitive node_ref=cw>
                            "Content warning"
                        </Toggle>
                    </div>
                    <button class="button-1" type="submit">
                        "Create"
                    </button>
                </ActionForm>
            </div>
            <div class="flex mx-auto text-2xl m-4 justify-center text-center">
                <ErrorView message=errormsg/>
                {err}
            </div>
        </div>
    }
}

#[server(CreateNovel, "/server")]
pub async fn create_novel(
    cx: Scope,
    title: String,
    summary: String,
    genre: String,
    role: String,
    lang: String,
    tags: String,
    cw: bool,
) -> Result<Result<(), String>, ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_session::Session;
    use actix_web::{http::StatusCode, web};
    use leptos_actix::{extract, ResponseOptions};
    use std::str::FromStr;
    use wordforge_api::{
        api::novel::{self, CreateNovelResult, NewNovel},
        enums::*,
        util::AppState,
        DbHandle,
    };

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let (pool, state, session) = extract(
        cx,
        |pool: Data<DbHandle>, state: web::Data<AppState>, session: Session| async move {
            (pool, state, session)
        },
    )
    .await?;

    let info = NewNovel {
        title,
        summary,
        genre: match Genres::from_str(&genre) {
            Ok(g) => g,
            Err(_) => {
                resp.set_status(StatusCode::BAD_REQUEST);
                return Ok(Err("Select a genre".to_string()));
            }
        },
        role: match Roles::from_str(&role) {
            Ok(r) => r,
            Err(_) => {
                resp.set_status(StatusCode::BAD_REQUEST);
                return Ok(Err("Select your role".to_string()));
            }
        },
        lang,
        sensitive: cw,
        tags,
    };

    match novel::create_novel(state, pool, session, info).await {
        CreateNovelResult::Ok(id) => Ok(Ok(leptos_actix::redirect(cx, &format!("/novel/{}", id)))),
        CreateNovelResult::InternalServerError(e) => Err(ServerFnError::ServerError(e)),
        CreateNovelResult::Unauthorized(e) => {
            resp.set_status(StatusCode::UNAUTHORIZED);
            Ok(Err(e))
        }
        CreateNovelResult::BadRequest(e) => {
            resp.set_status(StatusCode::BAD_REQUEST);
            Ok(Err(e))
        }
    }
}

#[server(GetGenres, "/server")]
pub async fn get_genres() -> Result<Vec<String>, ServerFnError> {
    use strum::IntoEnumIterator;
    use wordforge_api::enums::Genres;

    Ok(Genres::iter().map(|g| g.to_string()).collect())
}

#[server(GetRoles, "/server")]
pub async fn get_roles() -> Result<Vec<String>, ServerFnError> {
    use strum::IntoEnumIterator;
    use wordforge_api::enums::Roles;

    Ok(Roles::iter().map(|g| g.to_string()).collect())
}

#[server(GetLangs, "/server")]
pub async fn get_langs() -> Result<Vec<String>, ServerFnError> {
    Ok(isolang::languages()
        .filter_map(|lang| lang.to_639_1().map(|_| lang.to_name().to_string()))
        .collect())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Author {
    pub apub_id: String,
    pub role: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Novel {
    name: String,
    summary: String,
    authors: Vec<Author>,
    genre: String,
    tags: Vec<String>,
    language: Language,
    sensitive: bool,
    published: String,
}

#[component]
pub fn NovelView(cx: Scope) -> impl IntoView {
    let params = use_params::<NovelViewParams>(cx);
    let uuid = move || {
        params
            .with(|params| params.clone().map(|p| p.uuid))
            .unwrap()
    };

    let novel = create_resource(cx, uuid, move |id| get_novel(cx, id));
    let (authors, set_authors) = create_signal::<Vec<(String, String)>>(cx, Vec::new());
    let usernames = create_resource(cx, authors, move |authors| get_usernames(cx, authors));

    let author_view = move || {
        usernames.read(cx).map(|v| match v {
            Err(e) => {
                error!("usernames: {}", e.to_string());
                view! { cx, <span class="dark:bg-gray-800 rounded-xl px-4 py-2 my-2">"Something went wrong"</span> }.into_view(cx)
            }
            Ok(users) => {
                view! { cx,
                    <div class="dark:bg-gray-800 rounded-xl px-4 py-2 my-2">
                        <span class="text-gray-500 dark:text-gray-400">"Authors"</span>
                        <div class="flex flex-row justify-start flex-wrap gap-1 text-xl md:text-base overflow-auto max-h-40">
                            {users
                                .into_iter()
                                .map(|res| match res {
                                    (apub_id, role, Err(e)) => {
                                        log!("UNKNOWN: {e}");
                                        view! { cx,
                                            <a href=apub_id class="flex flex-col gap-0 dark:bg-gray-950 rounded-full px-4">
                                                <span class="my-auto text-sm md:text-xs dark:text-gray-500">{role}</span>
                                                <span class="my-auto dark:text-gray-400">"UNKNOWN"</span>
                                            </a>
                                        }
                                    }
                                    (apub_id, role, Ok(v)) => {
                                        view! { cx,
                                            <a href=apub_id class="flex flex-row gap-1 dark:bg-gray-950 rounded-full pr-4">
                                                <span class="w-6 h-6 ml-2 m-auto rounded-full bg-pink-500"></span>
                                                <div class="flex flex-col gap-0">
                                                    {role
                                                        .ne("None")
                                                        .then_some(
                                                            view! { cx, <span class="my-auto text-sm md:text-xs dark:text-gray-500">{role}</span> },
                                                        )} <span class="my-auto">{v}</span>
                                                </div>
                                            </a>
                                        }
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            }
            .into_view(cx),
        })
    };

    let metadata = move || {
        novel.read(cx).map(|v| match v {
            Ok(Err(e)) => {
                log!("novel view: {e}");
                view! { cx, <NotFoundPage/> }.into_view(cx)
            }
            Err(e) => {
                error!("novel server fn: {}", e.to_string());
                view! { cx, <InternalErrorPage/> }.into_view(cx)
            }
            Ok(Ok(novel)) => view! { cx,
                                 <h1 class="text-center p-2 text-3xl">{&novel.name}</h1>
                                 <Title text=novel.name.clone()/>
                                 <div class="flex flex-row overflow-auto whitespace-nowrap gap-1 text-xl md:text-base">
                                     <a
                                         class="dark:bg-gray-800 rounded-full px-2 py-1"
                                         href={
                                             let re = regex::Regex::new(r#"\s+"#).unwrap();
                                             let path = re.replace_all(&novel.genre.to_lowercase(), "_").to_string();
                                             format!("/explore/{path}")
                                         }
                                     >
                                         {&novel.genre}
                                     </a>
                                     <span class="dark:bg-gray-800 rounded-full px-2 py-1">
                                         {novel.language.to_name()}
                                     </span>
                                     <Show
                                         when={
                                             let sensitive = novel.sensitive;
                                             move || sensitive
                                         }
                                         fallback=|_| ()
                                     >
                                         <span class="dark:bg-red-600 rounded-full px-2 py-1">
                                             "Content warning"
                                         </span>
                                     </Show>
                                 </div>
                                 <div class="dark:bg-gray-800 rounded-xl text-xl md:text-base overflow-auto max-h-40 my-2 px-4 py-2">
                                     {novel
                                         .summary
                                         .lines()
                                         .map(|line| {
                                             view! { cx, <p>{line.to_string()}</p> }
                                                 .into_view(cx)
                                         })
                                         .collect::<Vec<_>>()}
                                 </div>
                                 <div>
                                     {
                                         let author_list = novel
                                             .authors
                                             .into_iter()
                                             .map(|a| (a.apub_id, a.role))
                                             .collect::<Vec<_>>();
                                         set_authors(author_list);
                                         author_view
                                     }
                                 </div>
                                 <div class="flex flex-row justify-start gap-2 h-fit whitespace-nowrap overflow-x-auto overflow-y-hidden">
                                     {novel
                                         .tags
                                         .iter()
                                         .map(|tag| {
                                             view! { cx,
                                                 <a
                                                     href=format!("/explore/tags/{tag}")
                                                     class="italic mb-2 mt-auto dark:text-gray-500 dark:hover:text-gray-400 rounded-full text-xl md:text-base"
                                                 >
                                                     {format!("#{tag}")}
                                                 </a>
                                             }
                                                 .into_view(cx)
                                         })
                                         .collect::<Vec<_>>()}
                                 </div>
                             }
            .into_view(cx),
        })
    };

    let validate =
        use_context::<Resource<(), Result<ValidationResult, ServerFnError>>>(cx).unwrap();
    let valid = create_memo(cx, move |_| {
        validate
            .read(cx)
            .map(|resp| resp.unwrap_or_else(|e| ValidationResult::Error(e.to_string())))
    });

    view! { cx,
        <Title text="Novel"/>
        <div class="mx-auto max-w-2xl px-4">
            <Suspense fallback=move || {
                view! { cx,
                    <Icon
                        icon=CgIcon::CgSpinner
                        class="dark:stroke-white py-1 w-10 h-10 m-auto animate-spin pointer-events-none"
                    />
                }
                    .into_view(cx)
            }>{metadata}</Suspense>
            <div class="dark:bg-gray-800 w-full rounded-xl px-4 py-2 my-2">
                <div class="flex flex-row justify-between w-full">
                    <span class="text-gray-600 dark:text-gray-400 text-lg my-auto p-1">
                        "Chapters"
                    </span>
                    <Suspense fallback=|| ()>
                        <Show
                            when=move || {
                                if let Some(ValidationResult::Ok((apub_id, _))) = valid() {
                                    authors().into_iter().map(|(a, _)| a).collect::<Vec<_>>().contains(&apub_id)
                                } else {
                                    false
                                }
                            }
                            fallback=|_| ()
                        >
                            <button class="flex flex-row gap-1 p-1 rounded-md text-gray-500 dark:text-gray-300 hover:dark:bg-gray-900 focus:dark:bg-gray-900">
                                <Icon
                                    icon=CgIcon::CgMathPlus
                                    class="dark:stroke-white w-6 h-6 my-auto stroke-0"
                                />
                                <span class="my-auto pr-1">"New chapter"</span>
                            </button>
                        </Show>
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

#[server(GetNovel, "/server")]
pub async fn get_novel(
    cx: Scope,
    uuid: String,
) -> Result<Result<Box<Novel>, String>, ServerFnError> {
    use activitypub_federation::config::Data;
    use leptos_actix::extract;
    use wordforge_api::{
        api::novel::{self, GetNovelResult},
        DbHandle,
    };

    let pool = extract(cx, |pool: Data<DbHandle>| async move { pool }).await?;

    match novel::get_novel(uuid, &pool).await {
        GetNovelResult::Ok(v) => {
            let novel = Box::new(Novel {
                name: v.name,
                summary: v.summary,
                authors: v
                    .authors
                    .into_iter()
                    .map(|a| Author {
                        apub_id: a.apub_id,
                        role: a.role.to_string(),
                    })
                    .collect(),
                genre: v.genre.to_string(),
                tags: v.tags,
                language: Language::from_639_1(&v.language).ok_or_else(|| {
                    ServerFnError::ServerError("Language parse error".to_string())
                })?,
                sensitive: v.sensitive,
                published: v.published,
            });
            Ok(Ok(novel))
        }
        GetNovelResult::PermanentRedirect(loc) => {
            leptos_actix::redirect(cx, &loc);
            Ok(Err(String::new()))
        }
        GetNovelResult::InternalServerError(e) => Err(ServerFnError::ServerError(e)),
        _ => Ok(Err("NotFound".to_string())),
    }
}

#[server(GetUsername, "/server")]
pub async fn get_usernames(
    cx: Scope,
    authors: Vec<(String, String)>,
) -> Result<Vec<(String, String, Result<String, String>)>, ServerFnError> {
    use activitypub_federation::{config::Data, fetch::object_id::ObjectId};
    use futures::future;
    use itertools::Itertools;
    use leptos_actix::extract;
    use reqwest::Url;
    use wordforge_api::{objects::person::User, DbHandle};

    let pool = extract(cx, |pool: Data<DbHandle>| async move { pool }).await?;

    let mut urls = vec![];
    for (apub_id, role) in authors.into_iter() {
        urls.push((
            Url::parse(&apub_id)
                .map_err(|e| ServerFnError::ServerError(format!("{apub_id}: {e}")))?,
            role,
        ));
    }

    let authors = urls.into_iter().fold(vec![], |mut acc, (url, role)| {
        if acc.is_empty() {
            acc.push(vec![(url, role)])
        } else {
            let inserted = 'inserted: {
                for urls in acc.iter_mut() {
                    if urls[0].0.host_str() == url.host_str() {
                        urls.push((url.clone(), role.clone()));
                        break 'inserted true;
                    }
                }
                false
            };
            if !inserted {
                acc.push(vec![(url, role)])
            };
        };
        acc
    });

    let mut fetchers = vec![];
    for urls in authors.iter() {
        let fetch = |urls: Vec<(ObjectId<User>, String)>| async {
            let mut res = vec![];
            for (apub_id, role) in urls.into_iter() {
                let domain = format!(
                    "{}{}",
                    apub_id.inner().host_str().expect("apub_id hostname"),
                    apub_id
                        .inner()
                        .port()
                        .map(|p| format!(":{}", p))
                        .unwrap_or(String::new())
                );
                let (href, name) = match apub_id.dereference(&pool).await {
                    Ok(user) => (
                        format!(
                            "/users/{}{}",
                            user.preferred_username,
                            domain
                                .eq(pool.domain())
                                .then_some(String::new())
                                .unwrap_or(format!("@{domain}"))
                        ),
                        Ok(user.name),
                    ),
                    Err(e) => (
                        apub_id.inner().to_string(),
                        Err(format!(
                            "{}: {}",
                            apub_id.inner(),
                            e.chain().map(|e| e.to_string()).join("\n")
                        )),
                    ),
                };
                res.push((href, role, name));
            }
            res
        };
        fetchers.push(fetch(
            urls.iter()
                .map(|(u, r)| (ObjectId::<User>::parse(u.as_str()).unwrap(), r.clone()))
                .collect(),
        ));
    }

    let res = future::join_all(fetchers)
        .await
        .into_iter()
        .flatten()
        .collect();
    Ok(res)
}
