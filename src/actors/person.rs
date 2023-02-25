use crate::schema::users;
use activitypub_federation::core::signatures::Keypair;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Validate)]
#[diesel(table_name = users)]
pub struct User {
    pub id: u32,
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub followers: Vec<String>,
    pub following: Vec<String>,
    pub public_key: String,
    private_key: Option<String>,
    pub published: DateTime<Utc>,
    #[validate(email)]
    pub email: String,
    // password: String, // TODO: Use some library to make this more secure
}

impl User {
    pub fn new(
        id: u32,
        username: String,
        display_name: String,
        email: String,
        keypair: Keypair,
    ) -> User {
        Self {
            id,
            preferred_username: username,
            name: display_name,
            summary: String::new(),
            followers: Vec::new(),
            following: Vec::new(),
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key),
            published: Utc::now(),
            email,
            // password: (),
        }
    }

    pub fn ap_id(&self, host: String) -> String {
        format!("{}/actors/{}", host, &self.id)
    }

    pub fn inbox(&self, host: String) -> String {
        format!("{}/inbox", &self.ap_id(host))
    }
}
