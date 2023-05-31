use crate::{
    activities::add::{Add, NewChapter},
    objects::novel::DbNovel,
    DbHandle,
};
use activitypub_federation::{
    config::Data,
    fetch::{object_id::ObjectId, webfinger::webfinger_resolve_actor},
    traits::Actor,
};
use actix_session::Session;
use sqlx::query;
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
) -> Result<(), ChapterCreationError> {
    let apub_id: Url = session
        .get::<String>("id")
        .map_err(|e| ChapterCreationError::InternalError(e.to_string()))?
        .ok_or(ChapterCreationError::Unauthorized)?
        .parse()
        .map_err(|e: ParseError| ChapterCreationError::InternalError(e.to_string()))?;
    session.renew();

    let (path, is_local) = if novel.contains('@') {
        (novel, false)
    } else {
        (format!("{}@{}", novel, data.domain()), true)
    };

    let novel: DbNovel = webfinger_resolve_actor(&path, data)
        .await
        .map_err(|_| ChapterCreationError::NotFound)?;

    if is_local {
        let novel_id = novel
            .apub_id
            .parse()
            .map_err(|e: ParseError| ChapterCreationError::InternalError(e.to_string()))?;
        create_chapter(chapter, &novel_id, data)
            .await
            .map_err(|e| ChapterCreationError::InternalError(e.to_string()))?;
    } else {
        let novel_id = novel.inbox();
        Add::send(chapter, apub_id, novel_id, scheme, data)
            .await
            .map_err(|e| ChapterCreationError::InternalError(e.to_string()))?;
    }

    Ok(())
}

pub async fn create_chapter(
    chapter: NewChapter,
    novel: &ObjectId<DbNovel>,
    data: &Data<DbHandle>,
) -> anyhow::Result<()> {
    let novel = novel.dereference_local(data).await?;

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
        chapter.title,
        chapter.summary,
        chapter.sensitive,
        sequence
    )
    .execute(data.app_data().as_ref())
    .await?;

    Ok(())
}
