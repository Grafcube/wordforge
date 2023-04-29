use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumIter, Serialize, Deserialize, PartialEq)]
pub enum Genres {
    Action,
    Adventure,
    Comedy,
    Drama,
    Educational,
    Fantasy,
    History,
    Horror,
    Mystery,
    #[strum(serialize = "Non-Fiction")]
    NonFiction,
    Romance,
    #[strum(serialize = "Sci-Fi")]
    SciFi,
    #[strum(serialize = "Slice of Life")]
    SliceOfLife,
    Sports,
    Superhero,
    Thriller,
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
