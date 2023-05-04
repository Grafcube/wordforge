use crate::{
    activities::{self, add::NewArticle},
    objects::{
        chapter::ChapterList,
        novel::{DbNovel, NovelAcceptedActivities},
        person::User,
    },
};
use activitypub_federation::{
    actix_web::inbox::receive_activity,
    config::Data,
    fetch::webfinger::{extract_webfinger_name, webfinger_resolve_actor},
    protocol::context::WithContext,
    traits::{Actor, Collection, Object},
};
use actix_session::Session;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    get, post,
    web::{self, Bytes},
    HttpRequest, HttpResponse,
};
use serde_json::json;
use url::Url;
use uuid::Uuid;
use wordforge_api::{
    api::novel::{create_novel, CreateNovelResult, NewNovel},
    DbHandle,
};

#[post("/novel")]
async fn new_novel(
    info: web::Json<NewNovel>,
    data: Data<DbHandle>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    match create_novel(data, session, info.into_inner()).await {
        CreateNovelResult::Ok(id) => Ok(HttpResponse::Ok().body(id)),
        CreateNovelResult::Unauthorized(e) => Err(ErrorUnauthorized(e)),
        CreateNovelResult::BadRequest(e) => Err(ErrorBadRequest(e)),
        CreateNovelResult::InternalServerError(e) => Err(ErrorInternalServerError(e)),
    }
}

pub async fn get_novel(
    path: web::Path<String>,
    data: Data<DbHandle>,
) -> actix_web::Result<HttpResponse> {
    if path.ends_with(data.domain()) {
        let id = extract_webfinger_name(&format!("acct:{path}"), &data)
            .map_err(|_| ErrorNotFound(json!({ "error": "Bad request" })))?;
        return Ok(HttpResponse::PermanentRedirect()
            .append_header(("Location", format!("/novel/{id}")))
            .finish());
    }
    let novel = if path.contains('@') {
        webfinger_resolve_actor(&path, &data)
            .await
            .map_err(|_| ErrorNotFound(json!({ "error": "Novel not found" })))?
    } else {
        let id = Uuid::parse_str(&path)
            .map_err(|_| ErrorBadRequest(json!({ "error": "Novel not found" })))?;
        DbNovel::read_from_uuid(id, &data)
            .await
            .map_err(ErrorInternalServerError)?
            .ok_or_else(|| ErrorNotFound(json!({ "error": "Novel not found" })))?
    }
    .into_json(&data)
    .await
    .map_err(ErrorInternalServerError)?;
    let res = WithContext::new_default(novel);
    Ok(HttpResponse::Ok().json(res))
}

#[post("/novel/{novel}/create")]
async fn add_chapter(
    path: web::Path<String>,
    info: web::Json<NewArticle>,
    session: Session,
    data: Data<DbHandle>,
) -> actix_web::Result<HttpResponse> {
    let apub_id: Url = session
        .get::<String>("id")?
        .ok_or_else(|| ErrorUnauthorized("Not signed in"))?
        .parse()
        .map_err(ErrorInternalServerError)?;
    session.renew();

    let novel_id: DbNovel = webfinger_resolve_actor(&path, &data)
        .await
        .map_err(|_| ErrorNotFound("Novel not found"))?;
    let novel_id = novel_id.inbox();
    let activity_id = activities::add::Add::send(info.into_inner(), apub_id, novel_id, &data)
        .await
        .map_err(ErrorInternalServerError)?;

    // TODO: Response with Chapter apub_id
    Ok(HttpResponse::Ok().body(activity_id.to_string()))
}

#[post("/novel/{uuid}/inbox")]
async fn novel_inbox(
    data: Data<DbHandle>,
    request: HttpRequest,
    payload: Bytes,
) -> actix_web::Result<HttpResponse> {
    receive_activity::<WithContext<NovelAcceptedActivities>, User, DbHandle>(
        request, payload, &data,
    )
    .await
    .map_err(ErrorInternalServerError)
}

#[get("/novel/{uuid}/outbox")]
async fn novel_outbox(
    uuid: web::Path<Uuid>,
    data: Data<DbHandle>,
) -> actix_web::Result<HttpResponse> {
    let owner = DbNovel::read_from_uuid(uuid.into_inner(), &data)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or(ErrorNotFound(json!({"error": "Novel not found"})))?;
    let chapters = ChapterList::read_local(&owner, &data)
        .await
        .map_err(ErrorInternalServerError)?;
    let res = WithContext::new_default(chapters);
    Ok(HttpResponse::Ok().json(res))
}
