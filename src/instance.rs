use activitypub_federation::{
    request_data::ApubContext, FederationSettings, InstanceConfig, UrlVerifier,
};
use async_trait::async_trait;
use reqwest::Client;
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

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(host: String, url: String) -> anyhow::Result<ApubContext<Database>> {
        let settings = FederationSettings::builder()
            .debug(cfg!(debug_assertions))
            .url_verifier(Box::new(VerifyUrl()))
            .build()?;

        let pool = PgPoolOptions::new()
            .max_connections(6)
            .connect(url.as_str())
            .await?;

        let local_instance = InstanceConfig::new(host, Client::default().into(), settings);
        Ok(ApubContext::new(Self { pool }, local_instance))
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
