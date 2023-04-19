use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[component]
pub(crate) fn Auth(cx: Scope) -> impl IntoView {
    let (errormsg, set_errormsg) = create_signal(cx, String::new());

    view! { cx,
        <Body class="main-screen"/>
        <Login set_errormsg=set_errormsg/>
    }
}

#[component]
pub(crate) fn Login(cx: Scope, set_errormsg: WriteSignal<String>) -> impl IntoView {
    let login = create_server_action::<ServerLogin>(cx);
    view! { cx,
        <ActionForm action=login>
            <input type="email" class="basic-input" placeholder="Email" name="email" required/>
            <input
                type="password"
                class="basic-input"
                placeholder="Password (minimum 8 characters)"
                name="password"
                required
            />
            <input type="submit" class="button-1" value="Sign in"/>
        </ActionForm>
    }
}

#[component]
fn ErrorView(cx: Scope, message: String) -> impl IntoView {
    view! { cx, <p class="text-red-800">{message}</p> }
}

#[server(ServerLogin, "/auth/login")]
pub async fn login(cx: Scope, email: String, password: String) -> Result<(), ServerFnError> {
    #[derive(Serialize, Deserialize, Validate)]
    struct LoginData {
        #[validate(email)]
        email: String,
        #[validate(length(min = 8))]
        password: String,
        client_app: String,
        #[validate(url)]
        client_website: Option<String>,
    }
    todo!()
}
