use std::str::FromStr;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::GroupType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat, Utc};
use isolang::Language;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use strum::{Display, EnumString};
use url::Url;
use uuid::Uuid;

#[derive(Clone, Debug, Display, EnumString, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    None,
    Writer,
    Adapter,
    Artist,
    Penciller,
    Inker,
    Colorist,
    Letterer,
    CoverArtist,
    Photographer,
    Editor,
    Assistant,
    Translator,
    Other,
}

#[derive(Clone, Debug, Display, EnumString, Serialize, Deserialize)]
pub enum Genres {
    Adventure,
    Alternative,
    Biography,
    Comedy,
    Crime,
    Education,
    Fantasy,
    History,
    Horror,
    Humor,
    Mystery,
    NonFiction,
    Romance,
    ScienceFiction,
    Sports,
    Superhero,
    Thriller,
    Western,
    Other,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Novel {
    id: ObjectId<DbNovel>,
    #[serde(rename = "type")]
    kind: GroupType,
    preferred_username: String,
    name: String,
    summary: String,
    authors: Vec<Author>,
    genre: Genres,
    tags: Vec<String>,
    language: String,
    sensitive: bool,
    inbox: Url,
    outbox: Url,
    public_key: PublicKey,
    published: String,
}

#[async_trait]
impl Object for DbNovel {
    type DataType = PgPool;
    type Kind = Novel;
    type Error = std::io::Error;

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
        .fetch_all(data.app_data())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
        .iter()
        .map(|author| Author {
            apub_id: author.apub_id.clone(),
            role: Roles::from_str(author.role.as_str()).unwrap(),
        })
        .collect();

        Ok(query!(
            r#"SELECT apub_id, preferred_username, title, summary, genre, tags,
               language, sensitive, inbox, outbox, public_key, private_key,
               published, last_refresh
               FROM novels WHERE lower(apub_id)=$1"#,
            object_id.to_string().to_lowercase()
        )
        .fetch_optional(data.app_data())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
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
        }))
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind {
            id: self.apub_id.parse().unwrap(),
            kind: Default::default(),
            preferred_username: self.preferred_username.to_string().to_lowercase(),
            name: self.title.clone(),
            summary: self.summary.clone(),
            authors: self.authors.clone(),
            genre: self.genre.clone(),
            tags: self.tags.clone(),
            language: self.language.to_639_1().unwrap().to_string(),
            sensitive: self.sensitive,
            inbox: self.inbox.parse().unwrap(),
            outbox: self.outbox.parse().unwrap(),
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
            apub_id: json.id.into_inner().into(),
            preferred_username: json
                .preferred_username
                .parse()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
            title: json.name,
            summary: json.summary,
            authors: json.authors,
            genre: json.genre,
            tags: json.tags,
            language: Language::from_639_1(json.language.as_str()).ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "Unknown language")
            })?,
            sensitive: json.sensitive,
            inbox: json.inbox.into(),
            outbox: json.outbox.into(),
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
