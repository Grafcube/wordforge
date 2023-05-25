use leptos::{ev::*, html::*, *};
use wasm_bindgen::JsCast;

#[component]
pub fn Panel(
    cx: Scope,
    when: RwSignal<bool>,
    class: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    let panel = create_node_ref::<Div>(cx); // Neither focusout nor blur are working

    view! { cx,
        <Show when=when fallback=|_| ()>
            <div
                node_ref=panel
                class=class
                on:keydown=move |ev: KeyboardEvent| {
                    if ev.key().as_str() == "Escape" {
                        ev.prevent_default();
                        when.set(false);
                    }
                }
                on:focusout=move |ev| {
                    let target = panel().unwrap();
                    let receiver = ev.related_target().map(|r| r.unchecked_into::<web_sys::Node>());
                    log!("panel -> {:?}", receiver);
                    if !target.contains(receiver.as_ref()) {
                        when.set(false);
                    }
                }
            >
                {children(cx)}
            </div>
        </Show>
    }
}
