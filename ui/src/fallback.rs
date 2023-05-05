use crate::app::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotFoundPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Page not found"/>
        <Overlay class="text-center dark:text-white">"404: Not found"</Overlay>
    }
}

#[component]
pub fn InternalErrorPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Something went wrong"/>
        <Overlay class="text-center dark:text-white">"500: Internal Server Error"</Overlay>
    }
}
