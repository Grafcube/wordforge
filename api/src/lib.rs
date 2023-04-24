use sqlx::PgPool;
use std::sync::Arc;

pub mod account;
pub mod util;

pub type DbHandle = Arc<PgPool>;
