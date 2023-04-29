use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn FilterListbox(
    cx: Scope,
    option: RwSignal<String>,
    name: &'static str,
    label: &'static str,
    items: Vec<String>,
) -> impl IntoView {
    let (menu, toggle_menu) = create_signal(cx, false);
    let (filter, set_filter) = create_signal(cx, String::new());
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
            .iter()
            .map(|i| {
                let val = i.clone();
                let val1 = i.clone(); // wtf
                view! { cx,
                    <li
                        on:click=move |_| {
                            option.set(val.clone());
                            toggle_menu(false);
                        }
                        value=i
                        class=move || {
                            let class = "flex align-middle justify-start p-2 m-1 cursor-pointer rounded-md hover:dark:bg-gray-800"
                                .to_string();
                            if option.get() == val1 { format!("{class} dark:bg-gray-900") } else { class }
                        }
                    >
                        {i}
                    </li>
                }
            })
            .collect::<Vec<_>>()
    };

    view! { cx,
        <div>
            <div
                class="flex items-start dark:bg-gray-800 rounded-md w-full p-2 cursor-pointer"
                on:click=move |_| {
                    set_filter(String::new());
                    toggle_menu(!menu());
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
                        v if v.is_empty() => "Select a genre".to_string(),
                        v => v,
                    }}
                </span>
            </div>
            <Show when=menu fallback=|_| ()>
                <ul class="absolute z-10 text-left w-fit dark:bg-gray-700 rounded-xl overflow-y-auto max-h-80 text-lg">
                    <input
                        class="dark:bg-gray-600 m-2 p-2 rounded-xl text-sl w-fit"
                        type="search"
                        placeholder="Filter"
                        name="filter"
                        autocomplete="off"
                        prop:value=filter
                        on:change=move |ev| set_filter(event_target_value(&ev))
                    />
                    {build_filter.clone()}
                </ul>
            </Show>
        </div>
    }
}
