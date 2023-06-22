use leptos::*;

#[component]
pub fn Tooltip<F, IV>(cx: Scope, view: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { cx,
        <div class="group w-max">
            {children(cx)}
            <div class="absolute z-50 w-max -translate-y-16 pointer-events-none p-2 rounded-md opacity-0 transition-opacity group-hover:opacity-100 duration-100 dark:bg-gray-700 dark:text-gray-400">
                {view()}
            </div>
        </div>
    }
}
