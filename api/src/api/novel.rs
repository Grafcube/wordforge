use crate::{
    enums::{Genres, Roles},
    objects::novel::{DbNovel, Novel},
    util::AppState,
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
use itertools::{sorted, Itertools};
use serde::{Deserialize, Serialize};
use sqlx::query;
use uuid::Uuid;

pub enum CreateNovelResult {
    Ok(String),
    Unauthorized(String),
    BadRequest(String),
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
) -> CreateNovelResult {
    let apub_id = match session.get::<String>("id") {
        Err(e) => return CreateNovelResult::InternalServerError(e.to_string()),
        Ok(Some(u)) => u,
        Ok(None) => return CreateNovelResult::Unauthorized("Not signed in".to_string()),
    };
    session.renew();

    let scheme = state.scheme.clone();

    let re = regex::Regex::new(r#"[\r\n]+"#).unwrap();
    let title = re.replace_all(info.title.trim(), "");
    let lang = match Language::from_name(info.lang.as_str()) {
        None => return CreateNovelResult::BadRequest("Invalid language".to_string()),
        Some(l) => l.to_639_1(),
    };
    let tags: Vec<String> = sorted(info.tags.split(',').map(|t| t.trim().to_string()))
        .dedup_by(|a, b| a.to_lowercase() == b.to_lowercase())
        .collect();
    let uuid = Uuid::new_v4();
    let keypair = match generate_actor_keypair() {
        Ok(k) => k,
        Err(e) => return CreateNovelResult::InternalServerError(e.to_string()),
    };
    let url = format!(
        "{}://{}/novel/{}",
        scheme.clone(),
        pool.domain(),
        uuid.to_string().to_lowercase()
    );
    let id = match query!(
        r#"INSERT INTO novels
           (apub_id, preferred_username, title, summary, authors, genre, tags,
           language, sensitive, inbox, outbox, public_key, private_key)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
           RETURNING apub_id"#,
        url,
        uuid,
        title.trim(),
        info.summary.trim(),
        &[apub_id.clone(),],
        info.genre.to_string(),
        tags.as_slice(),
        lang,
        info.sensitive,
        format!("{}://{}/inbox", scheme.clone(), url),
        format!("{}://{}/outbox", scheme.clone(), url),
        keypair.public_key,
        keypair.private_key
    )
    .fetch_one(pool.app_data().as_ref())
    .await
    {
        Ok(row) => row.apub_id,
        Err(e) => return CreateNovelResult::InternalServerError(e.to_string()),
    };

    if info.role != Roles::None {
        if let Err(e) = query!(
            "INSERT INTO author_roles VALUES ($1, $2, $3)",
            id,
            apub_id,
            info.role.to_string()
        )
        .execute(pool.app_data().as_ref())
        .await
        {
            return CreateNovelResult::InternalServerError(e.to_string());
        };
    }
    CreateNovelResult::Ok(uuid.to_string().to_lowercase())
}

pub enum GetNovelResult {
    Ok(Box<Novel>),
    PermanentRedirect(String),
    WebfingerNotFound,
    NovelNotFound,
    InternalServerError(String),
}

pub async fn get_novel(uuid: String, data: &Data<DbHandle>) -> GetNovelResult {
    if uuid.ends_with(data.domain()) {
        let id = match extract_webfinger_name(&format!("acct:{uuid}"), data) {
            Ok(v) => v,
            Err(_e) => return GetNovelResult::WebfingerNotFound,
        };
        return GetNovelResult::PermanentRedirect(format!("/novel/{id}"));
    }
    let novel = if uuid.contains('@') {
        match webfinger_resolve_actor(&uuid, data).await {
            Ok(v) => v,
            Err(_) => return GetNovelResult::NovelNotFound,
        }
    } else {
        let id = match Uuid::parse_str(&uuid) {
            Ok(v) => v,
            Err(_) => return GetNovelResult::NovelNotFound,
        };
        match DbNovel::read_from_uuid(id, data).await {
            Ok(Some(v)) => v,
            Err(e) => return GetNovelResult::InternalServerError(e.to_string()),
            Ok(None) => return GetNovelResult::NovelNotFound,
        }
    };
    match novel.into_json(data).await {
        Ok(v) => GetNovelResult::Ok(Box::new(v)),
        Err(e) => GetNovelResult::InternalServerError(e.to_string()),
    }
}
