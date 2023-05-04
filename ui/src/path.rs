use leptos::*;
use leptos_router::*;

#[derive(Params, Debug, PartialEq, Clone)]
pub struct NovelViewParams {
    pub uuid: String,
}
