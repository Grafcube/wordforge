use activitypub_federation::config::Data;
use actix_session::Session;
use actix_web::{
    error::{ErrorNotFound, ErrorUnauthorized},
    get, HttpResponse,
};
use sqlx::query;
use wordforge_api::DbHandle;

#[get("/validate")]
async fn validate(pool: Data<DbHandle>, session: Session) -> actix_web::Result<HttpResponse> {
    let id = session
        .get::<String>("id")?
        .ok_or_else(|| ErrorUnauthorized("Not signed in"))?;
    session.renew();
    let name = query!("SELECT name FROM users WHERE apub_id=$1", id)
        .fetch_one(pool.app_data().as_ref())
        .await
        .map_err(ErrorNotFound)?
        .name;
    Ok(HttpResponse::Ok().body(name))
}
