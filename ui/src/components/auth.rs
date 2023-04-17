use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub(crate) fn Auth(cx: Scope) -> impl IntoView {
    let (errormsg, set_errormsg) = create_signal(cx, String::new());

    view! { cx,
        <Body class="main-screen"/>
        <Login/>
    }
}

#[component]
pub(crate) fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <Form method="post" action="/api/v1/login">
            <input type="email" class="basic-input" placeholder="Email" name="email" required/>
            <input
                type="password"
                class="basic-input"
                placeholder="Password (minimum 8 characters)"
                name="password"
                required
            />
            <input type="hidden" name="client_app" value="Web"/>
            <input type="submit" class="button-1" value="Sign in"/>
        </Form>
    }
}

#[component]
fn ErrorView(cx: Scope, message: String) -> impl IntoView {
    view! { cx, <p class="text-red-800">{message}</p> }
}
