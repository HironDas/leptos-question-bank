use domain::new_user::NewUser;

use infra::{common::ServiceContext, user::UserService};
use leptos::prelude::*;

mod dto;

#[server]
pub async fn signup_action(user: User) -> Result<(), AppError> {
    leptos::logging::log!("Signup is clicked --");
    let new_user: NewUser = user.try_into()?;

    let ctx = expect_context::<ServiceContext>();
    let user_service = UserService::from(ctx);

    #[cfg(feature = "ssr")]
    {
        use leptos_axum::redirect;
        user_service.signup(new_user).await?;
        redirect("/");
    }
    Ok(())
}

use crate::{error::AppError, server_function::signup::dto::User};

#[server]
pub async fn is_user_taken(username: String) -> Result<bool, ServerFnError> {
    let exists;
    let ctx = expect_context::<ServiceContext>();
    #[cfg(feature = "ssr")]
    {
        let user_service = UserService::from(ctx);

        exists = user_service.is_user_exist(username).await?;
    }

    Ok(exists)
}

#[server]
pub async fn is_email_exists(email: String) -> Result<bool, ServerFnError> {
    let exists;
    let ctx = expect_context::<ServiceContext>();
    #[cfg(feature = "ssr")]
    {
        let user_service = UserService::from(ctx);
        exists = user_service.is_email_exist(email).await?;
    }
    // log!("Email Exists: {:?}", exists);

    Ok(exists)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_try_into_new_user_succeeds_for_valid_input() {
        let user = User {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "Hiron@12345".to_string(),
            confirm_password: "Hiron@12345".to_string(),
        };
        let result: Result<NewUser, _> = user.try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn user_try_into_new_user_fails_for_invalid_email() {
        let user = User {
            username: "testuser".to_string(),
            email: "not-an-email".to_string(),
            password: "Hiron@12345".to_string(),
            confirm_password: "Hiron@12345".to_string(),
        };
        let result: Result<NewUser, _> = user.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn user_try_into_new_user_fails_for_invalid_password() {
        let user = User {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "weak".to_string(),
            confirm_password: "weak".to_string(),
        };
        let result: Result<NewUser, _> = user.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn user_try_into_new_user_fails_for_invalid_username() {
        let user = User {
            username: "ab".to_string(),
            email: "test@example.com".to_string(),
            password: "Hiron@12345".to_string(),
            confirm_password: "Hiron@12345".to_string(),
        };
        let result: Result<NewUser, _> = user.try_into();
        assert!(result.is_err());
    }
}
