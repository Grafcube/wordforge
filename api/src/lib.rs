use sqlx::PgPool;
use std::sync::Arc;

pub mod account;
pub mod activities;
pub mod api;
pub mod enums;
pub mod objects;
pub mod util;

pub type DbHandle = Arc<PgPool>;
