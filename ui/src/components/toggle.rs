use leptos::{ev::*, html::*, *};

#[component]
pub fn Toggle(
    cx: Scope,
    value: RwSignal<bool>,
    node_ref: NodeRef<Input>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <label class="relative inline-flex place-items-start items-start text-left text-xl align-middle space-x-2 rounded-full cursor-pointer">
            <input
                node_ref=node_ref
                type="checkbox"
                autocomplete="off"
                class="sr-only peer"
                on:change=move |ev| {
                    let state = event_target::<web_sys::HtmlInputElement>(&ev).checked();
                    value.set(state);
                }
                on:keydown=move |ev: KeyboardEvent| {
                    if ev.key() == "Enter" {
                        ev.prevent_default();
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        target.set_checked(!target.checked());
                        value.set(target.checked());
                    }
                }
            />
            <span class="h-8 w-14 bg-gray-200 peer-focus:outline-none peer-focus:ring-1 rounded-full peer-focus:ring-purple-300 dark:peer-focus:ring-purple-800 peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:bg-white after:border-gray-300 after:border after:top-1 after:left-3 after:rounded-full after:h-6 after:w-6 after:transition-all after:duration-100 dark:border-gray-600 peer-checked:bg-purple-600"></span>
            <div class="my-auto">{children(cx)}</div>
        </label>
    }
}
