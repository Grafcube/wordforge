use crate::util::USERNAME_RE;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Content {
    pub id: Uuid,
    #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub followers: Vec<String>,
    pub following: Vec<String>,
    pub published: DateTime<Utc>,
}
