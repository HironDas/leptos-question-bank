use leptos::prelude::*;

#[server]
pub async fn home() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        use infra::common::ServiceContext;
        use leptos::logging::log;

        use crate::util::auth::auth;
        let jar = expect_context::<CookieJar>();
        let ctx = expect_context::<ServiceContext>();

        if let Err(_) = auth(ctx, jar).await {
            use leptos_axum::redirect;

            log!("Unauthorized");
            redirect("/unauthorized");
        }
    }
    Ok(())
}
