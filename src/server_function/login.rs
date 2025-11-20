// use axum_extra::extract::CookieJar;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use std::sync::Arc;

use crate::{
    domain::{login::Login as DomainLogin, user_email::UserEmail, user_password::UserPassword},
    error::QuestionBankError,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LoginCredential {
    pub id: String,
    pub password: String,
}

impl TryFrom<LoginCredential> for DomainLogin {
    type Error = QuestionBankError;

    fn try_from(value: LoginCredential) -> Result<Self, Self::Error> {
        use crate::domain::user_username::Username as Name;
        let password = UserPassword::parse(value.password)?;
        match UserEmail::parse(value.id.clone()) {
            Ok(email) => Ok(DomainLogin {
                username: None,
                email: Some(email),
                password,
            }),
            Err(_) => match Name::parse(value.id) {
                Ok(username) => Ok(DomainLogin {
                    username: Some(username),
                    email: None,
                    password,
                }),
                Err(err) => Err(QuestionBankError::ValidationError(err)),
            },
        }
    }
}

#[server]
pub async fn login(login: LoginCredential) -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();
    let login: DomainLogin = login.try_into()?;
    leptos::logging::log!("Login attempt: {:?}", login);
    #[cfg(feature = "ssr")]
    {
        use axum::http::{header::SET_COOKIE, HeaderValue};
        // use axum_extra::extract::cookie::Cookie;
        use leptos_axum::redirect;

        use sqlx::PgPool;
        use std::sync::Arc;
        // use time::Duration;

        let pool = expect_context::<Arc<PgPool>>();
        let user_id = user_logged_in(login, pool.clone()).await?;
        let session = store_session(user_id, pool.clone()).await?;

        let max_age_seconds = 7 * 24 * 3600;
        // let cookie = Cookie::build(("session", session))
        //     .http_only(true)
        //     .secure(true)
        //     .same_site(axum_extra::extract::cookie::SameSite::Lax)
        //     .path("/")
        //     .max_age(Duration::seconds(max_age_seconds));

        // let jar = jar.add(cookie);

        // provide_context(jar);

        let cookie_value = HeaderValue::from_str(&format!(
            "session={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
            session, max_age_seconds
        ))?;

        response.insert_header(SET_COOKIE, cookie_value);
        redirect("/home");
    }
    Ok(())
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    // username: String,
    pub password_hash: String,
}

#[cfg(feature = "ssr")]
pub async fn user_logged_in(
    user_login: DomainLogin,
    pool: Arc<sqlx::PgPool>,
) -> Result<i32, QuestionBankError> {
    use anyhow::Context;
    use pwhash::bcrypt;
    use validator::ValidationError;

    let user: User = match user_login {
        DomainLogin {
            username: None,
            email: Some(email),
            password: _,
        } => sqlx::query_as!(
            User,
            r#"SELECT id, password_hash FROM users WHERE email = $1"#,
            email.as_ref()
        )
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            use leptos::logging::log;

            log!("Failed to fetch user: {}", err);
            err
        })
        .context("Invalid email or password")?,
        DomainLogin {
            username: Some(username),
            email: None,
            password: _,
        } => sqlx::query_as!(
            User,
            r#"SELECT id, password_hash FROM users WHERE username = $1"#,
            username.as_ref()
        )
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            use leptos::logging::log;

            log!("Failed to fetch user: {}", err);
            err
        })
        .context("invalid user or password")?,
        _ => Err(QuestionBankError::ValidationError(
            ValidationError::new("UNPROCESSABLE_ENTITY")
                .with_message("email and username both are missing".into()),
        ))?,
    };

    if bcrypt::verify(user_login.password.as_ref(), &user.password_hash) {
        Ok(user.id)
    } else {
        use anyhow::anyhow;

        Err(anyhow!("Invalid user or password, please try again").into())
    }
}

#[cfg(feature = "ssr")]
pub async fn store_session(
    user_id: i32,
    pool: Arc<sqlx::PgPool>,
) -> Result<String, QuestionBankError> {
    use anyhow::Context;

    let session_id = uuid::Uuid::new_v4();
    let date = chrono::Local::now() + chrono::Duration::days(7);

    sqlx::query!(
        r#"DELETE FROM sessions where expires_at < $1"#,
        &chrono::Local::now().naive_local()
    )
    .execute(&*pool)
    .await
    .map_err(|err| {
        use leptos::logging::log;

        log!("Failed to Delete expired session: {}", err);
        err
    })
    .context("Failed to Delete expired session")?;

    sqlx::query!(
        r#"INSERT INTO sessions (user_id, session_token, expires_at) VALUES ($1, $2, $3)"#,
        &user_id,
        &session_id,
        &date.naive_local()
    )
    .execute(&*pool)
    .await
    .map_err(|err| {
        use leptos::logging::log;

        log!("Failed to store session: {}", err);
        err
    })
    .context("Failed to store session")?;

    Ok(session_id.to_string())
}
