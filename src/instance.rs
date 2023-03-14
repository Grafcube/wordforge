use activitypub_federation::config::{FederationConfig, UrlVerifier};
use async_trait::async_trait;
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

pub async fn new_database(host: String, url: String) -> anyhow::Result<FederationConfig<PgPool>> {
    let pool = PgPoolOptions::new()
        .max_connections(6)
        .connect(url.as_str())
        .await?;

    FederationConfig::builder()
        .debug(cfg!(debug_assertions))
        .domain(host)
        .url_verifier(Box::new(VerifyUrl()))
        .app_data(pool)
        .build()
        .map_err(anyhow::Error::new)
}
