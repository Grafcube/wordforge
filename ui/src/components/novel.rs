use crate::components::{basicinput::*, errorview::*, listbox::*, toggle::*};
use leptos::{ev::KeyboardEvent, html::*, *};
use leptos_meta::*;
use leptos_router::*;

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
        <Body class="main-screen p-2"/>
        <Title text="Create a new book"/>
        <h1 class="mx-auto text-3xl text-center">"Create a new book"</h1>
        <div class="flex justify-center text-center place-content-center items-center">
            <ActionForm action=create class="space-y-4 p-4 max-w-xl w-[36rem]">
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
    use actix_web::http::StatusCode;
    use leptos_actix::ResponseOptions;
    use std::str::FromStr;
    use wordforge_api::{
        api::novel::{self, CreateNovelResult, NewNovel},
        enums::*,
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

    let info = NewNovel {
        title,
        summary,
        genre: Genres::from_str(&genre).unwrap(),
        role: Roles::from_str(&role).unwrap(),
        lang,
        sensitive: cw,
        tags,
    };

    match novel::create_novel(pool, session, info).await {
        CreateNovelResult::Ok(id) => Ok(Ok(leptos_actix::redirect(cx, &format!("/novels/{}", id)))),
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
