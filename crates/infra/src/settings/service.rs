use domain::class::{AddClassInput, Class};
use validator::Validate;

use crate::{
    common::{ServiceContext, ServiceError},
    repository,
    settings::{repository, service},
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

    pub async fn add_class(&self, class: AddClassInput) -> Result<Class, ServiceError> {
        let pool = &self.context.pool;
        class.validate()?;

        let class = repository::class::insert_class(pool, &class.name, &class.name_bn).await?;
        Ok(class)
    }
}
