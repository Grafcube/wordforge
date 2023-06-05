use crate::components::{basicinput::FloatingLabel, toggle::Toggle};
use leptos::{
    ev::{Event, KeyboardEvent},
    html::*,
    *,
};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;

#[component]
pub fn NewChapter(
    cx: Scope,
    novel: String,
    trigger: WriteSignal<()>,
    #[allow(unused_variables)] node_ref: NodeRef<Dialog>,
) -> impl IntoView {
    let create = create_server_action::<CreateChapter>(cx);
    let response = create.value();
    let (errormsg, set_errormsg) = create_signal::<Option<String>>(cx, None);
    let err = move || {
        response().map(|v| match v {
            Ok(_) => {
                if let Some(v) = node_ref() {
                    v.close();
                }
                set_errormsg(None);
                trigger(());
            }
            Err(e) => set_errormsg(Some(e.to_string())),
        })
    };

    let form = create_node_ref::<Form>(cx);
    let summary = create_node_ref::<Textarea>(cx);
    let cw = create_node_ref::<Input>(cx);
    let (title, set_title) = create_signal(cx, String::new());
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

    let reset_form = move |_: Event| {
        form().unwrap_throw().reset();
        set_errormsg(None)
    };

    view! { cx,
        <dialog
            class="rounded-xl w-md max-w-md backdrop:bg-gray-950/60 dark:bg-gray-900 dark:text-white"
            node_ref=node_ref
            on:close=reset_form
            on:cancel=reset_form
        >
            <ActionForm
                class="flex flex-col justify-center text-center place-content-center items-center space-y-4 p-4 w-full"
                node_ref=form
                action=create
            >
                <div class="relative w-full">
                    <textarea
                        class="basic-input max-h-20 overflow-y-auto resize-none w-full peer"
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
                <div class="relative w-full">
                    <textarea
                        class="basic-input w-full peer"
                        placeholder=" "
                        node_ref=summary
                        name="summary"
                    ></textarea>
                    <FloatingLabel target="summary">"Summary"</FloatingLabel>
                </div>
                <input type="hidden" name="sensitive" value=move || sensitive().to_string()/>
                <div class="flex justify-start mr-auto">
                    <Toggle value=sensitive node_ref=cw>
                        "Content warning"
                    </Toggle>
                </div>
                <input type="hidden" name="novel" value=novel/>
                <div class="relative">
                    <input type="submit" class="button-1" value="Create"/>
                </div>
            </ActionForm>
            <div class="flex mx-auto text-2xl m-4 justify-center text-center">
                <p class="text-red-800 break-words">
                    {move || {
                        err();
                        errormsg
                    }}
                </p>
            </div>
        </dialog>
    }
}

#[component]
pub fn ChapterEntry(cx: Scope, chapter: Result<ChapterItem, ServerFnError>) -> impl IntoView {
    view! { cx,
        <ErrorBoundary fallback=move |cx, e| {
            view! { cx,
                <p class="text-red-600">
                    <ul>
                        {move || {
                            e.get()
                                .into_iter()
                                .map(|(_, e)| {
                                    view! { cx, <li>{e.to_string()}</li> }
                                })
                                .collect_view(cx)
                        }}
                    </ul>
                </p>
            }
        }>{chapter.map(|c| c.href)}</ErrorBoundary>
    }
}

#[server(CreateChapter, "/server")]
pub async fn create(
    cx: Scope,
    novel: String,
    title: String,
    summary: String,
    sensitive: bool,
) -> Result<(), ServerFnError> {
    use activitypub_federation::config::Data;
    use actix_session::Session;
    use actix_web::web;
    use leptos_actix::extract;
    use wordforge_api::{
        activities::add::NewChapter,
        api::chapter::{new_chapter, ChapterCreationError},
        util::AppState,
        DbHandle,
    };

    let (session, scheme, data) = extract(
        cx,
        |session: Session, state: web::Data<AppState>, data: Data<DbHandle>| async move {
            (session, state.scheme.clone(), data)
        },
    )
    .await?;

    let chapter = NewChapter {
        title,
        summary,
        sensitive,
    };

    match new_chapter(novel, chapter, session, &data, scheme).await {
        Ok(_) => Ok(()),
        Err(ChapterCreationError::InternalError(e)) => Err(ServerFnError::ServerError(e)),
        Err(ChapterCreationError::Unauthorized) => {
            Err(ServerFnError::ServerError("Not signed in".to_string()))
        }
        Err(ChapterCreationError::NotFound) => {
            Err(ServerFnError::ServerError("Novel not found".to_string()))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterItem {
    pub href: String,
    pub title: String,
    pub summary: String,
    pub sensitive: bool,
    pub published: String,
    pub updated: Option<String>,
}

#[server(GetChapters, "/server")]
pub async fn get_chapter_list(
    cx: Scope,
    novel: String,
) -> Result<Vec<Result<ChapterItem, ServerFnError>>, ServerFnError> {
    use activitypub_federation::config::Data;
    use chrono_humanize::HumanTime;
    use leptos_actix::extract;
    use wordforge_api::{
        api::chapter::{get_chapters, ChapterError},
        DbHandle,
    };

    let data = extract(cx, |data: Data<DbHandle>| async move { data }).await?;

    match get_chapters(novel, &data).await {
        Ok(c) => {
            let c = c
                .into_iter()
                .map(|c| match c {
                    Ok(c) => Ok(ChapterItem {
                        href: c.apub_id,
                        title: c.title,
                        summary: c.summary,
                        sensitive: c.sensitive,
                        published: HumanTime::from(c.published).to_string(),
                        updated: c.updated.map(|c| HumanTime::from(c).to_string()),
                    }),
                    Err(ChapterError::NotFound) => {
                        Err(ServerFnError::ServerError("Chapter not found".to_string()))
                    }
                    Err(ChapterError::InternalError(e)) => Err(ServerFnError::ServerError(e)),
                })
                .collect();
            Ok(c)
        }
        Err(ChapterError::NotFound) => {
            Err(ServerFnError::ServerError("Novel not found".to_string()))
        }
        Err(ChapterError::InternalError(e)) => Err(ServerFnError::ServerError(e)),
    }
}
