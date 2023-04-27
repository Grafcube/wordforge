use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn FilterListbox(
    cx: Scope,
    name: &'static str,
    label: &'static str,
    items: Vec<String>,
) -> impl IntoView {
    let (filter, set_filter) = create_signal(cx, String::new());
    let filter_items = move || {
        let term = filter().to_lowercase();
        let term = term.as_str();
        let mut res = items
            .iter()
            .filter(|i| i.to_lowercase().contains(term))
            .map(|i| i.to_string())
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

    view! { cx,
        <label for=name class="block">
            {label}
        </label>
        <select id=name name=name class="dark:bg-gray-800">
            {filter_items()
                .iter()
                .map(|i| {
                    view! { cx, <option value=i>{i}</option> }
                })
                .collect::<Vec<_>>()}
        </select>
    }
}
