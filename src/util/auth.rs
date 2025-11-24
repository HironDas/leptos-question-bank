use std::sync::Arc;

use axum_extra::extract::CookieJar;
#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
use crate::error::QuestionBankError;

#[cfg(feature = "ssr")]
pub async fn auth(pool: Arc<PgPool>, jar: CookieJar) -> Result<(), QuestionBankError> {
    use anyhow::Context;
    use leptos::logging;
    use uuid::Uuid;
    let session = jar.get("session").map(|cookie| cookie.value());
    logging::log!("Session in middleware: {:#?}", session);

    if let Some(token) = session {
        let token = Uuid::parse_str(token)
            .map_err(|err| {
                logging::log!("UUID Error: {:?}", err);
                err
            })
            .context("UUID Parseing Error")?;

        let is_authencated = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM sessions WHERE session_token = $1)
            "#,
            &token
        )
        .fetch_one(&*pool)
        .await
        .map_err(|err| {
            use leptos::logging::log;

            log!("Sqlx Error: {:?}", err);
            err
        })
        .context("Something went wrong to fetch session")?
        .exists;
        logging::log!("I AM IN MIDDLEWARE");
        if let Some(true) = is_authencated {
            Ok(())
        } else {
            Err(QuestionBankError::Authentication)
        }
    } else {
        Err(QuestionBankError::Authentication)
    }
}
