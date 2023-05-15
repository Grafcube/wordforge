use leptos::*;
use leptos_router::*;

#[derive(Params, Debug, PartialEq, Clone)]
pub struct NovelViewParams {
    pub uuid: String,
}

#[derive(Params, Debug, PartialEq, Clone)]
pub struct AuthQueries {
    pub redirect_to: String,
}
