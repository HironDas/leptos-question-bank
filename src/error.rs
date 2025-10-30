#[cfg(feature = "ssr")]
use axum::{http::StatusCode, response::IntoResponse};
use leptos::{
    prelude::{FromServerFnError, ServerFnError},
    server_fn::codec::JsonEncoding,
};
use std::fmt::Debug;
use thiserror::Error;
use validator::ValidationError;

#[derive(Error, serde::Serialize, serde::Deserialize)]
pub enum QuestionBankError {
    #[error(transparent)]
    ValidationError(ValidationError),
    #[error(transparent)]
    DbError(#[from] sqlx::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
                _ => StatusCode::BAD_REQUEST,
            },
            QuestionBankError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QuestionBankError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

impl FromServerFnError for QuestionBankError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: leptos::prelude::ServerFnErrorErr) -> Self {
        todo!()
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
