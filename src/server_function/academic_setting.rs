#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub id: u32,
    pub name: String,
    pub name_bn: String,
    pub order: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    pub id: u32,
    pub title: String,
    pub class_id: u32,
    pub order: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Chapter {
    pub id: u32,
    pub title: String,
    pub order: u32,
    pub class_id: u32,
    pub subject_id: u32,
    pub question_count: u32,
}

use leptos::prelude::*;

#[server]
pub async fn academic_setting() -> Result<Vec<Class>, ServerFnError> {
    let mut classes = Vec::new();
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
        } else {
            use crate::server_function::academic_helper::class::class;

            classes = class(pool.clone()).await?;
        }
    }
    Ok(classes)
}
