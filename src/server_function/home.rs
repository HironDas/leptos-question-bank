use leptos::prelude::*;

#[server]
pub async fn home() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use std::sync::Arc;

        use axum_extra::extract::CookieJar;
        use leptos::logging::log;
        use sqlx::PgPool;

        use crate::util::auth::auth;
        let jar = expect_context::<CookieJar>();
        let pool = expect_context::<Arc<PgPool>>();

        if let Err(_) = auth(pool.clone(), jar).await {
            use leptos_axum::redirect;

            log!("Unauthorized");
            redirect("/unauthorized");
        }
    }
    Ok(())
}
