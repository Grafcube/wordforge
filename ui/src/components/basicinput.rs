use leptos::*;

#[component]
pub fn FloatingLabel(cx: Scope, target: &'static str, children: Children) -> impl IntoView {
    view! { cx,
        <label
            for=target
            class="absolute text-xl text-gray-500 dark:text-gray-400 duration-100 transform -translate-y-4 scale-75 top-3 left-1 z-10 origin-[0] bg-transparent px-2 pb-1 peer-focus:px-2 peer-placeholder-shown:scale-100 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:top-6 peer-focus:top-3 peer-focus:scale-75 peer-focus:-translate-y-4"
        >
            {children(cx)}
        </label>
    }
}
