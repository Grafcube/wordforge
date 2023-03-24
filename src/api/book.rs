use crate::objects::book::{Genres, ReadingDirection, Roles};
use activitypub_federation::{config::Data, http_signatures::generate_actor_keypair};
use actix_session::Session;
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    post, web, HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct NewBook {
    title: String,
    summary: String,
    // collaborators: Vec<Url>,
    genre: Genres,
    role: Roles,
    tags: Vec<String>,
    reading_direction: ReadingDirection,
}

#[post("/book")]
async fn new_book(
    info: web::Json<NewBook>,
    data: Data<PgPool>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    let apub_id = session
        .get::<String>("id")?
        .ok_or_else(|| ErrorUnauthorized("Not signed in"))?;
    session.renew();
    match create_book(info.into_inner(), apub_id, data.app_data()).await {
        Ok(id) => Ok(HttpResponse::Ok().body(id.to_string())),
        Err(e) => Err(e),
    }
}

async fn create_book(info: NewBook, apub_id: String, conn: &PgPool) -> actix_web::Result<Uuid> {
    let keypair = generate_actor_keypair()?;
    let id = query!(
        "INSERT INTO book \
        (title, summary, authors, genre, tags, reading_direction, public_key, private_key) \
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
        RETURNING id",
        info.title,
        info.summary,
        &[apub_id.clone(),],
        info.genre.to_string(),
        info.tags.as_slice(),
        info.reading_direction.to_string(),
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
