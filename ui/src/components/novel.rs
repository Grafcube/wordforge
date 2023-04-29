use crate::components::{basicinput::*, listbox::*};
use leptos::{ev::KeyboardEvent, html::Textarea, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn CreateBook(cx: Scope) -> impl IntoView {
    let create = create_server_action::<CreateNovel>(cx);
    let summary = create_node_ref::<Textarea>(cx);
    let title_area_handler = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
            summary()
                .expect("summary field ref")
                .focus()
                .expect("summary focus");
        }
    };
    let genres = create_resource(cx, || (), move |_| get_genres());
    let genre = create_rw_signal(cx, String::new());

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
                        on:keydown=title_area_handler
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
                <Suspense fallback=move || {
                    view! { cx, <span>"Loading..."</span> }
                }>
                    {move || match genres.read(cx) {
                        None => {
                            view! { cx, <span>"Loading..."</span> }
                                .into_view(cx)
                        }
                        Some(Ok(items)) => {
                            view! { cx, <FilterListbox option=genre name="genres" label="Genres" items=items/> }
                                .into_view(cx)
                        }
                        Some(Err(e)) => {
                            error!("{}", e.to_string());
                            view! { cx, <span>"Something went wrong"</span> }
                                .into_view(cx)
                        }
                    }}
                </Suspense>
                <button class="button-1" type="submit">
                    "Create"
                </button>
            </ActionForm>
        </div>
    }
}

#[server(CreateNovel, "/server")]
pub async fn create_novel(cx: Scope) -> Result<(), ServerFnError> {
    todo!()
}

#[server(GetGenres, "/server")]
pub async fn get_genres() -> Result<Vec<String>, ServerFnError> {
    use strum::IntoEnumIterator;
    use wordforge_api::enums::Genres;

    Ok(Genres::iter().map(|g| g.to_string()).collect())
}
