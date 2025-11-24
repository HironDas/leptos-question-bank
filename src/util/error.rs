#[cfg(feature = "ssr")]
use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Debug;
use thiserror::Error;
use validator::ValidationError;

#[derive(Error)]
pub enum QuestionBankError {
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Authentication Error!!")]
    Authentication,
}

impl Debug for QuestionBankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[cfg(feature = "ssr")]
impl IntoResponse for QuestionBankError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            QuestionBankError::ValidationError(err) => match err.code.as_ref() {
                "UNPROCESSABLE_ENTITY" => StatusCode::UNPROCESSABLE_ENTITY,
                "CONFLICT" => StatusCode::CONFLICT,
                "Unauthorized" => StatusCode::UNAUTHORIZED,
                _ => StatusCode::BAD_REQUEST,
            },
            QuestionBankError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QuestionBankError::Authentication => StatusCode::UNAUTHORIZED,
        };
        (status, self.to_string()).into_response()
    }
}

fn error_chain_fmt(e: &dyn std::error::Error, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", e)?;
    if let Some(source) = e.source() {
        write!(f, "\nCaused by: ")?;
        error_chain_fmt(source, f)
    } else {
        Ok(())
    }
}
