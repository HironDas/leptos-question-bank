use leptos::prelude::*;

#[server]
pub async fn home() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        use leptos::logging::log;
        let jar = expect_context::<CookieJar>();
        let session = jar.get("session").map(|cookie| cookie.value().to_string());
        log!("Session: {:?}", session);
    }
    Ok(())
}
