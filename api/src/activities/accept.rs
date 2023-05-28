use crate::{
    activities::add::NewArticle,
    objects::{chapter::Chapter, novel::DbNovel},
    DbHandle,
};
use activitypub_federation::{
    activity_queue::send_activity, config::Data, fetch::object_id::ObjectId,
    kinds::activity::AcceptType, protocol::context::WithContext, traits::ActivityHandler,
};
use async_trait::async_trait;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::query;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Accept {
    actor: ObjectId<DbNovel>,
    object: Url,
    target: ObjectId<Chapter>,
    #[serde(rename = "type")]
    kind: AcceptType,
    id: Url,
}

impl Accept {
    pub async fn send(
        novel: &DbNovel,
        chapter: &NewArticle,
        activity: Url,
        inbox: Url,
        data: &Data<DbHandle>,
    ) -> anyhow::Result<Url> {
        let id = data
            .domain()
            .parse::<Url>()?
            .join(&format!("activities/{}", Local::now().timestamp_nanos()))?;

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

        let accept = Self {
            actor: novel.apub_id.to_string().parse()?,
            object: activity,
            target: apub_id.parse()?,
            kind: Default::default(),
            id: id.clone(),
        };

        let accept = WithContext::new_default(accept);
        send_activity(accept, novel, vec![inbox], data).await?;
        Ok(id)
    }
}

#[async_trait]
impl ActivityHandler for Accept {
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

    async fn receive(self, _data: &Data<Self::DataType>) -> anyhow::Result<()> {
        Ok(())
    }
}
