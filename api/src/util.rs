use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;

lazy_static! {
    pub static ref USERNAME_RE: Regex =
        Regex::new(r"(?i)^[a-z0-9_]+([a-z0-9_\.-]+[a-z0-9_]+)?$").unwrap();
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub scheme: String,
    pub client: Client,
}
