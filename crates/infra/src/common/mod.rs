use domain::Error;

mod context;
mod sevice_error;

pub use context::ServiceContext;
pub use sevice_error::ServiceError;

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
