use crate::util::USERNAME_RE;
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::PersonType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use async_trait::async_trait;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use url::Url;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct User {
    pub apub_id: String,
    #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub public_key: String,
    #[serde(skip_serializing)]
    private_key: Option<String>,
    pub published: DateTime<Utc>,
    pub last_refresh: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    id: ObjectId<User>,
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    name: String,
    summary: String,
    inbox: Url,
    outbox: Url,
    public_key: PublicKey,
    published: String,
}

impl User {
    pub async fn read_from_username(
        username: &str,
        data: &PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        query_as!(
            Self,
            r#"SELECT apub_id, preferred_username, name, summary, inbox, outbox,
            public_key, null as private_key, published, last_refresh
            FROM users WHERE preferred_username=$1"#,
            username.to_lowercase()
        )
        .fetch_optional(data)
        .await
    }
}

#[async_trait]
impl Object for User {
    type DataType = PgPool;
    type Kind = Person;
    type Error = anyhow::Error;

    fn last_refreshed_at(&self) -> Option<NaiveDateTime> {
        Some(self.last_refresh)
    }

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        query_as!(
            Self,
            r#"SELECT apub_id, preferred_username, name, summary, inbox, outbox,
            public_key, private_key, published, last_refresh
            FROM users WHERE apub_id=$1"#,
            object_id.to_string().to_lowercase()
        )
        .fetch_optional(data.app_data())
        .await
        .map_err(Self::Error::new)
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind {
            id: self.apub_id.parse()?,
            kind: Default::default(),
            preferred_username: self.preferred_username.clone(),
            name: self.name.clone(),
            summary: self.summary.clone(),
            inbox: self.inbox.parse()?,
            outbox: self.outbox.parse()?,
            public_key: self.public_key(),
            published: self.published.to_rfc3339_opts(SecondsFormat::Millis, true),
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(
        json: Self::Kind,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            apub_id: json.id.into_inner().into(),
            preferred_username: json.preferred_username,
            name: json.name,
            summary: json.summary,
            inbox: json.inbox.into(),
            outbox: json.outbox.into(),
            public_key: json.public_key.public_key_pem,
            private_key: None,
            published: json.published.parse()?,
            last_refresh: Local::now().naive_local(),
        })
    }
}

impl Actor for User {
    fn id(&self) -> Url {
        self.apub_id.parse().unwrap()
    }

    fn inbox(&self) -> Url {
        self.inbox.parse().unwrap()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }
}
