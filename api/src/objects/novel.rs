use crate::{
    activities,
    enums::{Genres, Roles},
    util::USERNAME_RE,
    DbHandle,
};
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::GroupType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{ActivityHandler, Actor, Object},
};
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat, Utc};
use isolang::Language;
use serde::{Deserialize, Serialize};
use sqlx::query;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum NovelAcceptedActivities {
    Add(activities::add::Add),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub apub_id: String,
    pub role: Roles,
}

#[derive(Serialize, Deserialize)]
pub struct DbNovel {
    pub apub_id: String,
    pub preferred_username: Uuid,
    pub title: String,
    pub summary: String,
    pub authors: Vec<Author>,
    pub genre: Genres,
    pub tags: Vec<String>,
    pub language: Language,
    pub sensitive: bool,
    pub inbox: String,
    pub outbox: String,
    pub public_key: String,
    #[serde(skip_serializing)]
    private_key: Option<String>,
    pub published: DateTime<Utc>,
    pub last_refresh: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Novel {
    id: ObjectId<DbNovel>,
    #[serde(rename = "type")]
    kind: GroupType,
    #[validate(regex(path = "USERNAME_RE", message = "Invalid username"))]
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub authors: Vec<Author>,
    attributed_to: Vec<Url>,
    pub genre: Genres,
    pub tags: Vec<String>,
    pub language: String,
    pub sensitive: bool,
    inbox: Url,
    outbox: Url,
    public_key: PublicKey,
    pub published: String,
}

impl DbNovel {
    pub async fn read_from_uuid(
        uuid: Uuid,
        data: &Data<DbHandle>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let apub_id = match query!(
            "SELECT apub_id FROM novels WHERE preferred_username=$1",
            uuid
        )
        .fetch_optional(data.app_data().as_ref())
        .await?
        {
            None => return Ok(None),
            Some(v) => v.apub_id,
        };

        Self::read_from_id(Url::parse(apub_id.as_str()).unwrap(), data).await
    }
}

#[async_trait]
impl Object for DbNovel {
    type DataType = DbHandle;
    type Kind = Novel;
    type Error = anyhow::Error;

    fn last_refreshed_at(&self) -> Option<NaiveDateTime> {
        Some(self.last_refresh)
    }

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let authors: Vec<Author> = query!(
            "SELECT author as apub_id, role FROM author_roles WHERE lower(id)=$1",
            object_id.to_string().to_lowercase(),
        )
        .fetch_all(data.app_data().as_ref())
        .await?
        .iter()
        .map(|author| Author {
            apub_id: author.apub_id.clone(),
            role: Roles::from_str(author.role.as_str()).unwrap(),
        })
        .collect();

        let novel = query!(
            r#"SELECT apub_id, preferred_username, title, summary, genre, tags,
               language, sensitive, inbox, outbox, public_key, private_key,
               published, last_refresh
               FROM novels WHERE lower(apub_id)=$1"#,
            object_id.to_string().to_lowercase()
        )
        .fetch_optional(data.app_data().as_ref())
        .await?
        .map(|row| Self {
            apub_id: row.apub_id,
            preferred_username: row.preferred_username,
            title: row.title,
            summary: row.summary,
            authors,
            genre: Genres::from_str(row.genre.as_str()).unwrap(),
            tags: row.tags,
            language: Language::from_639_1(row.language.as_str()).unwrap(),
            sensitive: row.sensitive,
            inbox: row.inbox,
            outbox: row.outbox,
            public_key: row.public_key,
            private_key: row.private_key,
            published: row.published,
            last_refresh: row.last_refresh,
        });

        Ok(novel)
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind {
            id: self.apub_id.parse()?,
            kind: Default::default(),
            preferred_username: self.preferred_username.to_string().to_lowercase(),
            name: self.title.clone(),
            summary: self.summary.clone(),
            authors: self.authors.clone(),
            attributed_to: self
                .authors
                .iter()
                .map(|a| a.apub_id.parse().unwrap())
                .collect(),
            genre: self.genre.clone(),
            tags: self.tags.clone(),
            language: self.language.to_639_1().unwrap().to_string(),
            sensitive: self.sensitive,
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
            preferred_username: json.preferred_username.parse()?,
            title: json.name,
            summary: json.summary,
            authors: json.authors,
            genre: json.genre,
            tags: json.tags,
            language: Language::from_639_1(json.language.as_str())
                .ok_or_else(|| anyhow!("Unknown language"))?,
            sensitive: json.sensitive,
            inbox: json.inbox.into(),
            outbox: json.outbox.into(),
            public_key: json.public_key.public_key_pem,
            private_key: None,
            published: json.published.parse()?,
            last_refresh: Local::now().naive_local(),
        })
    }
}

impl Actor for DbNovel {
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
