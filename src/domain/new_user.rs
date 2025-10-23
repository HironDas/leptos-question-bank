use crate::domain::{user_email::UserEmail, user_password::UserPassword};
use leptos::prelude::*;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct NewUser {
    #[validate(custom(function = "validate_unique_email"))]
    pub email: UserEmail,
    pub password: UserPassword,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: UserPassword,
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
