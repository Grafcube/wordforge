use crate::{
    activities::add::{Add, NewChapter},
    objects::novel::DbNovel,
    DbHandle,
};
use activitypub_federation::{
    config::Data, fetch::webfinger::webfinger_resolve_actor, traits::Actor,
};
use actix_session::Session;
use url::{ParseError, Url};

pub enum ChapterCreationError {
    InternalError(String),
    NotFound,
    Unauthorized,
}

pub async fn new_chapter(
    novel: String,
    chapter: NewChapter,
    session: Session,
    data: &Data<DbHandle>,
    scheme: String,
) -> Result<String, ChapterCreationError> {
    let apub_id: Url = session
        .get::<String>("id")
        .map_err(|e| ChapterCreationError::InternalError(e.to_string()))?
        .ok_or(ChapterCreationError::Unauthorized)?
        .parse()
        .map_err(|e: ParseError| ChapterCreationError::InternalError(e.to_string()))?;
    session.renew();

    let path = if novel.contains('@') {
        novel
    } else {
        format!("{}@{}", novel, data.domain())
    };

    let novel_id: DbNovel = webfinger_resolve_actor(&path, data)
        .await
        .map_err(|_| ChapterCreationError::NotFound)?;
    let novel_id = novel_id.inbox();
    let activity_id = Add::send(chapter, apub_id, novel_id, scheme, data)
        .await
        .map_err(|e| ChapterCreationError::InternalError(e.to_string()))?;

    Ok(activity_id.to_string())
}
