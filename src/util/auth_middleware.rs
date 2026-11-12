#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use axum::extract::State;
#[cfg(feature = "ssr")]
use axum::{body::Body, http::Request, middleware::Next, response::Response};
#[cfg(feature = "ssr")]
use axum_extra::extract::CookieJar;
#[cfg(feature = "ssr")]
use infra::common::ServiceError;
#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
use crate::error::ApiError;

#[cfg(feature = "ssr")]
pub async fn auth_middleware(
    State(pool): State<Arc<PgPool>>,
    jar: CookieJar,
    req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    use leptos::logging;
    use uuid::Uuid;
    let session = jar.get("session").map(|cookie| cookie.value());
    logging::log!("Session in middleware: {:#?}", session);

    if let Some(token) = session {
        let token = Uuid::parse_str(token).map_err(|err| {
            logging::log!("UUID Error: {:?}", err);
            ApiError::from(ServiceError::Authentication)
        })?;

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
            // err
            ApiError::from(ServiceError::UnexpectedError(anyhow::anyhow!(err)))
        })?
        .exists;

        logging::log!("I AM IN MIDDLEWARE");
        if let Some(true) = is_authencated {
            Ok(next.run(req).await)
        } else {
            Err(ServiceError::Authentication.into())
        }
    } else {
        Err(ApiError::from(ServiceError::Authentication))
    }
}
