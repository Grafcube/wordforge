use crate::{
    instance::DatabaseHandle, objects::person::User, schema::users::dsl::*, util::generate_id,
};
use activitypub_federation::{core::signatures::generate_actor_keypair, request_data::RequestData};
use actix_web::{error::ErrorInternalServerError, post, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NewUser {
    display_name: String,
    username: String,
    email: String,
}

#[post("/accounts")]
async fn create(
    pool: RequestData<DatabaseHandle>,
    info: web::Json<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get_pool().get().map_err(ErrorInternalServerError)?;
    match create_account(info.into_inner(), &mut conn) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(e),
    }
    .map_err(ErrorInternalServerError)
}

fn create_account(info: NewUser, conn: &mut PgConnection) -> anyhow::Result<User> {
    let uid = loop {
        let uid = generate_id();
        break match users.filter(id.eq(uid)).select(id).first::<u32>(conn) {
            Ok(_) => continue,
            Err(_) => uid,
        };
    };
    let user = User::new(
        uid,
        info.username,
        info.display_name,
        info.email,
        generate_actor_keypair()?,
    );
    diesel::insert_into(users)
        .values(user)
        .get_result(conn)
        .map_err(anyhow::Error::new)
}
