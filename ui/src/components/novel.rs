use crate::{
    components::{basicinput::*, errorview::*, listbox::*, toggle::*},
    fallback::*,
    path::NovelViewParams,
};
use isolang::Language;
use leptos::{ev::KeyboardEvent, html::*, *};
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
                            placeholder=""
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
                            placeholder=""
                            node_ref=summary
                            name="summary"
                        ></textarea>
                        <FloatingLabel target="summary">"Summary"</FloatingLabel>
                    </div>
                    <input type="hidden" name="genre" value=move || genre.get()/>
                    <Transition fallback=move || {
                        view! { cx, <span>"Loading..."</span> }
                    }>
                        {move || match genres.read(cx) {
                            None => {
                                view! { cx, <span>"Loading..."</span> }
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
                    <Transition fallback=move || {
                        view! { cx, <span>"Loading..."</span> }
                    }>
                        {move || match roles.read(cx) {
                            None => {
                                view! { cx, <span>"Loading..."</span> }
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
                    <Transition fallback=move || {
                        view! { cx, <span>"Loading..."</span> }
                    }>
                        {move || match langs.read(cx) {
                            None => {
                                view! { cx, <span>"Loading..."</span> }
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
                            placeholder=""
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
                        <FloatingLabel target="tags">"Tags (Comma separated)"</FloatingLabel>
                    </div>
                    <input type="hidden" name="cw" value=move || sensitive.get().to_string()/>
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
    use actix_web::{http::StatusCode, web};
    use leptos_actix::ResponseOptions;
    use std::str::FromStr;
    use wordforge_api::{
        api::novel::{self, CreateNovelResult, NewNovel},
        enums::*,
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
    let session = <actix_session::Session as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

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
    let (authors, set_authors) = create_signal::<Vec<String>>(cx, Vec::new());
    let usernames = create_resource(cx, authors, move |authors| get_usernames(cx, authors));

    let author_view = move || {
        usernames.read(cx).map(|v| match v {
            Err(e) => {
                error!("usernames: {}", e.to_string());
                view! { cx, <span>"Something went wrong"</span> }.into_view(cx)
            }
            Ok(users) => {
                view! { cx,
                    <ul>
                        {users
                            .into_iter()
                            .map(|res| match res {
                                (apub_id, Err(e)) => {
                                    error!("{e}");
                                    view! { cx,
                                        <li>
                                            <a href=apub_id>"<UNKNOWN>"</a>
                                        </li>
                                    }
                                }
                                (apub_id, Ok(v)) => {
                                    view! { cx,
                                        <li>
                                            <a href=apub_id>{v}</a>
                                        </li>
                                    }
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                }
            }
            .into_view(cx),
        })
    };

    let view = move || {
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
                                 <div class="dark:bg-gray-800 rounded-xl px-4 py-2">
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
                                             .iter()
                                             .map(|a| a.apub_id.clone())
                                             .collect::<Vec<String>>();
                                         set_authors(author_list);
                                     } <Suspense fallback=move || {
                                         view! { cx, "Loading..." }
                                             .into_view(cx)
                                     }>{author_view}</Suspense>
                                 </div>
                                 <a href="/todo">{&novel.genre}</a>
                                 <div>
                                     {novel
                                         .tags
                                         .iter()
                                         .map(|tag| {
                                             view! { cx, <span class="p-1">{tag}</span> }
                                                 .into_view(cx)
                                         })
                                         .collect::<Vec<_>>()}
                                 </div>
                                 <span>{novel.language.to_name()}</span>
                             }
            .into_view(cx),
        })
    };

    view! { cx,
        <Title text="Novel"/>
        <div class="mx-auto max-w-2xl px-4">
            <Suspense fallback=move || {
                view! { cx, "Loading..." }
                    .into_view(cx)
            }>{view}</Suspense>
        </div>
    }
}

#[server(GetNovel, "/server")]
pub async fn get_novel(
    cx: Scope,
    uuid: String,
) -> Result<Result<Box<Novel>, String>, ServerFnError> {
    use activitypub_federation::config::Data;
    use wordforge_api::{
        api::novel::{self, GetNovelResult},
        DbHandle,
    };

    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let pool = <Data<DbHandle> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

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
    authors: Vec<String>,
) -> Result<Vec<(String, Result<String, String>)>, ServerFnError> {
    use actix_web::web;
    use futures::future;
    use leptos_actix::ResponseOptions;
    use reqwest::{Method, Url};
    use serde::Deserialize;
    use wordforge_api::util::AppState;

    let resp = use_context::<ResponseOptions>(cx).unwrap();
    let req = use_context::<actix_web::HttpRequest>(cx).unwrap();
    let state = <web::Data<AppState> as actix_web::FromRequest>::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let client = &state.client;

    #[derive(Deserialize)]
    struct Res {
        name: String,
    }

    let mut urls = vec![];
    for apub_id in authors.iter() {
        urls.push(
            Url::parse(apub_id)
                .map_err(|e| ServerFnError::ServerError(format!("{apub_id}: {e}")))?,
        );
    }

    let authors = urls.into_iter().fold(vec![], |mut acc, url| {
        if acc.is_empty() {
            acc.push(vec![url])
        } else {
            let inserted = 'inserted: {
                for urls in acc.iter_mut() {
                    if urls[0].host_str() == url.host_str() {
                        urls.push(url.clone());
                        break 'inserted true;
                    }
                }
                false
            };
            if !inserted {
                acc.push(vec![url])
            };
        };
        acc
    });

    let mut fetchers = vec![];
    for urls in authors.iter() {
        let fetch = |urls: Vec<Url>| async {
            let mut res = vec![];
            for apub_id in urls.into_iter() {
                let name = match client
                    .request(Method::GET, apub_id.to_string())
                    .header("accept", "application/activity+json")
                    .send()
                    .await
                {
                    Ok(res) => {
                        if res.status().is_success() {
                            match res.json::<Res>().await {
                                Ok(v) => Ok((apub_id.to_string(), Ok(v.name))),
                                Err(e) => {
                                    Err(ServerFnError::ServerError(format!("{apub_id}: {e}")))
                                }
                            }
                        } else {
                            resp.set_status(res.status());
                            Ok((
                                apub_id.to_string(),
                                res.text().await.map_err(|e| format!("{apub_id}: {e}")),
                            ))
                        }
                    }
                    Err(e) => Err(ServerFnError::ServerError(format!("{apub_id}: {e}"))),
                };
                res.push(name);
            }
            res
        };
        fetchers.push(fetch(urls.clone()));
    }

    let res = future::join_all(fetchers).await;
    let mut result = vec![];
    for res in res.into_iter().flatten() {
        result.push(res?);
    }

    Ok(result)
}
