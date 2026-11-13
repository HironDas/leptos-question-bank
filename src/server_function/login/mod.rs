use infra::common::ServiceError;

#[cfg(feature = "ssr")]
use leptos::logging::log;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

use crate::{error::AppError, server_function::login::dto::LoginCredential};

mod dto;

#[server]
pub async fn login(login: LoginCredential) -> Result<(), AppError> {
    let response = expect_context::<ResponseOptions>();
    if login.id == "" {
        return Err(AppError::Validation(
            "Username/Email is empty. Please put a valid input.".to_string(),
        ));
    } else if login.password == "" {
        return Err(AppError::Validation(
            "Password is empty! Please input the password.".to_string(),
        ));
    }
    let login = login.try_into().map_err(|err: ServiceError| {
        log!("validataion Error: {:?}", err);
        AppError::Validation("Invalid username or password! please try again.".into())
    })?;
    // .context("Invalid username or password, please try again")?;

    leptos::logging::log!("Login attempt: {:?}", login);
    #[cfg(feature = "ssr")]
    {
        use axum::http::{header::SET_COOKIE, HeaderValue};
        use infra::{common::ServiceContext, user::UserService};
        use leptos_axum::redirect;

        let ctx = expect_context::<ServiceContext>();

        let max_age_seconds = 7 * 24 * 3600;

        let user_service = UserService::from(ctx);
        let session = user_service.login(login).await?;

        let cookie_value = HeaderValue::from_str(&format!(
            // "session={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
            "session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
            session, max_age_seconds
        ))
        .map_err(|error| {
            log!("Invalid Header: {}", error.to_string());
            AppError::Internal
        })?;

        response.insert_header(SET_COOKIE, cookie_value);
        redirect("/home");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::login::Login as DomainLogin;
    #[test]
    fn login_credential_with_email_parses_correctly() {
        let cred = LoginCredential {
            id: "test@example.com".to_string(),
            password: "Hiron@12345".to_string(),
        };
        let result: Result<DomainLogin, _> = cred.try_into();
        assert!(result.is_ok());
        let login = result.unwrap();
        assert!(login.email.is_some());
        assert!(login.username.is_none());
    }

    #[test]
    fn login_credential_with_username_parses_correctly() {
        let cred = LoginCredential {
            id: "testuser".to_string(),
            password: "Hiron@12345".to_string(),
        };
        let result: Result<DomainLogin, _> = cred.try_into();
        assert!(result.is_ok());
        let login = result.unwrap();
        assert!(login.username.is_some());
        assert!(login.email.is_none());
    }

    #[test]
    fn login_credential_with_invalid_id_fails() {
        let cred = LoginCredential {
            id: "ab".to_string(),
            password: "Hiron@12345".to_string(),
        };
        let result: Result<DomainLogin, _> = cred.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn login_credential_with_invalid_password_fails() {
        let cred = LoginCredential {
            id: "testuser".to_string(),
            password: "weak".to_string(),
        };
        let result: Result<DomainLogin, _> = cred.try_into();
        assert!(result.is_err());
    }
}
