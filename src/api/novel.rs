use crate::objects::novel::{Genres, Roles};
use activitypub_federation::{config::Data, http_signatures::generate_actor_keypair};
use actix_session::Session;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    post, web, HttpResponse,
};
use isolang::Language;
use itertools::{sorted, Itertools};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct NewNovel {
    title: String,
    summary: String,
    // collaborators: Vec<Url>,
    genre: Genres,
    role: Roles,
    lang: String,
    cw: bool,
    tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct LangRequest {
    query: Option<String>,
    get: Option<String>,
}

#[post("/novel")]
async fn new_novel(
    info: web::Json<NewNovel>,
    data: Data<PgPool>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let apub_id = session
        .get::<String>("id")?
        .ok_or_else(|| ErrorUnauthorized("Not signed in"))?;
    session.renew();
    match create_novel(info.into_inner(), apub_id, data.app_data()).await {
        Ok(id) => Ok(HttpResponse::Ok().body(id.to_string())),
        Err(e) => Err(e),
    }
}

async fn create_novel(info: NewNovel, apub_id: String, conn: &PgPool) -> actix_web::Result<Uuid> {
    let title = info
        .title
        .trim()
        .replace("\r\n", "")
        .replace(['\n', '\r'], "");
    let lang = match Language::from_name(info.lang.as_str()) {
        None => return Err(ErrorBadRequest("Invalid language")),
        Some(l) => l.to_639_1(),
    };
    let tags: Vec<String> = sorted(info.tags)
        .dedup_by(|a, b| a.to_lowercase() == b.to_lowercase())
        .collect();
    let keypair = generate_actor_keypair()?;
    let id = query!(
        "INSERT INTO novels \
        (title, summary, authors, genre, tags, language, content_warning, public_key, private_key) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
        RETURNING id",
        title.trim(),
        info.summary.trim(),
        &[apub_id.clone(),],
        info.genre.to_string(),
        tags.as_slice(),
        lang,
        info.cw,
        keypair.public_key,
        keypair.private_key
    )
    .fetch_one(conn)
    .await
    .map_err(ErrorInternalServerError)?
    .id;

    if info.role != Roles::None {
        query!(
            "INSERT INTO author_roles VALUES ($1, $2, $3)",
            id,
            apub_id,
            info.role.to_string()
        )
        .execute(conn)
        .await
        .map_err(ErrorInternalServerError)?;
    }
    Ok(id)
}
