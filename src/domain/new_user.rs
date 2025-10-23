use crate::domain::user_email::UserEmail;
use leptos::prelude::*;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct newUser {
    #[validate(custom(function = "validate_unique_email"))]
    email: UserEmail,
}

fn validate_unique_email(email: &UserEmail) -> Result<(), ValidationError> {
    if email.as_ref().contains('@') {
        Ok(())
    } else {
        Err(ValidationError::new("Already exists"))
    }
}

#[server]
async fn check_email(email: String) -> Result<(), ServerFnError> {
    todo!()
}
