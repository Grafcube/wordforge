use activitypub_federation::{
    config::Data,
    fetch::webfinger::{extract_webfinger_name, webfinger_resolve_actor},
    protocol::context::WithContext,
    traits::{Collection, Object},
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get, web, HttpResponse,
};
use serde_json::json;
use wordforge_api::{
    objects::{novel_list::NovelList, person::User},
    DbHandle,
};

pub async fn get_user(
    path: web::Path<String>,
    data: Data<DbHandle>,
) -> actix_web::Result<HttpResponse> {
    if path.ends_with(data.domain()) {
        let name = extract_webfinger_name(&format!("acct:{path}"), &data)
            .map_err(|_| ErrorNotFound("Bad request"))?;
        return Ok(HttpResponse::PermanentRedirect()
            .append_header(("Location", format!("/user/{name}")))
            .finish());
    }
    let user = if path.contains('@') {
        webfinger_resolve_actor(&path, &data)
            .await
            .map_err(|_| ErrorNotFound(json!({ "error": "User not found" })))?
    } else {
        User::read_from_username(&path, data.app_data())
            .await
            .map_err(ErrorInternalServerError)?
            .ok_or_else(|| ErrorNotFound(json!({ "error": "User not found" })))?
    }
    .into_json(&data)
    .await
    .map_err(ErrorInternalServerError)?;

    let res = WithContext::new_default(user);
    Ok(HttpResponse::Ok().json(res))
}

#[get("/user/{name}/outbox")]
async fn user_outbox(
    name: web::Path<String>,
    data: Data<DbHandle>,
) -> actix_web::Result<HttpResponse> {
    let owner = User::read_from_username(&name, &data)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or(ErrorNotFound(json!({"error": "User not found"})))?;
    let chapters = NovelList::read_local(&owner, &data)
        .await
        .map_err(ErrorInternalServerError)?;
    let res = WithContext::new_default(chapters);
    Ok(HttpResponse::Ok().json(res))
}
