use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotFoundPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Page not found"/>
        <span class="mx-auto text-center dark:text-white">"404: Not found"</span>
    }
}

#[component]
pub fn InternalErrorPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Something went wrong"/>
        <span class="mx-auto text-center dark:text-white">"500: Internal Server Error"</span>
    }
}
