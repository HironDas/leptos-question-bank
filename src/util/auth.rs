#[cfg(feature = "ssr")]
use axum_extra::extract::CookieJar;
#[cfg(feature = "ssr")]
use infra::{auth::AuthService, common::ServiceContext};

#[cfg(feature = "ssr")]
use crate::error::AppError;

#[cfg(feature = "ssr")]
pub async fn auth(ctx: ServiceContext, jar: CookieJar) -> Result<(), AppError> {
    // use leptos::logging;

    let session = jar.get("session").map(|cookie| cookie.value());
    let auth_service = AuthService::from(ctx);
    auth_service.auth(session).await?;
    Ok(())
}
// logging::log!("Session in Auth rs: {:#?}", session);

// let Some(token) = session else {
//     return Err(AppError::Authentication);
// };
// let token = Uuid::parse_str(token).map_err(|err| {
//     logging::log!("UUID Error: {:?}", err);
//     AppError::Authentication
// })?;
