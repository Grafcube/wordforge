use crate::app::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotFoundPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Body class="main-screen"/>
        <Topbar/>
        <span class="text-center dark:text-white">"404: Not found"</span>
    }
}
