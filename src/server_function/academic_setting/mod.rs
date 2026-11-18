mod dto;
use domain::class::{AddClassInput, ChapterInput, SubjectInput, UpdateClassInput};
pub use dto::{Chapter, Class, Subject};
use infra::common::ServiceContext;
use leptos::prelude::*;

use crate::error::AppError;

#[server]
pub async fn academic_setting() -> Result<Vec<Class>, AppError> {
    #[cfg(feature = "ssr")]
    {
        // let mut classes = Vec::new();
        use infra::{common::ServiceContext, settings::SettingService};

        let ctx = expect_context::<ServiceContext>();
        let setting_service = SettingService::from(ctx);

        let classes = setting_service.classes().await?;
        Ok(classes)
    }
}

#[server]
pub async fn add_class(input: AddClassInput) -> Result<Class, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::{common::ServiceContext, settings::SettingService};

        let ctx = expect_context::<ServiceContext>();

        let setting_service = SettingService::from(ctx.clone());

        let class = setting_service.add_class(input).await?;
        Ok(class)
    }
}

#[server]
pub async fn edit_class(input: UpdateClassInput) -> Result<Class, AppError> {
    #[cfg(feature = "ssr")]
    {
        // use axum_extra::extract::CookieJar;
        use infra::{common::ServiceContext, settings::SettingService};

        let ctx = expect_context::<ServiceContext>();

        let setting_service = SettingService::from(ctx.clone());

        // auth(ctx, jar).await?;

        let class = setting_service.edit_class(input).await?;
        Ok(class)
    }
}

#[server]
pub async fn subjects(id: u32) -> Result<Vec<Subject>, AppError> {
    #[cfg(feature = "ssr")]
    {
        // use axum_extra::extract::CookieJar;
        use infra::{common::ServiceContext, settings::SettingService};

        //use crate::util::auth::auth;

        let ctx = expect_context::<ServiceContext>();
        // let jar = expect_context::<CookieJar>();
        let setting_service = SettingService::from(ctx.clone());
        // auth(ctx, jar).await?;

        let subjects = setting_service.subjects(id).await?;
        Ok(subjects)
    }
}

#[server]
pub async fn add_subject(input: SubjectInput) -> Result<Subject, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::settings::SettingService;

        let ctx: ServiceContext = expect_context();
        let setting_service: SettingService = ctx.into();

        let subject = setting_service.add_subject(input).await?;
        Ok(subject)
    }
}

#[server]
pub async fn edit_subject(input: SubjectInput) -> Result<Subject, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::settings::SettingService;

        let ctx: ServiceContext = expect_context();
        let setting_service: SettingService = ctx.into();

        let subject = setting_service.edit_subject(input).await?;
        Ok(subject)
    }
}

#[server]
pub async fn chapters(id: u32) -> Result<Vec<Chapter>, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::settings::SettingService;

        let ctx: ServiceContext = expect_context();
        let setting_service: SettingService = ctx.into();

        let chapters = setting_service.chapters(id).await?;
        Ok(chapters)
    }
}

#[server]
pub async fn add_chapter(input: ChapterInput) -> Result<Chapter, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::settings::SettingService;

        let ctx: ServiceContext = expect_context();
        let setting_service: SettingService = ctx.into();

        let chapter = setting_service.add_chapter(input).await?;
        Ok(chapter)
    }
}

#[server]
pub async fn edit_chapter(input: ChapterInput) -> Result<Chapter, AppError> {
    #[cfg(feature = "ssr")]
    {
        use infra::settings::SettingService;

        let ctx: ServiceContext = expect_context();
        let setting_service: SettingService = ctx.into();

        let chapter = setting_service.edit_chapter(input).await?;
        Ok(chapter)
    }
}
