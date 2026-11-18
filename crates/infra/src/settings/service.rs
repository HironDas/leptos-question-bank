use anyhow::anyhow;
use domain::class::{
    AddClassInput, Chapter, ChapterInput, Class, Subject, SubjectInput, UpdateClassInput,
};
use validator::Validate;

use crate::{
    common::{ServiceContext, ServiceError},
    settings::repository::{self, subject},
};

#[derive(Clone)]
pub struct SettingService {
    context: ServiceContext,
}

impl From<ServiceContext> for SettingService {
    fn from(context: ServiceContext) -> Self {
        Self { context }
    }
}

impl SettingService {
    pub async fn classes(&self) -> Result<Vec<Class>, ServiceError> {
        let pool = &self.context.pool;
        let classes: Vec<Class> = repository::class::list_class(pool).await?;
        Ok(classes)
    }

    pub async fn add_class(&self, input: AddClassInput) -> Result<Class, ServiceError> {
        let pool = &self.context.pool;
        input.validate()?;

        let class = repository::class::insert_class(pool, &input.name, &input.name_bn).await?;
        Ok(class)
    }

    pub async fn edit_class(&self, input: UpdateClassInput) -> Result<Class, ServiceError> {
        let pool = &self.context.pool;
        input.validate()?;

        let class = repository::class::update_class(
            pool,
            input.class_id as i32,
            &input.name,
            &input.name_bn,
        )
        .await?;
        Ok(class)
    }

    pub async fn subjects(&self, class_id: u32) -> Result<Vec<Subject>, ServiceError> {
        let input = i32::try_from(class_id).map_err(|_| anyhow!("Class id parsing Failed!"))?;
        let pool = &self.context.pool;
        let subject = repository::subject::list_subjects(pool, input).await?;
        Ok(subject)
    }

    pub async fn add_subject(&self, input: SubjectInput) -> Result<Subject, ServiceError> {
        let pool = &self.context.pool;
        input.validate()?;
        let subject =
            repository::subject::insert_subject(pool, &input.title, input.id as i32).await?;

        Ok(subject)
    }

    pub async fn edit_subject(&self, input: SubjectInput) -> Result<Subject, ServiceError> {
        let pool = &self.context.pool;
        input.validate()?;
        let subject =
            repository::subject::update_subject(pool, &input.title, input.id as i32).await?;

        Ok(subject)
    }

    pub async fn chapters(&self, id: u32) -> Result<Vec<Chapter>, ServiceError> {
        let pool = &self.context.pool;
        let subject_id = i32::try_from(id).map_err(|_| anyhow!("Subject id parsing failed!"))?;

        let chapters = repository::chapter::list_chapters(pool, subject_id).await?;
        Ok(chapters)
    }

    pub async fn add_chapter(&self, input: ChapterInput) -> Result<Chapter, ServiceError> {
        let pool = &self.context.pool;
        input.validate()?;
        let chapter =
            repository::chapter::insert_chapter(pool, &input.title, input.id as i32).await?;
        Ok(chapter)
    }

    pub async fn edit_chapter(&self, input: ChapterInput) -> Result<Chapter, ServiceError> {
        let pool = &self.context.pool;
        input.validate()?;
        let chapter =
            repository::chapter::update_chapter(pool, &input.title, input.id as i32).await?;
        Ok(chapter)
    }
}
