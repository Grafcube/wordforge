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
use sqlx::{query, PgPool};
use url::Url;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct User {
    pub apub_id: ObjectId<User>,
    #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub inbox: Url,
    pub outbox: Url,
    pub public_key: String,
    #[serde(skip_serializing)]
    private_key: Option<String>,
    pub published: DateTime<Utc>,
    last_refresh: NaiveDateTime,
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

#[async_trait]
impl Object for User {
    type DataType = PgPool;
    type Kind = Person;
    type Error = std::io::Error;

    fn last_refreshed_at(&self) -> Option<NaiveDateTime> {
        Some(self.last_refresh)
    }

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let user = query!(
            "SELECT * FROM users WHERE apub_id=$1",
            object_id.to_string().to_lowercase()
        )
        .fetch_optional(data.app_data())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        match user {
            None => Ok(None),
            Some(user) => Ok(Some(Self {
                apub_id: Url::parse(user.apub_id.as_str()).unwrap().into(),
                preferred_username: user.preferred_username,
                name: user.name,
                summary: user.summary,
                inbox: Url::parse(user.outbox.as_str()).unwrap(),
                outbox: Url::parse(user.outbox.as_str()).unwrap(),
                public_key: user.public_key,
                private_key: user.private_key,
                published: user.published,
                last_refresh: user.last_refresh,
            })),
        }
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind {
            id: self.apub_id.clone(),
            kind: Default::default(),
            preferred_username: self.preferred_username.clone(),
            name: self.name.clone(),
            summary: self.summary.clone(),
            inbox: self.inbox.clone(),
            outbox: self.outbox.clone(),
            public_key: self.public_key(),
            published: self.published.to_rfc3339_opts(SecondsFormat::Millis, true),
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)
            .map(|_| ())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    async fn from_json(
        json: Self::Kind,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            apub_id: json.id,
            preferred_username: json.preferred_username,
            name: json.name,
            summary: json.summary,
            inbox: json.inbox,
            outbox: json.outbox,
            public_key: json.public_key.public_key_pem,
            private_key: None,
            published: json
                .published
                .parse()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
            last_refresh: Local::now().naive_local(),
        })
    }
}

impl Actor for User {
    fn id(&self) -> Url {
        self.apub_id.inner().clone()
    }

    fn inbox(&self) -> Url {
        self.inbox.clone()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }
}
