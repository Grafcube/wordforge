use leptos::{ev::KeyboardEvent, *};
use wasm_bindgen::JsCast;

#[component]
pub fn FilterListbox(
    cx: Scope,
    option: RwSignal<String>,
    name: &'static str,
    label: &'static str,
    initial: &'static str,
    items: Vec<String>,
) -> impl IntoView {
    let dropdown = create_node_ref::<html::Div>(cx);
    let listbox = create_node_ref::<html::Div>(cx);
    let (menu, show_menu) = create_signal(cx, false);
    let (filter, set_filter) = create_signal(cx, String::new());
    let filter_field = create_node_ref::<html::Input>(cx);
    let filter_items = move || {
        let term = filter().to_lowercase();
        let term = term.as_str();
        let mut res = items
            .iter()
            .filter_map(|i| {
                i.to_string()
                    .to_lowercase()
                    .contains(term)
                    .then(|| i.to_string())
            })
            .collect::<Vec<String>>();
        res.sort_by(|a, b| {
            a.to_string()
                .to_lowercase()
                .find(term)
                .partial_cmp(&b.to_string().to_lowercase().find(term))
                .unwrap()
        });
        res
    };
    let build_filter = move || {
        filter_items()
            .into_iter()
            .map(|i| {
                view! { cx,
                    <button
                        on:keydown=move |ev: KeyboardEvent| {
                            match ev.key().as_str() {
                                "Home" => {
                                    ev.prevent_default();
                                    if let Some(Ok(el))
                                        = dropdown()
                                            .unwrap()
                                            .first_element_child()
                                            .map(|sibling| sibling.dyn_into::<web_sys::HtmlElement>())
                                    {
                                        let _ = el.focus();
                                    }
                                }
                                "ArrowUp" => {
                                    ev.prevent_default();
                                    if let Some(Ok(sibling))
                                        = event_target::<web_sys::HtmlElement>(&ev)
                                            .previous_element_sibling()
                                            .map(|sibling| sibling.dyn_into::<web_sys::HtmlElement>())
                                    {
                                        let _ = sibling.focus();
                                    }
                                }
                                "ArrowDown" => {
                                    ev.prevent_default();
                                    if let Some(Ok(sibling))
                                        = event_target::<web_sys::HtmlElement>(&ev)
                                            .next_element_sibling()
                                            .map(|sibling| sibling.dyn_into::<web_sys::HtmlElement>())
                                    {
                                        let _ = sibling.focus();
                                    }
                                }
                                "End" => {
                                    ev.prevent_default();
                                    if let Some(Ok(el))
                                        = dropdown()
                                            .unwrap()
                                            .last_element_child()
                                            .map(|sibling| sibling.dyn_into::<web_sys::HtmlElement>())
                                    {
                                        let _ = el.focus();
                                    }
                                }
                                "/" => {
                                    ev.prevent_default();
                                    filter_field().unwrap().focus().unwrap();
                                }
                                _ => {}
                            }
                        }
                        on:click={
                            let val = i.clone();
                            move |ev| {
                                ev.prevent_default();
                                option.set(val.clone());
                                show_menu(false);
                            }
                        }
                        value=i.clone()
                        class={
                            let val = i.clone();
                            move || {
                                let class = "flex align-middle justify-start p-2 m-1 cursor-pointer rounded-md hover:dark:bg-gray-800 focus:dark:bg-gray-800"
                                    .to_string();
                                if option.get() == val { format!("{class} dark:bg-gray-900") } else { class }
                            }
                        }
                    >
                        {i}
                    </button>
                }
            })
            .collect::<Vec<_>>()
    };

    view! { cx,
        <div node_ref=listbox>
            <div
                class="flex items-start dark:bg-gray-800 rounded-md w-full p-2 cursor-pointer"
                on:click=move |_| {
                    set_filter(String::new());
                    show_menu(!menu());
                    if menu() {
                        filter_field().unwrap().focus().unwrap();
                    }
                }
            >
                <label
                    for=name
                    class="text-left pr-2 pointer-events-none text-gray-500 dark:text-gray-400"
                >
                    {label}
                </label>
                <span class="text-left pointer-events-none">
                    {move || match option.get() {
                        v if v.is_empty() => initial.to_string(),
                        v => v,
                    }}
                </span>
            </div>
            <Show when=menu fallback=|_| ()>
                <div
                    class="absolute flex flex-col overflow-y-auto z-50 text-left justify-items-stretch w-fit dark:bg-gray-700 rounded-xl max-h-80 text-lg"
                    on:keydown=move |ev: KeyboardEvent| {
                        if ev.key().as_str() == "Escape" {
                            ev.prevent_default();
                            show_menu(false);
                        }
                    }
                    on:focusout=move |ev| {
                        let target = listbox().unwrap();
                        let receiver = ev.related_target().map(|r| r.unchecked_into::<web_sys::Node>());
                        if !target.contains(receiver.as_ref()) {
                            show_menu(false);
                        }
                    }
                >
                    <input
                        class="dark:bg-gray-600 m-2 p-2 rounded-xl text-sl w-fit"
                        type="search"
                        placeholder="Filter"
                        name="filter"
                        autocomplete="off"
                        prop:value=filter
                        node_ref=filter_field
                        on:input=move |ev| set_filter(event_target_value(&ev))
                        on:keydown=move |ev: KeyboardEvent| {
                            if ev.key().as_str() == "ArrowDown" {
                                ev.prevent_default();
                                if let Some(Ok(el))
                                    = dropdown()
                                        .unwrap()
                                        .first_element_child()
                                        .map(|sibling| sibling.dyn_into::<web_sys::HtmlElement>())
                                {
                                    let _ = el.focus();
                                }
                            }
                        }
                    />
                    <div tabindex=-1 node_ref=dropdown class="flex flex-col overflow-y-auto">
                        {build_filter.clone()}
                    </div>
                </div>
            </Show>
        </div>
    }
}
