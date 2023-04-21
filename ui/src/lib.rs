use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub mod components;
pub mod util;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use components::auth::*;

    _ = ServerLogin::register();
}
