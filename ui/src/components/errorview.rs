use leptos::*;

#[component]
pub fn ErrorView(cx: Scope, message: ReadSignal<String>) -> impl IntoView {
    view! { cx, <p class="text-red-800">{message}</p> }
}
