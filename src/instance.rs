use crate::objects::person::User;
use activitypub_federation::{
    config::{Data, FederationConfig, UrlVerifier},
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name},
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound},
    get, web, HttpResponse,
};
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use url::Url;

#[derive(Clone)]
struct VerifyUrl();

#[async_trait]
impl UrlVerifier for VerifyUrl {
    async fn verify(&self, url: &Url) -> Result<(), &'static str> {
        println!("{url}");
        Ok(())
    }
}

pub async fn new_database(
    host: String,
    url: String,
) -> Result<FederationConfig<PgPool>, std::io::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(6)
        .connect(url.as_str())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    FederationConfig::builder()
        .debug(cfg!(debug_assertions))
        .domain(host)
        .url_verifier(Box::new(VerifyUrl()))
        .app_data(pool)
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[derive(Deserialize)]
struct WebfingerQuery {
    resource: String,
}

#[get("/.well-known/webfinger")]
async fn webfinger(
    query: web::Query<WebfingerQuery>,
    data: Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let name =
        extract_webfinger_name(&query.resource, &data).map_err(|_| ErrorNotFound("Bad domain"))?;
    let user = User::read_from_username(name.as_str(), data.app_data())
        .await
        .map_err(ErrorBadRequest)?
        .ok_or_else(|| ErrorNotFound("Local user not found"))?;
    let res = build_webfinger_response(
        query.resource.clone(),
        user.apub_id.parse().map_err(ErrorInternalServerError)?,
    );
    Ok(HttpResponse::Ok().json(res))
}
