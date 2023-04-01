use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref USERNAME_RE: Regex =
        Regex::new(r"(?i)^[a-z0-9_]+([a-z0-9_\.-]+[a-z0-9_]+)?$").unwrap();
}
