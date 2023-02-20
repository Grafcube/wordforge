use activitypub_federation::core::signatures::Keypair;
use chrono::prelude::*;
use url::Url;

pub struct User {
    pub id: u32,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub inbox: Url,
    pub followers: Vec<Url>,
    pub following: Vec<Url>,
    pub public_key: String,
    private_key: Option<String>,
    pub published: DateTime<Utc>,
}

impl User {
    pub fn new(id: u32, username: String, keypair: Keypair) -> User {
        unimplemented!()
    }
}
