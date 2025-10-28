use crate::domain::{
    new_user::NewUser, user_email::UserEmail, user_password::UserPassword, user_username::Username,
};

use leptos::prelude::*;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

impl TryFrom<User> for NewUser {
    type Error = ValidationError;
    fn try_from(value: User) -> Result<Self, Self::Error> {
        let username = Username::parse(value.username).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;
        let email = UserEmail::parse(value.email).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;
        let password = UserPassword::parse(value.password).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;
        let confirm_password = UserPassword::parse(value.confirm_password).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;

        let new_user = NewUser {
            username,
            email,
            password,
            confirm_password,
        };

        match new_user.validate() {
            Ok(_) => Ok(new_user),
            Err(errs) => {
                let mut first_error = None;
                errs.field_errors().into_iter().for_each(|(field, errors)| {
                    errors.into_iter().for_each(|error| {
                        println!("{}: {}", field, error.message.as_ref().unwrap());
                    });
                    first_error = errors.into_iter().next();
                });

                if let Some(error) = first_error {
                    Err(error.clone())
                } else {
                    Err(ValidationError::new("Unknown error"))
                }
            }
        }
    }
}

#[server]
pub async fn signup_action(user: User) -> Result<(), ServerFnError> {
    leptos::logging::log!("Signup is clicked --");
    let new_user: NewUser = user.try_into()?;

    let password_hash = new_user.hash_password();

    #[cfg(feature = "ssr")]
    {
        use sqlx::PgPool;
        use std::sync::Arc;

        let pool = expect_context::<Arc<PgPool>>();

        sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(new_user.username.as_ref())
        .bind(new_user.email.as_ref())
        .bind(&password_hash)
        .execute(&*pool)
        .await?;
    }
    Ok(())
}
