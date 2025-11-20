use leptos::prelude::*;

#[server]
pub async fn home() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        let jar = expect_context::<CookieJar>();
        let session = jar.get("session").map(|cookie| cookie.value().to_string());
        println!("Session: {:?}", session);
    }
    Ok(())
}
