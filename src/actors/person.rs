use crate::util::USERNAME_RE;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: i32,
    #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub followers: Vec<String>,
    pub following: Vec<String>,
    pub public_key: String,
    #[serde(skip_serializing)]
    pub private_key: Option<String>,
    pub published: DateTime<Utc>,
    #[validate(email)]
    pub email: String,
    // password: String, // TODO: Use some library to make this more secure
}

impl User {
    pub fn ap_id(&self, host: String) -> String {
        format!("{}/actors/{}", host, &self.id)
    }

    pub fn inbox(&self, host: String) -> String {
        format!("{}/inbox", &self.ap_id(host))
    }
}
