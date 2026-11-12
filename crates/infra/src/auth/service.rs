use sqlx::types::Uuid;

use crate::{
    auth::repository::is_auth,
    common::{db, ServiceContext, ServiceError},
};

#[derive(Clone)]
pub struct AuthService {
    pub(crate) context: ServiceContext,
}

impl From<ServiceContext> for AuthService {
    fn from(ctx: ServiceContext) -> Self {
        Self { context: ctx }
    }
}

impl AuthService {
    pub async fn auth(&self, session: Option<Uuid>) -> Result<(), ServiceError> {
        let Some(token) = session else {
            return Err(ServiceError::Authentication);
        };
        let mut tx = self.context.pool.begin().await.map_err(db)?;
        is_auth(&mut tx, token).await?;
        Ok(())
    }
}
