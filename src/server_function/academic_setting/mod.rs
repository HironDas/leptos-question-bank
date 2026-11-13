mod dto;
pub use dto::{Chapter, Class, Subject};
use leptos::prelude::*;

#[server]
pub async fn academic_setting() -> Result<Vec<Class>, ServerFnError> {
    let mut classes = Vec::new();
    #[cfg(feature = "ssr")]
    {
        use std::sync::Arc;

        use axum_extra::extract::CookieJar;
        use infra::common::ServiceContext;
        use leptos::logging::log;
        use sqlx::PgPool;

        use crate::util::auth::auth;
        let jar = expect_context::<CookieJar>();
        let ctx = expect_context::<ServiceContext>();

        if let Err(_) = auth(ctx.clone(), jar).await {
            use leptos_axum::redirect;

            log!("Unauthorized");
            redirect("/unauthorized");
        } else {
            use crate::server_function::academic_helper::class::class;

            classes = class(ctx.clone()).await?;
        }
    }
    Ok(classes)
}
