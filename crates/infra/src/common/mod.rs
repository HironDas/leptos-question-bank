use domain::Result;
use sqlx::PgPool;

#[derive(Clone)]
pub struct ServiceContext {
    pub(crate) pool: PgPool,
}

impl ServiceContext {
    pub fn new(pool: PgPool) -> Result<Self> {
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
