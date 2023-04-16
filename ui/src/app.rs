use leptos::{ev::*, *};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/wordforge.css"/>
        <Link rel="icon" href="/favicon.svg"/>
        <Title text="Wordforge: Federated creative writing"/>
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=|cx| {
                        view! { cx, <Home/> }
                    }
                />
                <Route
                    path="/auth"
                    view=|cx| {
                        view! { cx, <Auth/> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <body class="main-screen">
            <Topbar/>
            <div class="fixed flex flex-row">
                <Sidebar/>
                <div class="items-center text-left overflow-auto">
                    <p class="mx-auto text-6xl text-center">"EVENTS"</p>
                    <p class="mx-auto text-6xl text-center">"RECOMMENDATIONS"</p>
                </div>
            </div>
        </body>
    }
}

#[component]
fn Topbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="sticky top-0 w-screen dark:bg-gray-950 m-0 p-1">
            <A href="/" class="m-2 px-2 w-fit flex items-start align-middle">
                <img
                    src="/favicon.svg"
                    alt="Home"
                    width="40"
                    height="40"
                    class="mx-1 my-auto invert dark:invert-0"
                />
                <h1 class="mx-1 my-auto text-3xl text-left">"Wordforge"</h1>
            </A>
        </div>
    }
}

#[component]
fn Sidebar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="sticky flex flex-col items-start p-1 text-xl align-top h-screen left-0 top-0 w-60 dark:bg-gray-700">
            <A
                href="/auth"
                class="m-1 w-[95%] p-2 rounded-md text-center dark:bg-purple-600 hover:dark:bg-purple-700"
            >
                "Sign in / Sign up"
            </A>
            <A href="/" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Home"
            </A>
            <A href="/local" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Local"
            </A>
            <A href="/public" class="m-1 w-[95%] p-2 rounded-md hover:dark:bg-gray-800">
                "Public"
            </A>
        </div>
    }
}

#[component]
fn Auth(cx: Scope) -> impl IntoView {
    view! { cx,
        <body class="main-screen">
            <Login/>
        </body>
    }
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    let (_email, set_email) = create_signal(cx, String::new());
    let (_pass, set_pass) = create_signal(cx, String::new());

    view! { cx,
        <Form method="post" action="/api/v1/login">
            <input
                type="email"
                class="basic-input"
                placeholder="Email"
                name="email"
                required
                on:keyup=move |ev: KeyboardEvent| set_email(event_target_value(&ev))
                on:change=move |ev| set_email(event_target_value(&ev))
            />
            <input
                type="password"
                class="basic-input"
                placeholder="Password (minimum 8 characters)"
                name="password"
                required
                on:keyup=move |ev: KeyboardEvent| set_pass(event_target_value(&ev))
                on:change=move |ev| set_pass(event_target_value(&ev))
            />
            <input type="hidden" name="client_app" value="Web"/>
            <input type="submit" class="button-1" value="Sign in"/>
        </Form>
    }
}
