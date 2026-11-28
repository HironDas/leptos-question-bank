use crate::error::AppError;
use leptos::prelude::*;
use leptos::server;

#[server]
pub async fn signout() -> Result<(), AppError> {
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        use infra::{common::ServiceContext, user::UserService};
        use leptos_axum::ResponseOptions;

        let ctx = expect_context::<ServiceContext>();
        let jar = expect_context::<CookieJar>();
        let response = expect_context::<ResponseOptions>();

        let user_service = UserService::from(ctx);
        let session = jar.get("session").map(|cookie| cookie.value());

        if let Some(session_token) = session {
            use axum::http::HeaderValue;

            user_service.logout(session_token).await?;
            let cookie_value = HeaderValue::from_str(&format!(
                // "session={}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={}",
                "session=''; Path=/; HttpOnly; SameSite=Lax; Max-Age=0",
            ))
            .map_err(|error| {
                use leptos::logging::log;

                log!("Invalid Header: {}", error.to_string());
                AppError::Internal
            })?;

            response.insert_header(axum::http::header::SET_COOKIE, cookie_value);
        };
    }
    Ok(())
}
