use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Debug;
use thiserror::Error;
use validator::ValidationError;

#[derive(Error)]
pub enum ServiceError {
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Authentication Error!!")]
    Authentication,
}

impl Debug for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
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

#[cfg(test)]
mod tests {
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
        let err = ServiceError::Authentication;
        let display = format!("{}", err);
        assert!(display.contains("Authentication Error"));
    }

    #[test]
    fn unexpected_error_display() {
        let anyhow_err = anyhow::anyhow!("Something went wrong");
        let qb_err = ServiceError::UnexpectedError(anyhow_err);
        let display = format!("{}", qb_err);
        assert!(display.contains("Something went wrong"));
    }
}
