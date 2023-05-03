use sqlx::PgPool;
use std::sync::Arc;

pub mod account;
pub mod api;
pub mod enums;
pub mod util;

pub type DbHandle = Arc<PgPool>;
