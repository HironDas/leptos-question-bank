use std::sync::Arc;

use domain::Result;
use sqlx::PgPool;
#[derive(Clone)]
pub struct ServiceContext {
    pub(crate) pool: PgPool,
    pub(crate) rate_limit_secret: Arc<[u8; 32]>,
}

impl ServiceContext {
    pub fn new(pool: PgPool, secret: [u8; 32]) -> Result<Self> {
        Ok(Self {
            pool,
            rate_limit_secret: Arc::new(secret),
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
