use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Display, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    None,
    Writer,
    Adapter,
    Artist,
    Penciller,
    Inker,
    Colorist,
    Letterer,
    CoverArtist,
    Photographer,
    Editor,
    Assistant,
    Translator,
    Other,
}

#[derive(Display, Serialize, Deserialize)]
pub enum Genres {
    Adult,
    Adventure,
    Alternative,
    Biography,
    Caricature,
    Children,
    Computer,
    Crime,
    Education,
    Fantasy,
    History,
    Horror,
    Humor,
    Manga,
    Military,
    Mystery,
    NonFiction,
    Politics,
    RealLife,
    Religion,
    Romance,
    ScienceFiction,
    Sports,
    Superhero,
    Western,
    Other,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    apub_id: String,
    role: Roles,
}

#[derive(Serialize, Deserialize)]
pub struct Novel {
    id: Uuid,
    title: String,
    summary: String,
    authors: Vec<Author>,
    genre: Genres,
    tags: Vec<String>,
    pub public_key: String,
    #[serde(skip_serializing)]
    private_key: Option<String>,
    pub published: DateTime<Utc>,
    pub last_refresh: NaiveDateTime,
}
