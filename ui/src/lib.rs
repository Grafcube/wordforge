use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub(crate) mod components;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}
