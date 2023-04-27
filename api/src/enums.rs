use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq)]
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
