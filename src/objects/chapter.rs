use super::novel::DbNovel;
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{collection::OrderedCollectionType, object::ArticleType},
    protocol::verification::verify_domains_match,
    traits::{Collection, Object},
};
use async_trait::async_trait;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chapter {
    pub apub_id: String,
    pub audience: String,
    pub title: String,
    pub summary: String,
    pub sensitive: bool,
    pub content: String,
    pub published: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    pub last_refresh: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    id: ObjectId<Chapter>,
    #[serde(rename = "type")]
    kind: ArticleType,
    name: String,
    audience: Url,
    summary: String,
    sensitive: bool,
    content: String,
    published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<String>,
}

#[async_trait]
impl Object for Chapter {
    type DataType = PgPool;
    type Kind = Article;
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
            r#"SELECT apub_id, audience, title, summary, sensitive, content,
                 published, updated, last_refresh
               FROM chapters
               WHERE lower(apub_id)=$1"#,
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
            name: self.title,
            audience: self.audience.parse()?,
            summary: self.summary,
            sensitive: self.sensitive,
            content: self.content,
            published: self.published.to_rfc3339_opts(SecondsFormat::Millis, true),
            updated: self
                .updated
                .map(|t| t.to_rfc3339_opts(SecondsFormat::Millis, true)),
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
            audience: json.audience.into(),
            title: json.name,
            summary: json.summary,
            sensitive: json.sensitive,
            content: json.content,
            published: json.published.parse()?,
            updated: match json.updated {
                None => None,
                Some(u) => Some(u.parse()?),
            },
            last_refresh: Local::now().naive_local(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterList {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,
    total_items: usize,
    pub ordered_items: Vec<ObjectId<Chapter>>,
}

#[async_trait]
impl Collection for ChapterList {
    type Owner = DbNovel;
    type DataType = PgPool;
    type Kind = ChapterList;
    type Error = anyhow::Error;

    async fn read_local(
        owner: &Self::Owner,
        data: &Data<Self::DataType>,
    ) -> Result<Self::Kind, Self::Error> {
        let chapters: Vec<ObjectId<Chapter>> = query!(
            r#"SELECT apub_id
               FROM chapters
               WHERE lower(audience)=$1
               ORDER BY sequence DESC"#,
            owner.apub_id.to_string().to_lowercase()
        )
        .fetch_all(data.app_data())
        .await?
        .iter()
        .map(|chapter| chapter.apub_id.parse().unwrap())
        .collect();

        Ok(Self::Kind {
            kind: Default::default(),
            total_items: chapters.len(),
            ordered_items: chapters,
        })
    }

    async fn verify(
        _json: &Self::Kind,
        _expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn from_json(
        json: Self::Kind,
        _owner: &Self::Owner,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        Ok(json)
    }
}
