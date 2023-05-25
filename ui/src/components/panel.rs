use leptos::*;

#[component]
pub fn Panel(
    cx: Scope,
    when: RwSignal<bool>,
    class: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    view! { cx,
        <Show when=when fallback=|_| ()>
            <div class=class>{children(cx)}</div>
        </Show>
    }
}
