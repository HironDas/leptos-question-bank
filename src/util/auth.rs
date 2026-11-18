#[cfg(feature = "ssr")]
use axum_extra::extract::CookieJar;
#[cfg(feature = "ssr")]
use infra::{auth::AuthService, common::ServiceContext};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos::server;

#[cfg(feature = "ssr")]
use crate::error::AppError;

// #[cfg(feature = "ssr")]
// pub async fn auth(ctx: ServiceContext, jar: CookieJar) -> Result<Option<bool>, AppError> {
//     // use leptos::logging;

//     let session = jar.get("session").map(|cookie| cookie.value());
//     let auth_service = AuthService::from(ctx);
//     let allowed = auth_service.auth(session).await?.is_ok();
//     Ok(Some(allowed))
// }
//

#[server]
pub async fn is_auth() -> Result<Option<bool>, AppError> {
    // use leptos::logging;
    let ctx = expect_context::<ServiceContext>();
    let jar = expect_context::<CookieJar>();
    let session = jar.get("session").map(|cookie| cookie.value());
    let auth_service = AuthService::from(ctx);
    let allowed = auth_service.auth(session).await.is_ok();
    Ok(Some(allowed))
}
