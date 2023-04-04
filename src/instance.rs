use crate::objects::{novel::DbNovel, person::User};
use activitypub_federation::{
    config::{Data, FederationConfig, UrlVerifier},
    fetch::webfinger::{build_webfinger_response_with_type, extract_webfinger_name},
};
use actix_web::{error::ErrorNotFound, get, web, HttpResponse};
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use url::Url;
use uuid::Uuid;

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
        .unwrap_or(None);
    let novel = match Uuid::try_parse(&name) {
        Ok(uuid) => DbNovel::read_from_uuid(uuid, &data).await.unwrap_or(None),
        Err(_) => None,
    };

    let urls: Vec<(Url, Option<&str>)> = vec![
        (
            user.map(|v| Url::parse(v.apub_id.as_str()).expect("user parse error")),
            Some("Person"),
        ),
        (
            novel.map(|v| Url::parse(v.apub_id.as_str()).expect("novel parse error")),
            Some("Group"),
        ),
    ]
    .iter()
    .filter(|v| v.0.is_some())
    .map(|v| (v.0.clone().unwrap(), v.1))
    .collect();

    if urls.is_empty() {
        Err(ErrorNotFound("Local actor not found"))
    } else {
        let res = build_webfinger_response_with_type(query.resource.clone(), urls);
        Ok(HttpResponse::Ok().json(res))
    }
}
