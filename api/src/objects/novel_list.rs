use super::person::User;
use crate::DbHandle;
use activitypub_federation::{
    config::Data, kinds::collection::OrderedCollectionType, traits::Collection,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::query;
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NovelList {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,
    total_items: usize,
    pub ordered_items: Vec<String>,
}

#[async_trait]
impl Collection for NovelList {
    type Owner = User;
    type DataType = DbHandle;
    type Kind = NovelList;
    type Error = anyhow::Error;

    async fn read_local(
        owner: &Self::Owner,
        data: &Data<Self::DataType>,
    ) -> Result<Self::Kind, Self::Error> {
        let novels: Vec<String> = query!(
            r#"
            SELECT
                id
            FROM
                author_roles, novels
            WHERE
                lower(author)=$1 AND
                author_roles.id = novels.apub_id
            ORDER BY published DESC
        "#,
            owner.apub_id.to_string().to_lowercase()
        )
        .fetch_all(data.app_data().as_ref())
        .await?
        .into_iter()
        .map(|novel| novel.id)
        .collect();

        Ok(Self::Kind {
            kind: Default::default(),
            total_items: novels.len(),
            ordered_items: novels,
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
