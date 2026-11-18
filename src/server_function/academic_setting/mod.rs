mod dto;
use domain::class::{AddClassInput, UpdateClassInput};
pub use dto::{Chapter, Class, Subject};
use leptos::prelude::*;

use crate::error::AppError;

#[server]
pub async fn academic_setting() -> Result<Vec<Class>, AppError> {
    let mut classes = Vec::new();
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        use infra::{common::ServiceContext, settings::SettingService};
        use leptos::logging::log;

        use crate::util::auth::auth;
        let jar = expect_context::<CookieJar>();
        let ctx = expect_context::<ServiceContext>();
        let setting_service = SettingService::from(ctx.clone());
        if let Err(_) = auth(ctx, jar).await {
            use leptos_axum::redirect;
            log!("Unauthorized");
            redirect("/unauthorized");
        } else {
            classes = setting_service.classes().await?;
        }
    }
    Ok(classes)
}

#[server]
pub async fn add_class(input: AddClassInput) -> Result<Class, AppError> {
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        use infra::{common::ServiceContext, settings::SettingService};

        use crate::util::auth::auth;

        let jar = expect_context::<CookieJar>();
        let ctx = expect_context::<ServiceContext>();

        let setting_service = SettingService::from(ctx.clone());

        auth(ctx, jar).await?;

        let class = setting_service.add_class(input).await?;
        Ok(class)
    }
}

#[server]
pub async fn edit_class(input: UpdateClassInput) -> Result<Class, AppError> {
    #[cfg(feature = "ssr")]
    {
        use axum_extra::extract::CookieJar;
        use infra::{common::ServiceContext, settings::SettingService};

        use crate::util::auth::auth;

        let jar = expect_context::<CookieJar>();
        let ctx = expect_context::<ServiceContext>();

        let setting_service = SettingService::from(ctx.clone());

        auth(ctx, jar).await?;

        let class = setting_service.edit_class(input).await?;
        Ok(class)
    }
}
