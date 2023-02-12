use activitypub_federation::{
    request_data::ApubContext, FederationSettings, FederationSettingsBuilderError, InstanceConfig,
    UrlVerifier,
};
use async_trait::async_trait;
use reqwest::Client;
use std::sync::Arc;
use url::Url;

pub type DatabaseHandle = Arc<Database>;

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
pub struct Database();

impl Database {
    pub fn new(
        host: String,
    ) -> Result<ApubContext<DatabaseHandle>, FederationSettingsBuilderError> {
        let settings = FederationSettings::builder()
            .debug(cfg!(debug_assertions))
            .url_verifier(Box::new(VerifyUrl()))
            .build()?;

        let local_instance = InstanceConfig::new(host, Client::default().into(), settings);
        let instance = Arc::new(Database());
        Ok(ApubContext::new(instance, local_instance))
    }
}
