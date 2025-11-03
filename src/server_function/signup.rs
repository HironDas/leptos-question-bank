use crate::{
    domain::{
        new_user::NewUser, user_email::UserEmail, user_password::UserPassword,
        user_username::Username,
    },
    error::QuestionBankError,
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
    type Error = QuestionBankError;
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
                    Err(error.clone().into())
                } else {
                    Err(ValidationError::new("Unknown error").into())
                }
            }
        }
    }
}

#[server]
pub async fn signup_action(user: User) -> Result<(), ServerFnError> {
    leptos::logging::log!("Signup is clicked --");
    let new_user: NewUser = user.try_into()?;

    #[cfg(feature = "ssr")]
    insert_new_user(new_user).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn insert_new_user(user: NewUser) -> Result<(), QuestionBankError> {
    use anyhow::Context;
    use sqlx::PgPool;
    use std::sync::Arc;

    let pool = expect_context::<Arc<PgPool>>();
    let password_hash = user.hash_password();

    sqlx::query(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user.username.as_ref())
    .bind(user.email.as_ref())
    .bind(&password_hash)
    .execute(&*pool)
    .await
    .map(|result| {
        use leptos::logging::log;
        log!("User Inserted Successfully! and the id is {:?}", result);
        Ok(())
    })
    .map_err(|err| {
        use leptos::logging::log;

        log!("Sqlx Error: {:?}", err);
        err
        //QuestionBankError::UnexpectedError(err)
    })
    .context("User Insert Failed! Please fill all fields correctly")?
}

#[server]
pub async fn is_user_taken(username: String) -> Result<bool, ServerFnError> {
    let exists;
    use leptos::logging::log;
    #[cfg(feature = "ssr")]
    {
        use std::sync::Arc;

        use sqlx::PgPool;

        exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)
            "#,
            username
        )
        .fetch_one(&*expect_context::<Arc<PgPool>>())
        .await
        .map_err(|err| {
            use leptos::logging::log;

            log!("Sqlx Error: {:?}", err);
            err
            //QuestionBankError::UnexpectedError(err)
        })?
        .exists;
        //.context("User Insert Failed! Please fill all fields correctly")?
        //.exists;
    }
    log!("User Exists: {:?}", exists);

    Ok(exists.unwrap())
}

#[server]
pub async fn is_email_exists(email: String) -> Result<bool, ServerFnError> {
    let exists;
    use leptos::logging::log;
    #[cfg(feature = "ssr")]
    {
        use std::sync::Arc;

        use sqlx::PgPool;

        exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)
            "#,
            email
        )
        .fetch_one(&*expect_context::<Arc<PgPool>>())
        .await
        .map_err(|err| {
            use leptos::logging::log;

            log!("Sqlx Error: {:?}", err);
            err
            //QuestionBankError::UnexpectedError(err)
        })?
        .exists;
        //.context("User Insert Failed! Please fill all fields correctly")?
        //.exists;
    }
    log!("Email Exists: {:?}", exists);

    Ok(exists.unwrap())
}
