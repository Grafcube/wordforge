use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub mod components;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use components::{auth::*, novel::*};

    _ = ServerLogin::register();
    _ = ServerRegister::register();
    _ = UserValidate::register();
    _ = CreateNovel::register();
    _ = GetGenres::register();
    _ = GetRoles::register();
    _ = GetLangs::register();
}
