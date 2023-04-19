use crate::{
    instance::DbHandle,
    objects::{novel::DbNovel, person::User},
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
use serde::{Deserialize, Serialize};
use sqlx::query;
use url::Url;

#[derive(Serialize, Deserialize)]
struct NewChapter {
    title: String,
    summary: String,
    sensitive: bool,
    content: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewArticle {
    #[serde(rename = "type")]
    kind: ArticleType,
    name: String,
    summary: String,
    sensitive: bool,
    content: String,
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
            content: self.content,
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
            content: json.content,
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
        chapter: NewArticle,
        actor: Url,
        inbox: Url,
        data: &Data<DbHandle>,
    ) -> anyhow::Result<Url> {
        let user = User::read_from_id(actor.clone(), data)
            .await?
            .ok_or_else(|| anyhow!("Local user not found"))?;
        let id = data
            .domain()
            .parse::<Url>()?
            .join(&format!("activities/{}", Local::now().timestamp_nanos()))?;
        let add = Self {
            actor: actor.into(),
            object: WithContext::new_default(chapter),
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

    async fn verify(&self, _data: &Data<Self::DataType>) -> anyhow::Result<()> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> anyhow::Result<()> {
        let chapter = self.object.inner();
        let user = self.actor.dereference(data).await?;
        let novel = self.target.dereference_local(data).await?;

        let authors = query!(
            "SELECT authors FROM novels WHERE lower(apub_id)=$1",
            novel.apub_id.to_string().to_lowercase()
        )
        .fetch_one(data.app_data().as_ref())
        .await
        .map_err(|_| anyhow!("Novel not found"))?
        .authors;

        if !authors.contains(&user.apub_id) {
            return Err(anyhow!("No write permission: {}", novel.apub_id));
        }

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
               (apub_id, audience, title, summary, sensitive, content, sequence)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            apub_id,
            novel.apub_id.to_string(),
            chapter.name,
            chapter.summary,
            chapter.sensitive,
            chapter.content,
            sequence
        )
        .execute(data.app_data().as_ref())
        .await?;

        Ok(())
    }
}
