use activitypub_federation::{
    request_data::ApubContext, FederationSettings, InstanceConfig, UrlVerifier,
};
use async_trait::async_trait;
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use reqwest::Client;
use std::sync::Arc;
use url::Url;

pub type DatabaseHandle = Arc<Database>;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
    pool: Pool,
}

impl Database {
    pub fn new(host: String, url: String) -> Result<ApubContext<DatabaseHandle>, anyhow::Error> {
        let settings = FederationSettings::builder()
            .debug(cfg!(debug_assertions))
            .url_verifier(Box::new(VerifyUrl()))
            .build()?;

        let manager = ConnectionManager::<PgConnection>::new(url);

        let local_instance = InstanceConfig::new(host, Client::default().into(), settings);
        let instance = Arc::new(Self {
            pool: r2d2::Pool::builder().build(manager)?,
        });
        Ok(ApubContext::new(instance, local_instance))
    }
}
