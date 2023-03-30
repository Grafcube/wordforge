use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::Display;
use isolang::Language;
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
    Adventure,
    Alternative,
    Biography,
    Comedy,
    Crime,
    Education,
    Fantasy,
    History,
    Horror,
    Humor,
    Mystery,
    NonFiction,
    Romance,
    ScienceFiction,
    Sports,
    Superhero,
    Thriller,
    Western,
    Other,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub apub_id: String,
    pub role: Roles,
}

#[derive(Serialize, Deserialize)]
pub struct Novel {
    pub id: Uuid,
    pub title: String,
    pub summary: String,
    pub authors: Vec<Author>,
    pub genre: Genres,
    pub tags: Vec<String>,
    pub language: Language,
    pub content_warning: Option<String>,
    pub public_key: String,
    #[serde(skip_serializing)]
    private_key: Option<String>,
    pub published: DateTime<Utc>,
    pub last_refresh: NaiveDateTime,
}
