use crate::{
    objects::{novel::DbNovel, person::User},
    DbHandle,
};
use activitypub_federation::{
    activity_queue::send_activity,
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{activity::AddType, object::ArticleType},
    protocol::context::WithContext,
    traits::{ActivityHandler, Object},
};
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::Local;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::query;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct NewChapter {
    pub title: String,
    pub summary: String,
    pub sensitive: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticle {
    #[serde(rename = "type")]
    kind: ArticleType,
    name: String,
    summary: String,
    sensitive: bool,
}

#[async_trait]
impl Object for NewChapter {
    type DataType = DbHandle;
    type Kind = NewArticle;
    type Error = anyhow::Error;

    async fn read_from_id(
        _object_id: Url,
        _data: &Data<Self::DataType>,
    ) -> anyhow::Result<Option<Self>> {
        Ok(None)
    }

    async fn verify(
        _json: &Self::Kind,
        _expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind {
            kind: Default::default(),
            name: self.title,
            summary: self.summary,
            sensitive: self.sensitive,
        })
    }

    async fn from_json(
        json: Self::Kind,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            title: json.name,
            summary: json.summary,
            sensitive: json.sensitive,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Add {
    actor: ObjectId<User>,
    object: WithContext<NewArticle>,
    target: ObjectId<DbNovel>,
    #[serde(rename = "type")]
    kind: AddType,
    id: Url,
}

impl Add {
    pub async fn send(
        chapter: NewChapter,
        actor: Url,
        inbox: Url,
        scheme: String,
        data: &Data<DbHandle>,
    ) -> anyhow::Result<Url> {
        let user = User::read_from_id(actor.clone(), data)
            .await?
            .ok_or_else(|| anyhow!("Local user not found"))?;
        let id = format!("{}://{}", scheme, data.domain())
            .parse::<Url>()?
            .join(&format!("activities/{}", Local::now().timestamp_nanos()))?;
        let article = NewArticle {
            kind: Default::default(),
            name: chapter.title,
            summary: chapter.summary,
            sensitive: chapter.sensitive,
        };
        let add = Self {
            actor: actor.into(),
            object: WithContext::new_default(article),
            target: inbox.to_string().parse()?,
            kind: Default::default(),
            id: id.clone(),
        };
        let add = WithContext::new_default(add);
        send_activity(add, &user, vec![inbox], data).await?;
        Ok(id)
    }
}

#[async_trait]
impl ActivityHandler for Add {
    type DataType = DbHandle;
    type Error = anyhow::Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, data: &Data<Self::DataType>) -> anyhow::Result<()> {
        let user = self.actor.dereference(data).await?;
        let novel = self.target.dereference_local(data).await?;

        query!(
            "SELECT author FROM author_roles WHERE lower(id)=$1",
            novel.apub_id.to_string().to_lowercase()
        )
        .fetch_all(data.app_data().as_ref())
        .await
        .map_err(|e| anyhow!("{e}"))?
        .into_iter()
        .map(|row| row.author)
        .contains(&user.apub_id)
        .then_some(())
        .ok_or(anyhow!("No write permission"))
    }

    async fn receive(self, data: &Data<Self::DataType>) -> anyhow::Result<()> {
        let chapter = self.object.inner();
        let novel = self.target.dereference_local(data).await?;

        let sequence = query!(
            r#"SELECT max(sequence) AS sequence
               FROM chapters
               WHERE lower(audience)=$1"#,
            novel.apub_id.as_str()
        )
        .fetch_one(data.app_data().as_ref())
        .await?
        .sequence
        .unwrap_or(0);

        let apub_id = novel
            .apub_id
            .parse::<Url>()?
            .join(&sequence.to_string())?
            .to_string();

        query!(
            r#"INSERT INTO chapters
               (apub_id, audience, title, summary, sensitive, sequence)
               VALUES ($1, $2, $3, $4, $5, $6)"#,
            apub_id,
            novel.apub_id.to_string(),
            chapter.name,
            chapter.summary,
            chapter.sensitive,
            sequence
        )
        .execute(data.app_data().as_ref())
        .await?;

        // TODO: Send Announce if accepted. Requires follows to be implemented.

        Ok(())
    }
}
