use app::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub mod components;
pub mod fallback;
pub(crate) mod path;
pub mod routes;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        view! { cx, <App/> }
    });
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use components::chapter::CreateChapter;
    use routes::{auth::*, novel::*};

    _ = ServerLogin::register();
    _ = ServerRegister::register();
    _ = UserValidate::register();
    _ = Logout::register();
    _ = CreateNovel::register();
    _ = GetGenres::register();
    _ = GetRoles::register();
    _ = GetLangs::register();
    _ = GetNovel::register();
    _ = GetUsername::register();
    _ = CreateChapter::register();
}
