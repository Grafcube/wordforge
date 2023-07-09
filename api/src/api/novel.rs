use crate::{
    enums::{Genres, Roles},
    objects::novel::{DbNovel, Novel},
    util::{AppState, TAG_RE},
    DbHandle,
};
use activitypub_federation::{
    config::Data,
    fetch::webfinger::{extract_webfinger_name, webfinger_resolve_actor},
    http_signatures::generate_actor_keypair,
    traits::Object,
};
use actix_session::Session;
use actix_web::web;
use isolang::Language;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::query;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CreateNovelError {
    #[error("Create Novel: Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Create Novel: BadRequest: {0}")]
    BadRequest(String),
    #[error("Create Novel: InternalServerError: {0}")]
    InternalServerError(String),
}

#[derive(Serialize, Deserialize)]
pub struct NewNovel {
    pub title: String,
    pub summary: String,
    pub genre: Genres,
    pub role: Roles,
    pub lang: String,
    pub sensitive: bool,
    pub tags: String,
}

pub async fn create_novel(
    state: web::Data<AppState>,
    pool: Data<DbHandle>,
    session: Session,
    info: NewNovel,
) -> Result<String, CreateNovelError> {
    let apub_id = match session.get::<String>("id") {
        Err(e) => return Err(CreateNovelError::InternalServerError(e.to_string())),
        Ok(Some(u)) => u,
        Ok(None) => return Err(CreateNovelError::Unauthorized("Not signed in".to_string())),
    };
    session.renew();

    let scheme = &state.scheme;

    let re = regex::Regex::new(r#"[\r\n]+"#).unwrap();
    let title = re.replace_all(info.title.trim(), "");
    let lang = match Language::from_name(info.lang.as_str()) {
        None => return Err(CreateNovelError::BadRequest("Invalid language".to_string())),
        Some(l) => l.to_639_1(),
    };
    let tags = TAG_RE
        .find_iter(&info.tags)
        .map(|t| t.as_str().to_string())
        .sorted_by(|a, b| a.cmp(b))
        .dedup_by(|a, b| a.to_lowercase() == b.to_lowercase())
        .collect_vec();
    let uuid = Uuid::new_v4();
    let keypair = generate_actor_keypair()
        .map_err(|e| CreateNovelError::InternalServerError(e.to_string()))?;
    let url = format!(
        "{}://{}/novel/{}",
        scheme,
        pool.domain(),
        uuid.to_string().to_lowercase()
    );
    let id = match query!(
        r#"INSERT INTO novels
           (apub_id, preferred_username, title, summary, genre, tags, language,
             sensitive, inbox, outbox, public_key, private_key)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
           RETURNING apub_id"#,
        url,
        uuid,
        title.trim(),
        info.summary.trim(),
        info.genre.to_string(),
        tags.as_slice(),
        lang,
        info.sensitive,
        format!("{}/inbox", url),
        format!("{}/outbox", url),
        keypair.public_key,
        keypair.private_key
    )
    .fetch_one(pool.app_data().as_ref())
    .await
    {
        Ok(row) => row.apub_id,
        Err(e) => return Err(CreateNovelError::InternalServerError(e.to_string())),
    };

    query!(
        "INSERT INTO author_roles VALUES ($1, $2, $3)",
        id,
        apub_id,
        info.role.to_string()
    )
    .execute(pool.app_data().as_ref())
    .await
    .map_err(|e| CreateNovelError::InternalServerError(e.to_string()))?;
    Ok(uuid.to_string().to_lowercase())
}

#[derive(Debug, Error)]
pub enum GetNovelError {
    #[error("GetNovel PermanentRedirect: {0}")]
    PermanentRedirect(String),
    #[error("GetNovel WebfingerNotFound")]
    WebfingerNotFound,
    #[error("GetNovel NovelNotFound")]
    NovelNotFound,
    #[error("GetNovel InternalServerError: {0}")]
    InternalServerError(String),
}

pub async fn get_novel(uuid: String, data: &Data<DbHandle>) -> Result<Box<Novel>, GetNovelError> {
    if uuid.ends_with(data.domain()) {
        let id = extract_webfinger_name(&format!("acct:{uuid}"), data)
            .map_err(|_| GetNovelError::WebfingerNotFound)?;
        return Err(GetNovelError::PermanentRedirect(format!("/novel/{id}")));
    }
    let novel = if uuid.contains('@') {
        webfinger_resolve_actor(&uuid, data)
            .await
            .map_err(|_| GetNovelError::NovelNotFound)?
    } else {
        let id = Uuid::parse_str(&uuid).map_err(|_| GetNovelError::NovelNotFound)?;
        match DbNovel::read_from_uuid(id, data).await {
            Ok(Some(v)) => v,
            Err(e) => return Err(GetNovelError::InternalServerError(e.to_string())),
            Ok(None) => return Err(GetNovelError::NovelNotFound),
        }
    };
    match novel.into_json(data).await {
        Ok(v) => Ok(Box::new(v)),
        Err(e) => Err(GetNovelError::InternalServerError(e.to_string())),
    }
}
