#[cfg(feature = "ssr")]
use axum::{http::StatusCode, response::IntoResponse};
use infra::common::ServiceError;
use leptos::server_fn::{
    codec::JsonEncoding,
    error::{FromServerFnError, ServerFnErrorErr::ServerError},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum AppError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Authentication required")]
    Authentication,
    #[error("Resource not found")]
    NotFound,
    #[error("Operation conflicts with current state")]
    Conflict,
    #[error("Too many requests")]
    RateLimited,
    #[error("Service temprarily unavailable")]
    Unavialble,
    #[error("Internal server error")]
    Internal,
    #[error("Server function error: {0}")]
    ServerFn(String),
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(error: leptos::prelude::ServerFnErrorErr) -> Self {
        Self::ServerFn(error.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<ServiceError> for AppError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::ValidationError(err) => AppError::Validation(
                err.message
                    .as_ref()
                    .map(|msg| msg.to_string())
                    .unwrap_or_else(|| err.code.into_owned()),
            ),
            ServiceError::Authentication => AppError::Authentication,
            ServiceError::UnexpectedError(err) => AppError::Internal,
            ServiceError::Domain(err) => match err {
                domain::Error::Invalid(msg) => AppError::Validation(msg.to_owned()),
                domain::Error::NotFound => AppError::NotFound,
                domain::Error::Conflict => AppError::Conflict,
                // domain::Error::InsufficientStock => StatusCode::CONFLICT,
                domain::Error::RateLimited => AppError::RateLimited,
                // domain::Error::ProviderUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                domain::Error::Storage => AppError::Unavialble,
            },
        }
    }
}

pub struct ApiError(pub ServiceError);

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        Self(error)
    }
}

#[cfg(feature = "ssr")]
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            ServiceError::ValidationError(err) => match err.code.as_ref() {
                "UNPROCESSABLE_ENTITY" => StatusCode::UNPROCESSABLE_ENTITY,
                "CONFLICT" => StatusCode::CONFLICT,
                "Unauthorized" => StatusCode::UNAUTHORIZED,
                _ => StatusCode::BAD_REQUEST,
            },
            ServiceError::Domain(err) => match err {
                domain::Error::Invalid(_) => StatusCode::BAD_REQUEST,
                domain::Error::NotFound => StatusCode::NOT_FOUND,
                domain::Error::Conflict => StatusCode::CONFLICT,
                // domain::Error::InsufficientStock => StatusCode::CONFLICT,
                domain::Error::RateLimited => StatusCode::TOO_MANY_REQUESTS,
                // domain::Error::ProviderUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                domain::Error::Storage => StatusCode::SERVICE_UNAVAILABLE,
            },
            ServiceError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::Authentication => StatusCode::UNAUTHORIZED,
            // _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.0.to_string()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use validator::ValidationError;

    use super::*;

    #[test]
    fn validation_error_display() {
        let err = ValidationError::new("TEST_CODE").with_message("Test error message".into());
        let qb_err = ServiceError::ValidationError(err);
        let display = format!("{}", qb_err);
        assert!(display.contains("Test error message"));
    }

    #[test]
    fn authentication_error_display() {
        let err = AppError::Authentication;
        let display = format!("{}", err);
        assert!(display.contains("Authentication required"));
    }

    #[test]
    fn unexpected_error_display() {
        let anyhow_err = anyhow::anyhow!("Something went wrong");
        let qb_err = ServiceError::UnexpectedError(anyhow_err);
        let display = format!("{}", qb_err);
        assert!(display.contains("Something went wrong"));
    }
}
