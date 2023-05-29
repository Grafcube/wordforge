use crate::components::{basicinput::FloatingLabel, toggle::Toggle};
use leptos::{
    ev::{Event, KeyboardEvent},
    html::*,
    *,
};
use leptos_router::*;
use wasm_bindgen::UnwrapThrowExt;

#[component]
pub fn NewChapter(
    cx: Scope,
    #[allow(unused_variables)] node_ref: NodeRef<Dialog>,
) -> impl IntoView {
    let create = create_server_action::<CreateChapter>(cx);
    let response = create.value();
    let (errormsg, set_errormsg) = create_signal::<Option<String>>(cx, None);
    let err = move || {
        response().map(|v| match v {
            Ok(Ok(_)) => {
                if let Some(v) = node_ref() {
                    v.close();
                }
                set_errormsg(None);
            }
            Ok(Err(v)) => set_errormsg(Some(v)),
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

    view! { cx,
        <dialog
            class="rounded-xl w-md max-w-md dark:bg-gray-900 dark:text-white"
            node_ref=node_ref
            on:cancel=move |_: Event| {
                form().unwrap_throw().reset();
                set_errormsg(None)
            }
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

#[server(CreateChapter, "/server")]
pub async fn create(
    cx: Scope,
    title: String,
    summary: String,
    sensitive: bool,
) -> Result<Result<String, String>, ServerFnError> {
    Err(ServerFnError::ServerError("To be implemented".to_string()))
}
