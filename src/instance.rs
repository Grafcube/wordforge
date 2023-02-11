use std::sync::Arc;

use activitypub_federation::{
    InstanceSettings, InstanceSettingsBuilderError, LocalInstance, UrlVerifier,
};
use async_trait::async_trait;
use reqwest::Client;
use url::Url;

pub type InstanceHandle = Arc<Instance>;

#[derive(Clone)]
struct VerifyUrl();

#[async_trait]
impl UrlVerifier for VerifyUrl {
    async fn verify(&self, url: &Url) -> Result<(), &'static str> {
        println!("{url}");
        Ok(())
    }
}

pub struct Instance {
    local_instance: LocalInstance,
}

impl Instance {
    pub fn new(host: String) -> Result<InstanceHandle, InstanceSettingsBuilderError> {
        let settings = InstanceSettings::builder()
            .debug(cfg!(debug_assertions))
            .url_verifier(Box::new(VerifyUrl()))
            .build()?;

        let local_instance = LocalInstance::new(host.clone(), Client::default().into(), settings);
        Ok(Arc::new(Instance { local_instance }))
    }

    pub fn local_instance(&self) -> &LocalInstance {
        &self.local_instance
    }
}
