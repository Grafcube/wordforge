use actix_web::{post, Error, HttpResponse};

#[post("/accounts")]
pub fn create() -> Result<HttpResponse, Error> {
    unimplemented!()
}
