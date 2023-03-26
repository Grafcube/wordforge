use crate::objects::person::User;
use activitypub_federation::{config::Data, protocol::context::WithContext, traits::Object};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    web, HttpResponse,
};
use sqlx::PgPool;

pub async fn get_user(
    pool: Data<PgPool>,
    username: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let user = User::read_from_username(username.into_inner().as_str(), pool.app_data())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("User not found"))?;
    let user = user.into_json(&pool).await?;
    let res = WithContext::new_default(user);
    Ok(HttpResponse::Ok().json(res))
}
