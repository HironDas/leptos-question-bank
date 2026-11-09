use domain::{Error, Result};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ServiceContext {
    pub(crate) pool: PgPool,
}

impl ServiceContext {
    pub fn new(pool: PgPool) -> Result<Self> {
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

pub(crate) fn db(error: sqlx::Error) -> Error {
    if matches!(error, sqlx::Error::RowNotFound) {
        return Error::NotFound;
    }
    match error.as_database_error().and_then(|e| e.code()).as_deref() {
        Some("23505" | "40001" | "40P01" | "55P03") => Error::Conflict,
        Some("23503" | "23514" | "22003") => Error::Invalid("Invalid relationship or constraint"),
        _ => Error::Storage,
    }
}
