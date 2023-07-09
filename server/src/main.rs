use crate::instance::{new_database, webfinger};
use activitypub_federation::config::FederationMiddleware;
use actix_files::Files;
use actix_session::{
    config::{CookieContentSecurity, PersistentSession},
    storage::RedisActorSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key, SameSite},
    middleware::{self, Compress, NormalizePath},
    web, HttpServer,
};
use leptos::view;
use leptos_actix::{generate_route_list, handle_server_fns, LeptosRoutes};
use std::{env, io};
use wordforge_api::util::AppState;
use wordforge_ui::{app::*, register_server_functions};

mod api;
mod instance;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        },
    ));

    register_server_functions();

    let uiconf = leptos::get_configuration(None)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let addr = uiconf.leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> });
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT is required");
    let redis_url = format!("localhost:{redis_port}");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let state = AppState {
        scheme: env::var("SCHEME").expect("SCHEME is required").into(),
    };
    let config = new_database(addr.to_string(), db_url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let key = Key::from(include_bytes!("cookie.key")); // TODO: Better way to do this

    log::info!("Starting server");
    log::info!("Listening on {addr}");

    HttpServer::new(move || {
        let session =
            SessionMiddleware::builder(RedisActorSessionStore::new(redis_url.clone()), key.clone())
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(7)))
                .cookie_content_security(CookieContentSecurity::Private)
                .cookie_same_site(SameSite::Strict)
                .cookie_secure(cfg!(not(debug_assertions)))
                .build();

        let opts = &uiconf.leptos_options;
        let site_root = &opts.site_root;
        let routes = &routes;

        actix_web::App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .wrap(session)
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(FederationMiddleware::new(config.clone()))
            .route("/user/{name}", api::users())
            .route("/novel/{uuid}", api::novels())
            .service(api::novel::novel_inbox)
            .service(api::novel::novel_outbox)
            .service(api::user::user_outbox)
            .service(api::scope())
            .service(webfinger)
            .route("/server/{tail:.*}", handle_server_fns())
            .leptos_routes(
                opts.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", site_root))
    })
    .bind(&addr)?
    .run()
    .await
}
